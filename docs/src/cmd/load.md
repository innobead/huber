# The `load` Command

The `load` command reads the package list from a file generated by the [save](./save.md) command and installs the packages.

```console
$ huber load --help
Load installed packages from a file generated by save command

Usage: huber load [OPTIONS]

Options:
      --file <FILE>
          Load a package list to install [default: huber-packages.txt]
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
```

## Examples

### Load installed packages from a file

```console
$ huber load --file output.txt
[INFO ] Loading packages from output.txt
[INFO ] Loaded packages: total 1: [
        "k9s@v0.32.7",
    ]
[INFO ] Installing packages: total 1
[INFO ] Installing package k9s@v0.32.7
[INFO ] Installed executables of k9s:
    [
        "/home/davidko/.huber/bin/k9s",
    ]
[INFO ] k9s@v0.32.7 installed
[INFO ] Installed packages: total 1
```

