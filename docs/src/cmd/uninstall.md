# The `uninstall` Command

The `uninstall` command uninstalls packages.

```console
$ huber uninstall --help
Uninstall packages

Usage: huber uninstall [OPTIONS] [NAME]...

Arguments:
  [NAME]...  Package name

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

# Examples

## Uninstall multiple packages

```console
$ huber uninstall k9s kubectl
[INFO ] Uninstalling k9s
[INFO ] Uninstalled k9s
[INFO ] Uninstalling kubectl
[INFO ] Uninstalled kubectl
```

## Uninstall an unmanaged package directly from a repository

```console
$ huber uninstall rancher/rke
[INFO ] Uninstalling rancher/rke
[INFO ] Uninstalled rancher/rke
```