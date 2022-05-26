<p align="center">
<img src="./media/logo.png" alt="drawing" style="width:400px;"/>
</P>

## 👁 Overview 

Xcode replacement-ish development environment for neovim. Supports building/watching/running
xcode products, in addition to logging and auto-completion. It requires and recommends using
[XcodeGen] as a way to configure and generate xcode projects without interacting with Xcode.

It is still work in progress, if you have something in mind or found a bug open an issue.

## 🌝 Motivation

After purchasing a MacBook, I decided to get into iOS/macOS applications development. Though, coming from vim/shell development experience and being heavily keyboard driven, I could not handle the switch to closed sourced, opinionated, mouse-driven development environment. I tried workarounds such as [XVim2] and builtin vim emulator but still I'd catch myself often looking for my mouse. 

Being a long time vim user and having previously develop few lua/nvim plugins, I decided to take sometime to invest in both creating a fairly complicated development environment for `xOS` and enrich my experience with [rust].

## 🌟 Features:

* **Auto-Completion and Code navigation**\
    [sourcekit-lsp] doesn't support auto-indexing. Therefore, [xbase] includes a custom build server that auto-generate compiled command on directory changes (i.e. file removed/added).
* **Multi-nvim instance support**\
    Thanks to having a single daemon running and simple client/server architecture, users can have and use multiple nvim instance running without process duplications and shared state. E.g. stop watch service ran from a nvim instance in another instance.
* **Auto-start/stop main background daemon**\
    No need to manual start/stop daemon background. The daemon will auto start/stop based on
    connect neovim (client instances)
* **Multi Target/Project Support**\
    Users can develop across different projects and build/run on root directory changes or once.
* **Simulator Support**\
    Pick a simulator devices relative to target platform and run the target once or per file changes. If the simulator isn't running then it will auto launch it.
* **Logging buffer**\
  real-time logging of print statements and build logs. This feature depends partially on [xcodebuild] crate which is still work-in-progress and might not print errors. Currently logs are specific to the requested client, if you find yourself needing shared logs, open an issue.
* **Statusline Support**\
    Global variable to update statusline with build/run commands + [feline] provider. Other
    statusline plugins support are welcomed.
* **No footprint**\
    light resource use. I've been using xbase for a while now, usually 0.0% cpu and 0.1% memory.



## Installation

packer.nvim:

```lua 
use { 
  'tami5/xbase', 
    run = 'make install',
    requires = {
      "nvim-lua/plenary.nvim",
      "nvim-telescope/telescope.nvim"
    }
}
```

`make install` would build all required binaries in addition to a lua library. The binaries will be moved to `path/to/repo/bin` and the lua library will be mvoed to `path/to/repo/lua/libxbase.so`.

## 🎮 Usage

Currently, once you start a neovim instance where it has root has `project.yml`, the daemon server will auto-start if no instance is running, then the root will be registered for recompile-watch.

From there, you have a telescope picker

```lua
vim.api.nvim_create_autocmd({ "BufEnter", "BufWinEnter" }, {
  pattern = { "*.m", "*.swift", "*.c", "*.yml" },
  callback = function()
    vim.keymap.set("n", "<leader>ef", require("xbase.pickers").actions, { buffer = true })
  end,
})
-- so that on a new buffer it would work
vim.keymap.set("n", "<leader>ef", require("xbase.pickers").actions, { buffer = true })
```

### Configuration (defaults)
```lua 
local defaults = {
  --- Log level. Set to error to ignore everything: { "trace", "debug", "info", "warn", "error" }
  log_level = "debug",
  --- Default log buffer direction: { "horizontal", "vertical", "float" }
  default_log_buffer_direction = "horizontal",
  --- Statusline provider configurations
  statusline = {
    running = { icon = "⚙", color = "#e0af68" },
    device_running = { icon = "", color = "#4a6edb" },
    success = { icon = "", color = "#1abc9c" },
    failure = { icon = "", color = "#db4b4b" },
    show_progress = false,
  },
  --- TODO(nvim): Limit devices platform to select from
  simctl = {
    iOS = {
      "iPhone 13 Pro",
      "iPad (9th generation)",
    },
  },
  mappings = {
    --- Whether xbase mapping should be disabled.
    enable = true,
    --- Open build picker. showing targets and configuration.
    build_picker = "<leader>b", --- set to 0 to disable
    --- Open run picker. showing targets, devices and configuration
    run_picker = "<leader>r", --- set to 0 to disable
    --- Open watch picker. showing run or build, targets, devices and configuration
    watch_picker = "<leader>s", --- set to 0 to disable
    --- A list of all the previous pickers
    all_picker = "<leader>ef", --- set to 0 to disable
    --- horizontal toggle log buffer
    toggle_split_log_buffer = "<leader>ls",
    --- vertical toggle log buffer
    toggle_vsplit_log_buffer = "<leader>lv",
  }
}
```

## Preview

Watch build service. 

![](./media/statusline_watch.gif)

On error it opens a log buffer where you can inspect what went wrong, otherwise only the
statusline get updated.

> NOTE: this is using a custom feline provider located in `watch.lua`, pr is welcome for other statusline support.

## Debugging

### Read logs
```bash 
# Daemon logs
tail -f /tmp/xbase-daemon.log
# Build Server logs
tail -f /tmp/xbase-server.log
```


[XcodeGen]: https://github.com/yonaskolb/XcodeGen
[sourcekit-lsp]: https://github.com/apple/sourcekit-lsp
[xbase]: https://github.com/tami5/xbase
[xcodebuild]: https://github.com/tami5/xcodebuild
[feline]: https://github.com/feline-nvim/feline.nvim
[XVim2]: https://github.com/XVimProject/XVim2
[rust]: https://www.rust-lang.org
