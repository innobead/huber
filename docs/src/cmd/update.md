# The `update` Command

The `update` command updates the installed packages.

```shell
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