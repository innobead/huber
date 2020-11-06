# What is Huber?

**Huber** is a single development entry to simplify 'github release' package management. 

- Manages released artifacts from github, and decide which version as the current to use ✅
- Provides a managed package list of famous started github projects ✅
- Provides to manage self own package list for any development purpose *(soon)*
- 'huber.yaml' can be created in any public/private github repo to be aware to Huber to manage released artifacts *(soon)*   
- Supports cross platforms *(Linux ready, MacOS andWindows soon)* 

# Getting Started

## Installing Huber

```console
curl -sfSL https://raw.githubusercontent.com/innobead/kuber/master/hack/install-huber.sh | bash
```

## Install a package from github

```console

```

# Tutorials

## Installing package

```console

```

## Setting current version

```console

```

## Searching available packages

```console
❯ ./target/debug/huber search
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
❯ ./target/debug/huber search -n k3s --all
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

## Showing installed version info

## Uninstalling package

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

## Uninstall Huber

# Notes

- Huber is not product ready project, but active under development. Any feedback is welcome!