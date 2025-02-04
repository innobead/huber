<div align="center" style="text-align: center;">
<img src="https://raw.githubusercontent.com/innobead/huber/HEAD/docs/src/images/huber_logo.png" alt="huber" style="width:300px;"/>
</div>

<div align="center">

[![crates.io](https://img.shields.io/crates/v/huber.svg)](https://crates.io/crates/huber)
[![Releases](https://img.shields.io/github/release/innobead/huber/all.svg)](https://github.com/innobead/huber/releases)
[![GitHub](https://img.shields.io/github/license/innobead/huber)](https://github.com/innobead/huber/blob/master/LICENSE)
[![Docs](https://img.shields.io/badge/docs-latest-green.svg)](https://innobead.github.io/huber/)

</div>

**Huber** is a command-line interface tool for managing packages released from GitHub repositories. It allows you to install, update, and manage packages from GitHub repository releases in a simple and efficient way.

What features does Huber provide?

- Manage (install, update, uninstall, show, current) multiple version packages from GitHub repository releases
- Search popular GitHub repositories that Huber manages in a curated list
- Manage your own repositories to install packages you need
- Lock and unlock installed package versions
- Save and restore package versions
- and more..., please check the documentation for more details

> [!NOTE]  
> This documentation is for the version starting from 1.0.0. If you are using older versions, suggest upgrading to the latest version.

# Installation

Huber is supported on Linux, macOS, and Windows platforms.

- Linux (x86_64/amd64, aarch64/arm64, arm)
- MacOS (x86_64/amd64, aarch64/arm64)
- Windows (x86_64/amd64)

You can install Huber via the following methods:

**Cargo:**

```shell
$ cargo install huber
```

**Shell script:**

```shell
$ curl -sfSL https://raw.githubusercontent.com/innobead/huber/main/hack/install.sh | sh -
```

**PowerShell:**

```powershell
. { iwr -useb https://raw.githubusercontent.com/innobead/huber/main/hack/windows/install.ps1 } | iex; install
```

**Precompiled binaries:**

Download Huber executables from [GitHub releases](https://github.com/innobead/huber/releases)

# Getting Started

After installing Huber, you can start using it by running the `huber` command.

```shell
$ huber --help
Huber, simplify GitHub package management

Usage: huber [OPTIONS] <COMMAND>

Commands:
  config       Manage Huber configurations
  current      Update the current package versions
  completions  Show command completions for the specified shell
  flush        Remove outdated installed artifacts
  info         Show package information
  install      Install packages
  repo         Manage repositories
  reset        Reset Huber
  search       Search package
  self-update  Update huber
  show         Show installed packages
  uninstall    Uninstall packages
  update       Updates the installed packages
  save         Save the installed package list to a file
  load         Load installed packages from a file generated by save command
  lock         Lock packages or Show locked packages
  unlock       Unlock packages
  help         Print this message or the help of the given subcommand(s)

Options:
  -l, --log-level <LOG_LEVEL>
          Log level [default: OFF]
      --github-token <GITHUB_TOKEN>
          GitHub token; Optional until reaching the rate limit of GitHub API [env: GITHUB_TOKEN=]
      --github-key <GITHUB_KEY>
          Github SSH key path; Optional, if you want to use SSH to clone the Huber repository [env: GITHUB_KEY=]
      --huber-dir <HUBER_DIR>
          Huber directory [default: /home/davidko/.huber]
      --github-base-uri <GITHUB_BASE_URI>
          GitHub base URI [env: GITHUB_BASE_URI=] [default: https://api.github.com]
  -h, --help
          Print help
  -V, --version
          Print version
```

Search and install a package:

```shell
$ huber search k9s
 Name  Description                                          Source 
 k9s   🐶 Kubernetes CLI To Manage Your Clusters In Style!  https://github.com/derailed/k9s 
 
$ huber install k9s
[INFO ] k9s version not specified, getting the latest version (v0.32.7)
[INFO ] Installing package k9s@latest/v0.32.7
[INFO ] Downloading https://github.com/derailed/k9s/releases/download/v0.32.7/k9s_Linux_amd64.tar.gz
[INFO ] Installed executables of k9s:
    [
        "/home/davidko/.huber/bin/k9s",
    ]
[INFO ] k9s@latest/v0.32.7 installed

$ k9s version
 ____  __.________       
|    |/ _/   __   \______
|      < \____    /  ___/
|    |  \   /    /\___ \ 
|____|__ \ /____//____  >
        \/            \/ 

Version:    v0.32.7
Commit:     6b5d24f5741a1789fb97ba3e11f0ee868d93459d
Date:       2024-11-16T20:22:28Z
```

For frequently used and advanced commands, please check the detailed documentation [here](https://innobead.github.io/huber/).

# License

The Huber source and documentation are released under the Apache License v2.0.
