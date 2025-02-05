# The `unlock` Command

The `unlock` command unlocks packages.

```console
$ huber unlock --help
Unlock packages

Usage: huber unlock [OPTIONS] <NAME>...

Arguments:
  <NAME>...  Package name

Options:
      --all
          Unlock all the locked packages
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

## Unlock all the locked packages

```console
$huber lock k9s
[INFO ] Locking packages
[INFO ] Locking package k9s@=0.32.5
[INFO ] Packages locked successfully: {
        "k9s": "0.32.5",
    }

$ huber unlock --all
[INFO ] Unlocking packages
[INFO ] Unlocking all packages
[INFO ] Unlocked packages

$ huber lock show
[INFO ] No packages locked
```
