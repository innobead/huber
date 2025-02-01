# The `search` Command

The `search` command searches for a package by name or regex pattern.

```shell
$ huber search --help
Search package

Usage: huber search [OPTIONS] [NAME]

Arguments:
  [NAME]  Package name or regex search with --pattern

Options:
  -l, --log-level <LOG_LEVEL>
          Log level [default: OFF]
      --pattern
          Regex search
      --github-token <GITHUB_TOKEN>
          GitHub token; Optional until reaching the rate limit of GitHub API [env: GITHUB_TOKEN=]
      --owner <OWNER>
          Package owner
      --all
          Show all the released versions
      --github-key <GITHUB_KEY>
          Github SSH key path; Optional, if you want to use SSH to clone the Huber repository [env: GITHUB_KEY=]
      --huber-dir <HUBER_DIR>
          Huber directory [default: /home/davidko/.huber]
      --github-base-uri <GITHUB_BASE_URI>
          GitHub base URI [env: GITHUB_BASE_URI=] [default: https://api.github.com]
  -h, --help
          Print help
```