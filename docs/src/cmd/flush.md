# The `flush` Command

The `flush` command removes outdated installed artifacts to free up disk space.

```console
Remove outdated installed artifacts

Usage: huber flush [OPTIONS]

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

## Examples

### Flush outdated installed (non-current) artifacts

```console
$ huber show --all
 Name     Version  Current  Kind 
 k9s      v0.32.7  false    Release 
 k9s      v0.32.5  true     Release 

$ huber flush
[INFO ] Removing k9s (version: v0.32.7, source: github)
[INFO ] k9s (version: v0.32.7, source: github) removed

$ huber show --all
 Name     Version  Current  Kind 
 k9s      v0.32.5  true     Release 
```