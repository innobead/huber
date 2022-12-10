[![crates.io](https://img.shields.io/crates/v/huber.svg)](https://crates.io/crates/huber)

![huber in action](./doc/huber.png)

**Huber** is to simplify the package management from GitHub projects with a builtin awesome list (live updating) of popular projects. It also supports *repository* feature for managing the package installation from your personal Github project. Please check the complete introduction as below.

> `Huber is a (new) tool for easy installation of dev/ops CLI tools directly from GitHub. No more trawling the releases pages!` introduced by [Coffee and Cloud Native - 88](https://www.youtube.com/watch?v=LgA6hpKdncw)

- Manages released artifacts from GitHub, and decide which version as the current to use
- Manages the default package repository with an awesome package list including popular star GitHub projects (ex: K8s, K3s, cloud-native, etc)
- Supports `huber.yaml` in any public/private GitHub repo aware to Huber to manage the described packages
- Supports secure protocols (`SSH`/`HTTPS`) for github communication
- Supports cross platforms
  - Linux x86_64/amd64
  - Linux arm64/aarch64
  - Linux arm-v7
  - MacOS x86_64/amd64
  - Windows x86_64/amd64

![huber in action](./doc/demo.svg)

# Getting Started

## Prerequisites

Please install necessary dependent libraries on the supported platforms to make sure huber work as expected. Use appropriate OS package manager to install the dependencies. 

### Linux (usually installed already)

```console
# Ubuntu
❯ apt install libssl-dev libarchive-dev

# openSUSE
❯ zypper install libopenssl-devel libarchive-devel

# CentOS
❯ dnf install openssl-devel libarchive-devel
```

### MacOS

```console
❯ brew install libarchive
❯ export PKG_CONFIG_PATH=/usr/local/opt/libarchive/lib/pkgconfig
❯ brew install openssl
```

### Windows

Please install [libarchive](https://www.libarchive.org/), which can be installed with Cygwin, MinGW (`pacman -S mingw-w64-x86_64-libarchive`) or Vcpkg (`vcpkg install libarchive`).

For vcpkg, you can refer to [vcpkg#quick-start-windows](https://github.com/microsoft/vcpkg#quick-start-windows).

## Installing Huber

### Installing the official version

For official releases, please install the latest release as below command. After the installation, `huber` will be automatically added to the environment path by updating the`$HOME/.bashrc`. 
But, if it does not work as expected, please remember to add `$HOME/.huber/bin` in the environment path, so `huber` will be looked up correctly. 

```console
# Cargo
❯ cargo install huber

# Linux
❯ curl -sfSL https://raw.githubusercontent.com/innobead/huber/master/hack/install.sh | bash

# Windows
❯ . { iwr -useb https://raw.githubusercontent.com/innobead/huber/master/hack/windows/install.ps1 } | iex; install
```

### Installing the development version

For Linux or Mac users, follow below steps to prepare the environment.

```console
# Setup development environment
❯ make set-dev

# Install huber in $HOME/.cargo/bin, $HOME/.huber/bin and update ~/.bashrc
❯ make install
```

For Windows users, follow below steps to prepare the environment.

1. Download VS 2019 build tool installer, and install C++ development toolchain
2. Pull vcpkg git repo (https://github.com/microsoft/vcpkg), then do bootstrap via `./bootstrap-vcpkg.bat`
3. Run `./hack/windows/setup-dev.ps1`

# Tutorials

> Note: 
> - If encountering github access rate limitation, suggest to set **GITHUB_TOKEN** environment variable when running the huber command, 
> or use the global option `--github-token` or configure the huber configuration via `huber config update`. 
> Ref: [how to create a personal access token](https://github.com/settings/tokens)
> 
> - If using **SSH** to connect github, must set **GITHUB_KEY** environment variable when running the huber command, 
> or use the global option `--github-key` or configure the huber configuration via `huber config update`.


```console
❯ huber help
huber 
Huber, simplify github package management

USAGE:
    huber [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --github-key <string>      Github SSH private key path for authenticating public/private github repository
                                   access. This is required if you connect github w/ SSH instead of https [env:
                                   GITHUB_KEY=]
    -t, --github-token <string>    Github token, used for authorized access instead of limited public access [env:
                                   GITHUB_TOKEN=]
    -l, --log-level <string>       Log level [default: error]  [possible values: off, error, warn, info, debug, trace]
    -o, --output <string>          Output format [default: console]  [possible values: console, json, yaml]

SUBCOMMANDS:
    config         Manages the configuration
    current        Updates the current package version [aliases: c]
    flush          Flushes inactive artifacts [aliases: f]
    help           Prints this message or the help of the given subcommand(s)
    info           Shows the package info [aliases: i]
    install        Installs the package [aliases: in]
    repo           Manages repositories
    reset          Resets huber [aliases: r]
    search         Searches package [aliases: se]
    self-update    Updates huber [aliases: su]
    show           Shows installed packages [aliases: s]
    uninstall      Uninstalls package [aliases: un, rm]
    update         Updates the installed package [aliases: u]

```
## Managing packages

### Searching packages

For the complete package list, please refer to [huber managed package list](doc/packages.md)

```console
❯ huber search
 Name         Description                                                                                              Source 
 arkade       Your one-stop CLI for Kubernetes                                                                         https://github.com/alexellis/arkade 
 bat          A cat(1) clone with wings.                                                                               https://github.com/sharkdp/bat 
 conftest     Write tests against structured configuration data using the Open Policy Agent Rego query language        https://github.com/open-policy-agent/conftest 
 consul       Consul is a distributed, highly available, and data center aware solution to connect and configure a...  https://github.com/hashicorp/consul 
 containerd   An open and reliable container runtime                                                                   https://github.com/containerd/containerd 
 ctlptl       Making local Kubernetes clusters fun and easy to set up                                                  https://github.com/tilt-dev/ctlptl 
 dasel        Query and update data structures from the command line. Comparable to jq/yq but supports JSON, TOML,...  https://github.com/TomWright/dasel 
 deno         A secure JavaScript and TypeScript runtime                                                               https://github.com/denoland/deno 
 faas-cli     Official CLI for OpenFaaS                                                                                https://github.com/openfaas/faas-cli 
 firecracker  Secure and fast microVMs for serverless computing.                                                       https://github.com/firecracker-microvm/firecracker 
 fission      Fast and Simple Serverless Functions for Kubernetes                                                      https://github.com/fission/fission 
 fleet        Manage large fleets of Kubernetes clusters                                                               https://github.com/rancher/fleet 
 gh           GitHub’s official command line tool                                                                      https://github.com/cli/cli 
 helm         The Kubernetes Package Manager                                                                           https://github.com/helm/helm 
 hub          A command-line tool that makes git easier to use with GitHub.                                            https://github.com/github/hub 
 huber        Huber,  simplify `GitHub release` package management 📦 with a live awesome list of GitHub project...    https://github.com/innobead/huber 
 istio        Connect, secure, control, and observe services.                                                          https://github.com/istio/istio 
 k0s          k0s - Zero Friction Kubernetes                                                                           https://github.com/k0sproject/k0s 
 k3ai         K3ai is a lightweight, fully automated, AI infrastructure-in-a-box solution that allows anyone to ex...  https://github.com/kf5i/k3ai 
 k3d          Little helper to run Rancher Lab's k3s in Docker                                                         https://github.com/rancher/k3d 
 k3s          Lightweight Kubernetes                                                                                   https://github.com/rancher/k3s 
 k3sup        bootstrap Kubernetes with k3s over SSH < 1 min 🚀                                                        https://github.com/alexellis/k3sup 
 k9s          🐶 Kubernetes CLI To Manage Your Clusters In Style!                                                      https://github.com/derailed/k9s 
 kind         Kubernetes IN Docker - local clusters for testing Kubernetes                                             https://github.com/kubernetes-sigs/kind 
 ...

```

### Searching package versions

```console
❯ huber search k3s
 Name  Description             Source 
 k3s   Lightweight Kubernetes  "https://github.com/rancher/k3s" 

❯ huber search -p k3
 Name   Description                                                                                              Source 
 k3ai   K3ai is a lightweight, fully automated, AI infrastructure-in-a-box solution that allows anyone to ex...  https://github.com/kf5i/k3ai 
 k3d    Little helper to run Rancher Lab's k3s in Docker                                                         https://github.com/rancher/k3d 
 k3s    Lightweight Kubernetes                                                                                   https://github.com/rancher/k3s 
 k3sup  bootstrap Kubernetes with k3s over SSH < 1 min 🚀                                                        https://github.com/alexellis/k3sup 

❯ huber search k3s --all
 Name  Version            Kind 
 k3s   v1.19.3-rc1+k3s2   PreRelease 
 k3s   v1.19.3-rc1+k3s1   PreRelease 
 k3s   v1.19.3+k3s2       Release 
 k3s   v1.19.3+k3s1       Release 
 k3s   v1.19.2-rc2+k3s1   PreRelease 
 k3s   v1.19.2-rc1+k3s1   PreRelease 
 k3s   v1.19.2+k3s1       Release 
 k3s   v1.19.1-rc2+k3s1   PreRelease 
 k3s   v1.19.1-rc1+k3s1   PreRelease 
 k3s   v1.19.1+k3s1       Release 
 k3s   v1.18.9-rc1+k3s1   PreRelease 
 k3s   v1.18.9+k3s1       Release 
 k3s   v1.18.8-rc1+k3s1   PreRelease 
 k3s   v1.18.8+k3s1       Release 
 k3s   v1.18.10-rc1+k3s2  PreRelease 
 k3s   v1.18.10-rc1+k3s1  PreRelease 
 k3s   v1.18.10+k3s2      Release 
 k3s   v1.18.10+k3s1      Release 
 k3s   v1.17.9+k3s1       Release 
 k3s   v1.17.13-rc1+k3s2  PreRelease 
 k3s   v1.17.13-rc1+k3s1  PreRelease 
 k3s   v1.17.13+k3s2      Release 
 k3s   v1.17.13+k3s1      Release 
 k3s   v1.17.12+k3s1      Release 
 k3s   v1.17.11-rc1+k3s1  PreRelease 
 k3s   v1.17.11+k3s1      Release 
 k3s   v1.16.15+k3s1      Release 
 k3s   v1.16.14-rc1+k3s1  PreRelease 
 k3s   v1.16.14+k3s1      Release 
 k3s   v1.16.13+k3s1      Release 
```

### Installing the package

```console
❯ huber install k3s
Installing k3s
Downloading package artifacts from github
Setting k3s (version: v1.19.3+k3s3, source: github) as the current package
Installed executables:
 - /home/davidko/.huber/bin/k3s
k3s (version: v1.19.3+k3s3, source: github) installed

❯ huber install k3s -v v1.18.10+k3s2
Updating k3s to k3s (version: v1.19.3+k3s3, source: github)
Downloading package artifacts from github
Setting k3s (version: v1.18.10+k3s2, source: github) as the current package
Installed executables:
 - /home/davidko/.huber/bin/k3s
k3s (version: v1.18.10+k3s2, source: github) updated
```

### Updating the package

```console
❯ huber update velero
Updating velero (version: v1.4.3, source: github) to the latest version
Downloading package artifacts from github
Setting velero (version: v1.5.2, source: github) as the current package
Installed executables:
 - /home/davidko/.huber/bin/velero
velero updated
```

### Uninstalling the package

```console
❯ huber uninstall k3s
Uninstalling k3s
k3s uninstalled
```

### Setting the current installed package version

```console
❯ huber show k3s --all
 Name  Version        Current  Executables                     Kind 
 k3s   v1.19.3+k3s3   false                                    Release 
 k3s   v1.18.10+k3s2  true     - /home/davidko/.huber/bin/k3s  Release 

❯ huber current k3s v1.19.3+k3s3
Setting k3s (version: v1.19.3+k3s3, source: github) as the current package
Updated executables:
 - /home/davidko/.huber/bin/k3s
k3s (version: v1.19.3+k3s3, source: github) as current updated

❯ huber show k3s --all
 Name  Version        Current  Executables                     Kind 
 k3s   v1.19.3+k3s3   true     - /home/davidko/.huber/bin/k3s  Release 
 k3s   v1.18.10+k3s2  false                                    Release 
```

### Showing the package info

```console
❯ huber info k3s
 Name  Version       Description  Source            Targets                        Release Kind 
 k3s   v1.19.3+k3s3               Github:           - LinuxAmd64:                  Release 
                                    owner: rancher      artifact_templates:         
                                    repo: k3s             - "{version}/k3s"         
                                                    - LinuxArm64:                   
                                                        artifact_templates:         
                                                          - "{version}/k3s-arm64"   

❯ huber info k3s -o yaml
---
name: k3s
version: v1.19.3+k3s3
description: ~
source:
  Github:
    owner: rancher
    repo: k3s
targets:
  - LinuxAmd64:
      artifact_templates:
        - "{version}/k3s"
  - LinuxArm64:
      artifact_templates:
        - "{version}/k3s-arm64"
detail:
  Github:
    package:
      url: "https://api.github.com/repos/rancher/k3s/releases/33895674"
      html_url: "https://github.com/rancher/k3s/releases/tag/v1.19.3%2Bk3s3"
      assets_url: "https://api.github.com/repos/rancher/k3s/releases/33895674/assets"
      upload_url: "https://uploads.github.com/repos/rancher/k3s/releases/33895674/assets{?name,label}"
      tarball_url: "https://api.github.com/repos/rancher/k3s/tarball/v1.19.3+k3s3"
      zipball_url: "https://api.github.com/repos/rancher/k3s/zipball/v1.19.3+k3s3"
      id: 33895674
      tag_name: v1.19.3+k3s3
      target_commitish: release-1.19
      name: v1.19.3+k3s3
      draft: false
      prerelease: false
      created_at: "2020-11-13T05:06:42Z"
      published_at: "2020-11-13T07:14:25Z"
      assets:
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267759"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/e2e-passed-amd64-parallel.log"
          id: 28267759
          name: e2e-passed-amd64-parallel.log
          label: ""
          state: uploaded
          content_type: text/plain; charset=utf-8
          size: 1256238
          download_count: 8
          created_at: "2020-11-13T07:38:19Z"
          updated_at: "2020-11-13T07:38:20Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267760"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/e2e-passed-amd64-serial.log"
          id: 28267760
          name: e2e-passed-amd64-serial.log
          label: ""
          state: uploaded
          content_type: text/plain; charset=utf-8
          size: 86608
          download_count: 6
          created_at: "2020-11-13T07:38:20Z"
          updated_at: "2020-11-13T07:38:20Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267171"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/k3s"
          id: 28267171
          name: k3s
          label: ""
          state: uploaded
          content_type: application/octet-stream
          size: 53481472
          download_count: 5435
          created_at: "2020-11-13T07:20:38Z"
          updated_at: "2020-11-13T07:20:39Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267172"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/k3s-airgap-images-amd64.tar"
          id: 28267172
          name: k3s-airgap-images-amd64.tar
          label: ""
          state: uploaded
          content_type: application/x-tar
          size: 369606656
          download_count: 267
          created_at: "2020-11-13T07:20:39Z"
          updated_at: "2020-11-13T07:20:46Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267106"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/k3s-airgap-images-arm.tar"
          id: 28267106
          name: k3s-airgap-images-arm.tar
          label: ""
          state: uploaded
          content_type: application/x-tar
          size: 318740480
          download_count: 22
          created_at: "2020-11-13T07:18:10Z"
          updated_at: "2020-11-13T07:18:19Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267043"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/k3s-airgap-images-arm64.tar"
          id: 28267043
          name: k3s-airgap-images-arm64.tar
          label: ""
          state: uploaded
          content_type: application/x-tar
          size: 352361984
          download_count: 32
          created_at: "2020-11-13T07:16:51Z"
          updated_at: "2020-11-13T07:16:57Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267047"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/k3s-arm64"
          id: 28267047
          name: k3s-arm64
          label: ""
          state: uploaded
          content_type: application/octet-stream
          size: 48758784
          download_count: 699
          created_at: "2020-11-13T07:16:57Z"
          updated_at: "2020-11-13T07:16:58Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267108"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/k3s-armhf"
          id: 28267108
          name: k3s-armhf
          label: ""
          state: uploaded
          content_type: application/octet-stream
          size: 48496640
          download_count: 624
          created_at: "2020-11-13T07:18:19Z"
          updated_at: "2020-11-13T07:18:21Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267173"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/k3s-images.txt"
          id: 28267173
          name: k3s-images.txt
          label: ""
          state: uploaded
          content_type: text/plain; charset=utf-8
          size: 313
          download_count: 49
          created_at: "2020-11-13T07:20:46Z"
          updated_at: "2020-11-13T07:20:46Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267174"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/sha256sum-amd64.txt"
          id: 28267174
          name: sha256sum-amd64.txt
          label: ""
          state: uploaded
          content_type: text/plain; charset=utf-8
          size: 245
          download_count: 5745
          created_at: "2020-11-13T07:20:46Z"
          updated_at: "2020-11-13T07:20:47Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267110"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/sha256sum-arm.txt"
          id: 28267110
          name: sha256sum-arm.txt
          label: ""
          state: uploaded
          content_type: text/plain; charset=utf-8
          size: 168
          download_count: 805
          created_at: "2020-11-13T07:18:21Z"
          updated_at: "2020-11-13T07:18:21Z"
        - url: "https://api.github.com/repos/rancher/k3s/releases/assets/28267048"
          browser_download_url: "https://github.com/rancher/k3s/releases/download/v1.19.3%2Bk3s3/sha256sum-arm64.txt"
          id: 28267048
          name: sha256sum-arm64.txt
          label: ""
          state: uploaded
          content_type: text/plain; charset=utf-8
          size: 170
          download_count: 760
          created_at: "2020-11-13T07:16:59Z"
          updated_at: "2020-11-13T07:16:59Z"
release_kind: Release
```

### Showing the installed package version info

```console
❯ huber show
 Name  Version       Current  Kind 
 k3s   v1.19.3+k3s3  true     Release

❯ huber show k3s
 Name  Version       Current  Executables                     Kind 
 k3s   v1.19.3+k3s3  true     - /home/davidko/.huber/bin/k3s  Release 

❯ huber show k3s --all
 Name  Version        Current  Executables                     Kind 
 k3s   v1.19.3+k3s3   true     - /home/davidko/.huber/bin/k3s  Release 
 k3s   v1.18.10+k3s2  false                                    Release 
```

### Flushing non-current packages

```console
❯ huber show k3s --all
 Name  Version        Current  Executables                     Kind 
 k3s   v1.19.3+k3s3   true     - /home/davidko/.huber/bin/k3s  Release 
 k3s   v1.18.10+k3s2  false                                    Release 

❯ huber flush
Removing k3s (version: v1.18.10+k3s2, source: github)

❯ huber show --all
 Name  Version       Current  Executables                     Kind 
 k3s   v1.19.3+k3s3  true     - /home/davidko/.huber/bin/k3s  Release 
```

## Managing repositories

### Adding a repository
The repository can be private, public github repos or even a local huber.yaml. If it's private, make sure you have the permission
to access via the authorized SSH private key.

```console
❯ huber repo add remote-repo --url https://github.com/innobead/huber_unmanaged_demo
Repository { name: "remote-repo", url: Some("https://github.com/innobead/huber_unmanaged_demo"), file: None } added

❯ huber repo add local-repo -f huber.yaml
Repository { name: "local-repo", url: None, file: Some("huber.yaml") } added
```

**Add huber.yaml in the top-level directory of the repository** (ref: [huber.yaml example](https://github.com/innobead/huber_unmanaged_demo/blob/master/huber.yaml))
```yaml
---
- name: conftest2
  description: Write tests against structured configuration data using the Open Policy Agent Rego query language
  source:
    Github:
      owner: open-policy-agent
      repo: conftest
  targets:
    - LinuxAmd64:
        artifact_templates:
          - "conftest_{version}_Linux_x86_64.tar.gz"
    - LinuxAmd64:
        artifact_templates:
          - "conftest_{version}_Linux_arm64.tar.gz"
    - MacOS:
        artifact_templates:
          - "conftest_{version}_Darwin_x86_64.tar.gz"
    - Windows:
        artifact_templates:
          - "conftest_{version}_Windows_x86_64.zip"
  detail: ~
```

### Listing repositories
```console
❯ huber repo list
 Name         Url                                               File
 remote-repo  https://github.com/innobead/huber_unmanaged_demo
```

### Installing the package from the repository
```console
❯ huber install conftest2
Installing conftest2
Downloading package artifacts from github
Setting conftest2 (version: v0.22.0, source: github) as the current package
Installed executables:
 - /home/davidko/.huber/bin/conftest
conftest2 (version: v0.22.0, source: github) installed
```

### Deleting a repository
```console
❯ huber repo remove remote-repo
remote-repo removed
```

## Managing configuration
### Updating the configuration
```console
❯ huber config update --github-token mytoken --github-key /home/davidko/.ssh/id_rsa
Updating the configuration
The configuration updated

 Log Level  Output Format  Github Token  Github Key 
 ERROR      Console        mytoken       /home/davidko/.ssh/id_rsa
```

### Showing the configuration
```console
❯ huber config show
 Log Level  Output Format  Github Token  Github Key 
 ERROR      Console        mytoken       /home/davidko/.ssh/id_rsa 
```

## Others

### Resetting Huber

```console
❯ huber reset
Resetting huber by removing created caches, downloaded files and installed packages
Done

```

### Self updating Huber

```console
❯ huber self-update
Updating huber to v1.0.0
huber v1.0.0 updated

❯ huber self-update
Error: No update available: 404 Not Found: Not Found

```

# Contribution

If you would like to add some useful tools in the builtin managed packages list, please have a PR as below steps.

1. Add a new package module in `crates/generator/src/pkg`
2. Update the added package in `crates/generator/src/build.rs`
3. `make generate` w/ your `GITHUB_TOKEN` to check if the new package manifest generated in `generated/packages` and `generated/index.yaml` updated accordingly
4. Fire a PR to make it accept

