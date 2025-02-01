# The `self-update` Command

The `self-update` command updates Huber if a new version is available.

```shell
$ huber self-update --help
Update huber

Usage: huber self-update [OPTIONS]

Options:
  -l, --log-level <LOG_LEVEL>
          Log level [default: OFF]
      --prefer-stdlib <PREFER_STDLIB>
          Prefer standard library (only for Linux or Windows) [possible values: gnu, musl, msvc]
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