# The `completions` Command

The `completions` command shows command completions for the specified shell. It is useful for setting up command completions for Huber.

```console
$ huber completions --help
Show command completions for the specified shell

Usage: huber completions [OPTIONS] <SHELL>

Arguments:
  <SHELL>  Shell name [possible values: bash, elvish, fish, powershell, zsh]

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