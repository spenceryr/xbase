#[tokio::main]
async fn main() -> anyhow::Result<()> {
    xcodebase::install_tracing("/tmp", "xcodebase-daemon.log", tracing::Level::TRACE, true)?;
    xcodebase::Daemon::default().run().await
}
