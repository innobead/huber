# The `update` Command

The `update` command updates the installed packages.

```console
$ huber update --help
Updates the installed packages

Usage: huber update [OPTIONS] [NAME]...

Arguments:
  [NAME]...  Package name

Options:
  -l, --log-level <LOG_LEVEL>
          Log level [default: OFF]
      --prefer-stdlib <PREFER_STDLIB>
          Prefer standard library (only for Linux or Windows) [possible values: gnu, musl, msvc]
      --dryrun
          Dry run to show available updates
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
```

## Examples

### Update the installed packages

```console
$ huber install k9s@v0.32.5 kubectl@v1.31.0
[INFO ] Installing package k9s@v0.32.5
[INFO ] Installing package kubectl@v1.31.0
[INFO ] Installed executables of k9s:
    [
        "/home/davidko/.huber/bin/k9s",
    ]
[INFO ] k9s@v0.32.5 installed
[INFO ] Installed executables of kubectl:
    [
        "/home/davidko/.huber/bin/kubectl",
    ]
[INFO ] kubectl@v1.31.0 installed

$ huber update
[INFO ] Checking for updates for k9s. The latest installed version is v0.32.5
[INFO ] Checking for updates for kubectl. The latest installed version is v1.31.0
[INFO ] Found the latest version of kubectl: v1.32.1
[INFO ] Updating package kubectl from v1.31.0 to v1.32.1
[INFO ] Updating kubectl (version: v1.31.0, source: github) to kubectl (version: v1.32.1, source: github)
[INFO ] Found the latest version of k9s: v0.32.7
[INFO ] Updating package k9s from v0.32.5 to v0.32.7
[INFO ] Updating k9s (version: v0.32.5, source: github) to k9s (version: v0.32.7, source: github)
[INFO ] Installed executables of kubectl:
    [
        "/home/davidko/.huber/bin/kubectl",
    ]
[INFO ] Package kubectl updated to v1.32.1 successfully
[INFO ] Installed executables of k9s:
    [
        "/home/davidko/.huber/bin/k9s",
    ]
[INFO ] Package k9s updated to v0.32.7 successfully

```

### Update an unmanaged package directly from a repository

```console
$ huber install rancher/rke@v1.7.0
[INFO ] Installing package rancher/rke@v1.7.0
[INFO ] Prefer downloading assets belonging to the specified stdlib: gnu
[INFO ] Downloading https://github.com/rancher/rke/releases/download/v1.7.0/rke_linux-amd64
[INFO ] Installed executables of rancher/rke:
    [
        "/home/davidko/.huber/bin/rke",
    ]
[INFO ] rancher/rke@v1.7.0 installed

$ huber update rancher/rke
[INFO ] Checking for updates for rancher/rke. The latest installed version is v1.7.0
[INFO ] Found the latest version of rancher/rke: v1.7.2
[INFO ] Updating package rancher/rke from v1.7.0 to v1.7.2
[INFO ] Updating rancher/rke (version: v1.7.0, source: github) to rancher/rke (version: v1.7.2, source: github)
[INFO ] Prefer downloading assets belonging to the specified stdlib: gnu
[INFO ] Downloading https://github.com/rancher/rke/releases/download/v1.7.2/rke_linux-amd64
[INFO ] Installed executables of rancher/rke:
    [
        "/home/davidko/.huber/bin/rke",
    ]
[INFO ] Package rancher/rke updated to v1.7.2 successfully
```