# The `show` Command

The `show` command shows installed packages.

```console
$ huber show --help
Show installed packages

Usage: huber show [OPTIONS] [NAME]

Arguments:
  [NAME]  Package name

Options:
      --all
          Show all the installed versions
  -l, --log-level <LOG_LEVEL>
          Log level [default: OFF]
      --detail
          Show the detailed artifact info
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

### Show installed packages

```console
```console
$ huber show --all
 Name     Version  Current  Kind 
 k9s      v0.32.7  true     Release 
 k9s      v0.32.5  false    Release 
```
