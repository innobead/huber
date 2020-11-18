# What is Huber 📦

**Huber** is a single development entry to simplify `GitHub release` package management with a live awesome list of GitHub projects.

- Manages released artifacts from github, and decide which version as the current to use
- Manages the default package repository with an awesome package list including popular star github projects (ex: K8s, K3s, cloud-native, etc)
- Supports `huber.yaml` in any public/private github repo aware to Huber to manage the described packages
- Supports secure protocols (`SSH`/`HTTPS`) for github communication
- Supports cross platforms

# Getting Started

## Prerequisites
For Linux/MacOS users, please install necessary dependent libraries to make sure huber work as expected. Please run `make set-dev` or `hack/setup-dev.sh` to setup the environment or use any appropriate OS package manager to install the dependencies.

### Linux
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

## Installing Huber

### Installing the official version

For official releases, please install the latest release as below command. After the installation, `huber` path will be automatically added to the environment path by updating the`$HOME/.bashrc`. 
But, if it does not work as expected, please remember to add `$HOME/.huber/bin` in the environment path, so `huber` will be looked up correctly. 

```console
❯ curl -sfSL https://raw.githubusercontent.com/innobead/huber/master/hack/install.sh | bash

# Update the environment path if required
❯ echo "export PATH=\$HOME/.huber/bin:\$PATH" > ~/.bashrc
```

### Installing the development version

For development purpose, please make sure go 1.14 installed, then build and install `kubefire` in the `GOBIN` path.

```console
# Setup development environment
❯ make set-dev

# Install huber in $HOME/.cargo/bin, $HOME/.huber/bin and update ~/.bashrc
❯ make install
```

# Tutorials

- [Managing packages](#managing-packages)
    - [Searching packages](#searching-packages)
    - [Searching package versions](#searching-package-versions)
    - [Installing the package](#installing-the-package)
    - [Updating the package](#updating-the-package)
    - [Uninstalling the package](#uninstalling-the-package)
    - [Setting the current installed package version](#setting-the-current-installed-package-version)
    - [Showing the package info](#showing-the-package-info)
    - [Showing the installed package version info](#showing-the-installed-package-version-info)
    - [Flushing non-current packages](#flushing-non-current-packages)
- [Manages repositories](#manages-repositories)
    - [Adding a repository](#adding-a-repository)
    - [Listing repositories](#listing-repositories)
    - [Installing the package from the repository](#installing-the-package-from-the-repository)
    - [Deleting a repository](#deleting-a-repository)
- [Manage the configuration](#manage-the-configuration)
    - [Updating the configuration](#updating-the-configuration)
    - [Showing the configuration](#showing-the-configuration)
- [Others](#others)
    - [Resetting Huber](#resetting-huber)
    - [Self updating Huber](#self-updating-huber)


> Note: 
> - If encountering github access rate limitation, suggest to set **GITHUB_TOKEN** environment when running huber command, 
> or use the global option `--github-token` or configure the huber configuration via `huber config update`. 
> Ref: [how to create a personal access token](https://github.com/settings/tokens)
> 
> - If you use **SSH** instead of **HTTPS** to connect github, use **GITHUB_KEY** environment when running huber command, 
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
 kpt                                                                                                                   https://github.com/GoogleContainerTools/kpt 
 krew         📦 Find and install kubectl plugins                                                                      https://github.com/kubernetes-sigs/krew 
 kube-bench   Checks whether Kubernetes is deployed according to security best practices as defined in the CIS Kub...  https://github.com/aquasecurity/kube-bench 
 kubectl      Production-Grade Container Scheduling and Management                                                     https://github.com/kubernetes/kubernetes 
 kubefire     KubeFire, creates and manages Kubernetes Clusters using Firecracker microVMs                             https://github.com/innobead/kubefire 
 kudo         Kubernetes Universal Declarative Operator (KUDO)                                                         https://github.com/kudobuilder/kudo 
 kuttl        KUbernetes Test TooL (kuttl)                                                                             https://github.com/kudobuilder/kuttl 
 lens         Lens - The Kubernetes IDE                                                                                https://github.com/lensapp/lens 
 minikube     Run Kubernetes locally                                                                                   https://github.com/kubernetes/minikube 
 mkcert       A simple zero-config tool to make locally trusted development certificates with any names you'd like...  https://github.com/FiloSottile/mkcert 
 nomad        Nomad is an easy-to-use, flexible, and performant workload orchestrator that can deploy a mix of mic...  https://github.com/hashicorp/nomad 
 octant       Highly extensible platform for developers to better understand the complexity of Kubernetes clusters...  https://github.com/vmware-tanzu/octant 
 okteto       Develop your applications directly in your Kubernetes Cluster                                            https://github.com/okteto/okteto 
 opa          An open source, general-purpose policy engine.                                                           https://github.com/open-policy-agent/opa 
 oras         OCI Registry As Storage                                                                                  https://github.com/deislabs/oras 
 pack         CLI for building apps using Cloud Native Buildpacks                                                      https://github.com/buildpacks/pack 
 packer       Packer is a tool for creating identical machine images for multiple platforms from a single source c...  https://github.com/hashicorp/packer 
 powershell   PowerShell for every system!                                                                             https://github.com/PowerShell/PowerShell 
 pulumi       Pulumi - Modern Infrastructure as Code. Any cloud, any language 🚀                                       https://github.com/pulumi/pulumi 
 rio          Application Deployment Engine for Kubernetes                                                             https://github.com/rancher/rio 
 rke          Rancher Kubernetes Engine (RKE), an extremely simple, lightning fast Kubernetes distribution that ru...  https://github.com/rancher/rke 
 rke2                                                                                                                  https://github.com/rancher/rke2 
 skaffold     Easy and Repeatable Kubernetes Development                                                               https://github.com/GoogleContainerTools/skaffold 
 sonobuoy     Sonobuoy is a diagnostic tool that makes it easier to understand the state of a Kubernetes cluster b...  https://github.com/vmware-tanzu/sonobuoy 
 submariner   Submariner's Operator installs and maintains your Submariner deployment.                                 https://github.com/submariner-io/submariner-operator 
 syncthing    Open Source Continuous File Synchronization                                                              https://github.com/syncthing/syncthing 
 terraform    Terraform enables you to safely and predictably create, change, and improve infrastructure. It is an...  https://github.com/hashicorp/terraform 
 tilt         A multi-service dev environment for teams on Kubernetes                                                  https://github.com/tilt-dev/tilt 
 trivy        A Simple and Comprehensive Vulnerability Scanner for Containers, Suitable for CI                         https://github.com/aquasecurity/trivy 
 typescript   TypeScript is a superset of JavaScript that compiles to clean JavaScript output.                         https://github.com/microsoft/TypeScript 
 velero       Backup and migrate Kubernetes applications and their persistent volumes                                  https://github.com/vmware-tanzu/velero 
 waypoint     A tool to build, deploy, and release any application on any platform.                                    https://github.com/hashicorp/waypoint 
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

## Manages repositories

### Adding a repository
The repository can be private or publice. If it's private, make sure you have the permission to access via the authroized SSH private key.

```console
❯ huber repo add 3rdparty-repo https://github.com/innobead/huber_unmanaged_demo
Repository { name: "3rdparty-repo", url: "https://github.com/innobead/huber_unmanaged_demo" } added
```

**Add huber.yaml in the first folder layer of the repository** (ref: [huber.yaml example](https://github.com/innobead/huber_unmanaged_demo/blob/master/huber.yaml))
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
 Name           Url 
 3rdparty-repo  https://github.com/innobead/huber_unmanaged_demo
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
❯ huber repo remove 3rdparty-repo
3rdparty-repo removed
```

## Manage the configuration
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

If you would like to add some useful tools in the builtin manged packages list, please have a PR as below steps.

1. Add a new package module in `src/generator/src/pkg`
2. Update the added package in `src/generator/src/build.rs`
3. `make generate` w/ your `GITHUB_TOKEN` to check if the new package manifest generated in `generated/packages` and `generated/index.yaml` updated accordingly
4. Fire a PR to make it accept

# Notes

- Huber is not product ready project, but active under development. Any feedback is welcome!
