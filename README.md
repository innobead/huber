# What is Huber?

**Huber** is a single development entry to simplify 'github release' package management. 

- Manages released artifacts from github, and decide which version as the current to use ✅
- Provides a managed package list of famous started github projects ✅
- Provides to manage self own package list for any development purpose *(soon)*
- 'huber.yaml' can be created in any public/private github repo to be aware to Huber to manage released artifacts *(soon)*   
- Supports cross platforms *(Linux ready, MacOS andWindows soon)* 

# Getting Started

## Installing Huber (official release soon)

### Cargo

```console
# Have cargo, rustc installed via rustup
make install

# Add huber home/bin in your environment path
export PATH=$HOME/.huber/bin:$PATH
```

# Tutorials

> Note: 
> If encountering github access rate limitation, suggest to set GITHUB_TOKEN environment when running huber command, 
> or use the global option '--github-token' instead.

## Searching available packages

```console
❯ huber search
 Name         Description                                                                                                                           Source 
 containerd   An open and reliable container runtime                                                                                              https://github.com/containerd/containerd 
 deno         A secure JavaScript and TypeScript runtime                                                                                          https://github.com/denoland/deno 
 firecracker  Secure and fast microVMs for serverless computing.                                                                                  https://github.com/firecracker-microvm/firecracker 
 fleet        Manage large fleets of Kubernetes clusters                                                                                          https://github.com/rancher/fleet 
 gh           GitHub’s official command line tool                                                                                                 https://github.com/cli/cli 
 helm         The Kubernetes Package Manager                                                                                                      https://github.com/helm/helm 
 istio        Connect, secure, control, and observe services.                                                                                     https://github.com/istio/istio 
 k3s          Lightweight Kubernetes                                                                                                              https://github.com/rancher/k3s 
 kube-bench   Checks whether Kubernetes is deployed according to security best practices as defined in the CIS Kubernetes Benchmark               https://github.com/aquasecurity/kube-bench 
 kubefire     KubeFire, creates and manages Kubernetes Clusters using Firecracker microVMs                                                        https://github.com/innobead/kubefire 
 pulumi       Pulumi - Modern Infrastructure as Code. Any cloud, any language 🚀                                                                  https://github.com/pulumi/pulumi 
 rio          Application Deployment Engine for Kubernetes                                                                                        https://github.com/rancher/rio 
 rke          Rancher Kubernetes Engine (RKE), an extremely simple, lightning fast Kubernetes distribution that runs entirely within containers.  https://github.com/rancher/rke 
 trivy        A Simple and Comprehensive Vulnerability Scanner for Containers, Suitable for CI                                                    https://github.com/aquasecurity/trivy 
 typescript   TypeScript is a superset of JavaScript that compiles to clean JavaScript output.                                                    https://github.com/microsoft/TypeScript 
 velero       Backup and migrate Kubernetes applications and their persistent volumes                                                             https://github.com/vmware-tanzu/velero 

```

## Searching available package versions

```console
❯ huber search -n k3s
 Name  Description             Source 
 k3s   Lightweight Kubernetes  "https://github.com/rancher/k3s" 

❯ huber search -n k3s --all
 Name  Version 
 k3s   v1.19.3+k3s2 
 k3s   v1.18.10+k3s2 
 k3s   v1.17.13+k3s2 
 k3s   v1.19.3-rc1+k3s2 
 k3s   v1.17.13-rc1+k3s2 
 k3s   v1.18.10-rc1+k3s2 
 k3s   v1.18.10+k3s1 
 k3s   v1.17.13+k3s1 
 k3s   v1.18.10-rc1+k3s1 
 k3s   v1.17.13-rc1+k3s1 
 k3s   v1.19.3-rc1+k3s1 
 k3s   v1.19.3+k3s1 
 k3s   v1.19.2+k3s1 
 k3s   v1.19.2-rc2+k3s1 
 k3s   v1.19.2-rc1+k3s1 
 k3s   v1.18.9-rc1+k3s1 
 k3s   v1.17.12+k3s1 
 k3s   v1.18.9+k3s1 
 k3s   v1.19.1-rc2+k3s1 
 k3s   v1.19.1+k3s1 
 k3s   v1.19.1-rc1+k3s1 
 k3s   v1.16.15+k3s1 
 k3s   v1.18.8-rc1+k3s1 
 k3s   v1.17.11-rc1+k3s1 
 k3s   v1.16.14-rc1+k3s1 
 k3s   v1.16.14+k3s1 
 k3s   v1.17.11+k3s1 
 k3s   v1.18.8+k3s1 
 k3s   v1.16.13+k3s1 
 k3s   v1.17.9+k3s1 

```

## Installing the package

```console
❯ huber install k3s
Installing k3s
Downloading package artifacts from github
Setting k3s (version: v1.19.3+k3s2, source: github) as the current package
k3s (version: v1.19.3+k3s2, source: github) installed

❯ huber install k3s -v v1.18.10+k3s2
Updating k3s to k3s (version: v1.19.3+k3s2, source: github)
Downloading package artifacts from github
Setting k3s (version: v1.18.10+k3s2, source: github) as the current package
k3s (version: v1.18.10+k3s2, source: github) updated

```

## Updating the package

```console
❯ huber update velero
Updating velero (version: v1.4.3, source: github) to the latest version
Downloading package artifacts from github
Setting velero (version: v1.5.2, source: github) as the current package
velero updated

```

## Setting the current installed package version

```console
❯ huber show -n gh --all
 Name  Version  Current 
 gh    v1.2.0   false 
 gh    v1.1.0   true 

❯ huber current gh -v v1.2.0
Setting gh (version: v1.2.0, source: github) as the current package
gh (version: v1.2.0, source: github) as current updated

❯ huber show -n gh --all
 Name  Version  Current 
 gh    v1.2.0   true 
 gh    v1.1.0   false 

```

## Showing the package info

```console
❯ huber info k3s
 Name  Source            Targets                        Version 
 k3s   Github:           - LinuxAmd64:                  v1.19.3+k3s2 
         owner: rancher      artifact_templates:         
         repo: k3s             - "{version}/k3s"         
                             checksum: ~                 
                         - LinuxArm64:                   
                             artifact_templates:         
                               - "{version}/k3s-arm64"   
                             checksum: ~   

❯ huber info k3s -o yaml
---
name: k3s
version: v1.19.3+k3s2
source:
  Github:
    owner: rancher
    repo: k3s
targets:
  - LinuxAmd64:
      artifact_templates:
        - "{version}/k3s"
      checksum: ~
  - LinuxArm64:
      artifact_templates:
        - "{version}/k3s-arm64"
      checksum: ~

```

## Showing the installed package version info

```console
❯ huber show
 Name    Version       Current 
 velero  v1.5.2        true 
 k3s     v1.19.3+k3s2  true

❯ huber show -n k3s
 Name  Version       Current 
 k3s   v1.19.3+k3s2  true

❯ huber show -n k3s --all
 Name  Version        Current 
 k3s   v1.19.3+k3s2   false 
 k3s   v1.18.10+k3s2  true 

```

## Flushing non-current packages

```console
❯ huber show --all
 Name    Version        Current 
 velero  v1.5.2         true 
 k3s     v1.19.3+k3s2   false 
 k3s     v1.18.10+k3s2  true 

❯ huber flush
Bypassed velero, no inactive releases to remove
Removing k3s (version: v1.19.3+k3s2, source: github)

❯ huber show --all
 Name    Version        Current 
 velero  v1.5.2         true 
 k3s     v1.18.10+k3s2  true

```

## Uninstalling the package

```console
❯ huber uninstall k3s
Uninstalling k3s
k3s uninstalled

```

## Resetting Huber

```console
❯ huber reset
Resetting huber by removing created caches, downloaded files and installed packages
Done

```

## Self updating Huber

```console
❯ huber self-update

```

## Misc

Please check the command help to see all supported capabilities.

```console
❯ huber help
huber 0.1.0
Huber, simplify github package management

USAGE:
    huber [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --github-token <string>    Github token, used for authored access instead of limited public access [env:
                                   GITHUB_TOKEN=]
    -l, --log-level <string>       Log level [default: off]  [possible values: off, error, warn, info, debug, trace]
    -o, --output <string>          Output format [default: console]  [possible values: console, json, yaml]

SUBCOMMANDS:
    current        Updates the current package version [aliases: c]
    flush          Flushes inactive artifacts [aliases: f]
    help           Prints this message or the help of the given subcommand(s)
    info           Shows the package info [aliases: i]
    install        Installs the package [aliases: in]
    reset          Resets huber [aliases: r]
    search         Searches package [aliases: se]
    self-update    Updates huber [aliases: su]
    show           Shows installed packages [aliases: s]
    uninstall      Uninstalls package [aliases: un]
    update         Updates the installed package [aliases: u]

```

# Contribution

If you would like to add some useful tools in the builtin manged packages list, please have a PR as below steps.

1. Add a new package module in `src/generator/src/pkg`
2. Update the added package in `src/generator/src/build.rs`
3. `make generate` w/ your `GITHUB_TOKEN` to check if the new package manifest generated in `generated/packages` and `generated/index.yaml` updated accordingly
4. Fire a PR to make it accept

# Notes

- Huber is not product ready project, but active under development. Any feedback is welcome!