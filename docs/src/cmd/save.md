# The `save` Command

The `save` command saves the installed package list to a file which can be used by the [load](./load.md) command.

```shell
$ huber save --help
Save the installed package list to a file

Usage: huber save [OPTIONS]

Options:
      --file <FILE>
          File path to save the installed package list [default: huber-packages.txt]
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

## Save the installed package list

```shell
$ huber save --file output.txt
[INFO ] Collecting installed current packages
[INFO ] Saving the package list to output.txt
[INFO ] Saved the package list to /home/davidko/github/innobead/huber/output.txt

$ cat output.txt
k9s@v0.32.7
```
