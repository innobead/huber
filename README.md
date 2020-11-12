# What is Huber 📦

**Huber** is a single development entry to simplify `GitHub release` package management with a live awesome list of GitHub projects.

- Manages released artifacts from github, and decide which version as the current to use ✅
- Manages a awesome package list including popular star github projects across different development areas ✅
- Supports to manage self own package list for any development purpose ✅
- Supports `huber.yaml` in any public/private github repo aware to Huber to manage the described packages ✅   
- Supports cross platforms *(Linux, MacOS ready, and Windows soon)* 

# Getting Started

## Installing Huber
> *The development way is provided for now, but the official release soon

```console
# Setup development environment
make set-dev

# Install huber in $HOME/.cargo/bin, $HOME/.huber/bin and update ~/.bashrc
make install
```

# Tutorials

> Note: 
> If encountering github access rate limitation, suggest to set GITHUB_TOKEN environment when running huber command, 
> or use the global option '--github-token' instead.

## Searching available packages

```console
❯ huber search
 Name         Description                                                                                              Source 
 bat          A cat(1) clone with wings.                                                                               https://github.com/sharkdp/bat 
 consul       Consul is a distributed, highly available, and data center aware solution to connect and configure a...  https://github.com/hashicorp/consul 
 containerd   An open and reliable container runtime                                                                   https://github.com/containerd/containerd 
 dasel        Query and update data structures from the command line. Comparable to jq/yq but supports JSON, TOML,...  https://github.com/TomWright/dasel 
 deno         A secure JavaScript and TypeScript runtime                                                               https://github.com/denoland/deno 
 firecracker  Secure and fast microVMs for serverless computing.                                                       https://github.com/firecracker-microvm/firecracker 
 fleet        Manage large fleets of Kubernetes clusters                                                               https://github.com/rancher/fleet 
 gh           GitHub’s official command line tool                                                                      https://github.com/cli/cli 
 helm         The Kubernetes Package Manager                                                                           https://github.com/helm/helm 
 istio        Connect, secure, control, and observe services.                                                          https://github.com/istio/istio 
 k3ai         K3ai is a lightweight, fully automated, AI infrastructure-in-a-box solution that allows anyone to ex...  https://github.com/kf5i/k3ai 
 k3d          Little helper to run Rancher Lab's k3s in Docker                                                         https://github.com/rancher/k3d 
 k3s          Lightweight Kubernetes                                                                                   https://github.com/rancher/k3s 
 k3sup        bootstrap Kubernetes with k3s over SSH < 1 min 🚀                                                        https://github.com/alexellis/k3sup 
 kind         Kubernetes IN Docker - local clusters for testing Kubernetes                                             https://github.com/kubernetes-sigs/kind 
 kpt                                                                                                                   https://github.com/GoogleContainerTools/kpt 
 krew         📦 Find and install kubectl plugins                                                                      https://github.com/kubernetes-sigs/krew 
 kube-bench   Checks whether Kubernetes is deployed according to security best practices as defined in the CIS Kub...  https://github.com/aquasecurity/kube-bench 
 kubefire     KubeFire, creates and manages Kubernetes Clusters using Firecracker microVMs                             https://github.com/innobead/kubefire 
 minikube     Run Kubernetes locally                                                                                   https://github.com/kubernetes/minikube 
 nomad        Nomad is an easy-to-use, flexible, and performant workload orchestrator that can deploy a mix of mic...  https://github.com/hashicorp/nomad 
 octant       Highly extensible platform for developers to better understand the complexity of Kubernetes clusters...  https://github.com/vmware-tanzu/octant 
 okteto       Develop your applications directly in your Kubernetes Cluster                                            https://github.com/okteto/okteto 
 opa          An open source, general-purpose policy engine.                                                           https://github.com/open-policy-agent/opa 
 oras         OCI Registry As Storage                                                                                  https://github.com/deislabs/oras 
 pack         CLI for building apps using Cloud Native Buildpacks                                                      https://github.com/buildpacks/pack 
 packer       Packer is a tool for creating identical machine images for multiple platforms from a single source c...  https://github.com/hashicorp/packer 
 pulumi       Pulumi - Modern Infrastructure as Code. Any cloud, any language 🚀                                       https://github.com/pulumi/pulumi 
 rio          Application Deployment Engine for Kubernetes                                                             https://github.com/rancher/rio 
 rke          Rancher Kubernetes Engine (RKE), an extremely simple, lightning fast Kubernetes distribution that ru...  https://github.com/rancher/rke 
 skaffold     Easy and Repeatable Kubernetes Development                                                               https://github.com/GoogleContainerTools/skaffold 
 sonobuoy     Sonobuoy is a diagnostic tool that makes it easier to understand the state of a Kubernetes cluster b...  https://github.com/vmware-tanzu/sonobuoy 
 submariner   Submariner's Operator installs and maintains your Submariner deployment.                                 https://github.com/submariner-io/submariner-operator 
 terraform    Terraform enables you to safely and predictably create, change, and improve infrastructure. It is an...  https://github.com/hashicorp/terraform 
 trivy        A Simple and Comprehensive Vulnerability Scanner for Containers, Suitable for CI                         https://github.com/aquasecurity/trivy 
 typescript   TypeScript is a superset of JavaScript that compiles to clean JavaScript output.                         https://github.com/microsoft/TypeScript 
 velero       Backup and migrate Kubernetes applications and their persistent volumes                                  https://github.com/vmware-tanzu/velero 
 waypoint     A tool to build, deploy, and release any application on any platform.                                    https://github.com/hashicorp/waypoint 
...

```

## Searching available package versions

```console
❯ huber search k3s
 Name  Description             Source 
 k3s   Lightweight Kubernetes  "https://github.com/rancher/k3s" 

❯ huber search -p k3 -a
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

## Installing the package

```console
❯ huber install k3s
Installing k3s
Downloading package artifacts from github
Setting k3s (version: v1.19.3+k3s2, source: github) as the current package
Installed executables:
 - /home/davidko/.huber/bin/k3s
k3s (version: v1.19.3+k3s2, source: github) installed

❯ huber install k3s -v v1.18.10+k3s2
Updating k3s to k3s (version: v1.19.3+k3s2, source: github)
Downloading package artifacts from github
Setting k3s (version: v1.18.10+k3s2, source: github) as the current package
Installed executables:
 - /home/davidko/.huber/bin/k3s
k3s (version: v1.18.10+k3s2, source: github) updated

```

## Updating the package

```console
❯ huber update velero
Updating velero (version: v1.4.3, source: github) to the latest version
Downloading package artifacts from github
Setting velero (version: v1.5.2, source: github) as the current package
Installed executables:
 - /home/davidko/.huber/bin/velero
velero updated

```

## Setting the current installed package version

```console
❯ huber show gh --all
 Name  Version  Current 
 gh    v1.2.0   false 
 gh    v1.1.0   true 

❯ huber current gh -v v1.2.0
Setting gh (version: v1.2.0, source: github) as the current package
Updated executables:
 - /home/davidko/.huber/bin/gh
gh (version: v1.2.0, source: github) as current updated

❯ huber show gh --all
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

❯ huber show velero
 Name    Version  Current  Executables 
 velero  v1.5.1   true     - /home/davidko/.huber/bin/velero 

❯ huber show velero --all
 Name    Version  Current  Executables 
 velero  v1.5.2   false     
 velero  v1.5.1   true     - /home/davidko/.huber/bin/velero

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
