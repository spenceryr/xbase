mod simctl;

use tokio::sync::MutexGuard;

pub use self::simctl::*;

use crate::{
    client::Client,
    constants::DAEMON_STATE,
    nvim::BufferDirection,
    state::State,
    types::Platform,
    util::{fmt, pid},
    Error, Result,
};
use {
    process_stream::{Process, ProcessItem, StreamExt},
    tap::Pipe,
    tokio::task::JoinHandle,
    xcodebuild::parser::BuildSettings,
};

pub struct Runner {
    pub client: Client,
    pub target: String,
    pub platform: Platform,
    pub udid: Option<String>,
    pub direction: BufferDirection,
    pub args: Vec<String>,
}

impl Runner {
    pub async fn run<'a>(
        self,
        state: &'a MutexGuard<'_, State>,
        settings: BuildSettings,
    ) -> Result<JoinHandle<Result<()>>> {
        if self.platform.is_mac_os() {
            return self.run_as_macos_app(state, settings).await;
        } else {
            return self.run_with_simctl(state, settings).await;
        }
    }
}

/// MacOS Runner
impl Runner {
    pub async fn run_as_macos_app<'a>(
        self,
        state: &'a MutexGuard<'_, State>,
        settings: BuildSettings,
    ) -> Result<JoinHandle<Result<()>>> {
        let nvim = self.client.nvim(state)?;
        let ref mut logger = nvim.logger();

        logger.log_title().await?;
        logger.open_win().await?;

        tokio::spawn(async move {
            let program = settings.path_to_output_binary()?;
            let mut stream = Process::new(&program).stream()?;

            tracing::debug!("Running binary {program:?}");

            use ProcessItem::*;
            // NOTE: This is required so when neovim exist this should also exit
            while let Some(update) = stream.next().await {
                let state = DAEMON_STATE.clone();
                let state = state.lock().await;
                let nvim = state.clients.get(&self.client.pid)?;
                let mut logger = nvim.logger();

                // NOTE: NSLog get directed to error by default which is odd
                match update {
                    Output(msg) => logger.log(msg).await?,
                    Error(msg) => logger.log(format!("[Error] {msg}")).await?,
                    Exit(ref code) => {
                        let success = code == "0";
                        let msg = fmt::as_section(if success {
                            format!("[Exit] {code}")
                        } else {
                            format!("[Error] Exit {code}")
                        });
                        logger.log(msg).await?;
                        logger.set_status_end(success, true).await?;
                    }
                }
            }
            Ok(())
        })
        .pipe(Ok)
    }
}

/// Simctl Runner
impl Runner {
    pub async fn run_with_simctl<'a>(
        self,
        state: &'a MutexGuard<'_, State>,
        settings: BuildSettings,
    ) -> Result<JoinHandle<Result<()>>> {
        let Self {
            client,
            target,
            udid,
            ..
        } = self;

        let ref mut logger = state.clients.get(&client.pid)?.logger();
        let app_id = settings.product_bundle_identifier;
        let path_to_app = settings.metal_library_output_dir;

        logger.set_running().await?;

        let runner = {
            let device = if let Some(udid) = udid {
                state.devices.get(&udid).cloned()
            } else {
                None
            }
            .ok_or_else(|| Error::Run("udid not found!!".to_string()))?;

            let runner = SimDeviceRunner::new(device, target.clone(), app_id, path_to_app);
            runner.boot(logger).await?;
            runner.install(logger).await?;
            runner
        };

        let mut launcher = runner.launch(logger).await?;

        match pid::get_by_name("Simulator") {
            Err(Error::NotFound(_, _)) => {
                let msg = format!("[Simulator] Launching");
                tracing::info!("{msg}");
                logger.log(msg).await?;
                tokio::process::Command::new("open")
                    .args(&["-a", "Simulator"])
                    .spawn()?
                    .wait()
                    .await?;
                let msg = format!("[Simulator] Connected");
                logger.log(msg).await?;
            }
            Err(err) => {
                let msg = err.to_string();
                tracing::error!("{msg}");
                logger.log(msg).await?;
            }
            _ => {}
        };

        logger.log(fmt::separator()).await?;

        let mut stream = launcher.stream()?;

        tokio::spawn(async move {
            while let Some(output) = stream.next().await {
                let state = DAEMON_STATE.clone();
                let state = state.lock().await;
                let mut logger = match state.clients.get(&client.pid) {
                    Ok(nvim) => nvim.logger(),
                    Err(_) => {
                        tracing::info!("Nvim Instance closed, closing runner ..");
                        launcher.kill().await;
                        break;
                    }
                };

                use ProcessItem::*;
                match output {
                    Output(msg) => {
                        if !msg.contains("ignoring singular matrix") {
                            logger.log(msg).await?;
                        }
                    }
                    Error(msg) => {
                        logger.log(format!("[Error] {msg}")).await?;
                    }
                    // TODO: this should be skipped when user re-run the app
                    Exit(code) => {
                        logger.log(format!("[Exit] {code}")).await?;
                        break;
                    }
                };
                drop(state);
            }

            drop(stream);

            Ok(())
        })
        .pipe(Ok)
    }
}