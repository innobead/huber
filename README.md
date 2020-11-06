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
make release

# Add huber home/bin in your environment path
export PATH=$HOME/.huber/bin:$PATH
```

# Tutorials

## Installing package

```console
❯ huber install k3s
Installing k3s
Downloading package artifacts from github
Setting k3s (version: v1.19.3+k3s2, source: github) as the current package
k3s (version: v1.19.3+k3s2, source: github) installed

```

## Setting current version

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

## Searching available packages

```console
❯ huber search
 Name      Source 
 gh        Github: 
             owner: cli 
             repo: cli 
 velero    Github: 
             owner: vmware-tanzu 
             repo: velero 
 kubefire  Github: 
             owner: innobead 
             repo: kubefire 
 k3s       Github: 
             owner: rancher 
             repo: k3s 
 rke       Github: 
             owner: rancher 
             repo: rke 
 rio       Github: 
             owner: rancher 
             repo: rio 

```

## Searching available package versions

```console
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

# Show package detailed info

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

## Showing installed version info

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

## Uninstalling package

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
    -l, --log-level <string>       Log level [default: info]  [possible values: off, error, warn, info, debug, trace]
    -o, --output <string>          Output format [default: console]  [possible values: console, json, yaml]

SUBCOMMANDS:
    current        Update current package version
    flush          Flush inactive artifacts (ex: remove non-current packages)
    help           Prints this message or the help of the given subcommand(s)
    info           Show package info
    install        Install package
    reset          Reset huber (ex: remove installed packages)
    search         Search package
    self-update    Update huber
    show           Show installed packages
    uninstall      Uninstall package

```

# Contribution

If you would like to add some useful tools in the builtin manged packages list, please have a PR as below steps.

1. Add a new package module in `src/generator/src/pkg`
2. Update the added package in `src/generator/src/build.rs`
3. `make build` to see if the new package manifest generated in `generated/packages` and `generated/index.yaml` updated accordingly
4. Fire a PR to make it accept

# Notes

- Huber is not product ready project, but active under development. Any feedback is welcome!