# The `lock` Command

The `lock` command allows you to lock packages or display locked packages. When a package is locked, it will not be updated by the `huber update` command if its version is equal to or lower than the locked version requirement.

The package locking uses Cargo's version requirement format.

```shell
the locked version.

```shell
$ huber lock --help
Lock packages or Show locked packages

Usage: huber lock [OPTIONS] [NAME_VERSION]... [COMMAND]

Commands:
  show  Show locked versions
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [NAME_VERSION]...  Package name (e.g. 'package-name', 'package-name@semver' or 'package-name@<semver-requirement>' using Cargo's dependency version requirement format)

Options:
      --all
          Lock all installed `current` packages
  -l, --log-level <LOG_LEVEL>
          Log level [default: OFF]
      --caret-required
          Treat version requirement as a caret requirement if no version requirement is specified
      --github-token <GITHUB_TOKEN>
          GitHub token; Optional until reaching the rate limit of GitHub API [env: GITHUB_TOKEN=]
      --github-key <GITHUB_KEY>
          Github SSH key path; Optional, if you want to use SSH to clone the Huber repository [env: GITHUB_KEY=]
      --tilde-required
          Treat version requirement as a tilde requirement if no version requirement is specified
      --huber-dir <HUBER_DIR>
          Huber directory [default: /home/davidko/.huber]
      --github-base-uri <GITHUB_BASE_URI>
          GitHub base URI [env: GITHUB_BASE_URI=] [default: https://api.github.com]
  -h, --help
          Print help
```

## The `lock show` Command

The `lock show` command shows locked packages and their locked version requirements.

```shell
$ huber lock show --help
Usage: huber lock show [OPTIONS]

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
```