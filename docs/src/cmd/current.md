# The `current` Command

The `current` command updates the current package versions if there are multiple versions of the same package installed.

```shell
$ huber current --help
Update the current package versions

Usage: huber current [OPTIONS] <NAME_VERSION>...

Arguments:
  <NAME_VERSION>...  Package name with version (e.g. 'package-name@version')

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