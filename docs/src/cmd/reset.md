# The `reset` Command

The `reset` command resets Huber to its initial state.

```console
$ huber reset --help
Reset Huber

Usage: huber reset [OPTIONS]

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

## Reset Huber

```console
$ huber install k9s
[INFO ] k9s version not specified, getting the latest version (v0.32.7)
[INFO ] Installing package k9s@latest/v0.32.7
[INFO ] Installed executables of k9s:
    [
        "/home/davidko/.huber/bin/k9s",
    ]
[INFO ] k9s@latest/v0.32.7 installed
 
$ huber reset
[INFO ] Resetting Huber by removing created caches, downloaded files and installed packages
[INFO ] Huber reset

$ huber show
[INFO ] No packages installed
```
