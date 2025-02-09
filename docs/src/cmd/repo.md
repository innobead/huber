# The `repo` Command

The `repo` command allows you to manage repositories. Huber includes two types of repositories:

- Huber managed repository, which provides a curated list of packages
- Local repository, which supports installing packages directly from remote GitHub repositories

In addition to these built-in repositories, you can also add your own repositories to install packages from them.

```console
$ huber repo --help
Manage repositories

Usage: huber repo [OPTIONS] <COMMAND>

Commands:
  add     Add a new repo
  remove  Remove a repo
  show    Show all repos
  help    Print this message or the help of the given subcommand(s)

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

# The `repo add` Command

The `repo add` command adds a new repository using the URL or file path of the Huber package index file.

```console
$ huber repo add --help
Add a new repository

Usage: huber repo add [OPTIONS] --url <URL> --file <FILE> <NAME>

Arguments:
  <NAME>  Repo name

Options:
  -l, --log-level <LOG_LEVEL>
          Log level [default: OFF]
      --url <URL>
          URL of the Huber package index file
      --file <FILE>
          File path of the Huber package index file
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

The index file is a YAML file that contains the list of packages as the below example:

```yaml
- name: aichat
  description: All-in-one LLM CLI tool featuring Shell Assistant, Chat-REPL, RAG, AI Tools & Agents, with access to OpenAI, Claude, Gemini, Ollama, Groq, and more.
  source: !Github
    owner: sigoden
    repo: aichat
  targets:
  - !LinuxAmd64
    artifact_templates:
    - 'aichat-v{version}-x86_64-unknown-linux-musl.tar.gz'

```

# The `repo remove` Command

The `repo remove` command removes a repository.

```console
Remove a repository

Usage: huber repo remove [OPTIONS] [NAME]...

Arguments:
  [NAME]...  Repo names

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

# The `repo show` Command

The `repo show` command shows all repositories.

```console
$ huber repo show --help
Show all repositories

Usage: huber repo show [OPTIONS]

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

### Add a new repository via a file

```console
$ huber repo add --url https://raw.githubusercontent.com/innobead/huber/refs/heads/main/docs/src/cmd/repo/huber.yaml self-hosted
[INFO ] Adding repo self-hosted
[INFO ] Repo self-hosted added

$ huber repo show
 Name         Url                                                                                            File 
 local                                                                                                        
 self-hosted  https://raw.githubusercontent.com/innobead/huber/refs/heads/main/docs/src/cmd/repo/huber.yaml   
 
$ huber search aichat --repo self-hosted
 Name    Description                                                                                              Source 
 aichat  All-in-one LLM CLI tool featuring Shell Assistant, Chat-REPL, RAG, AI Tools & Agents, with access to...  https://github.com/sigoden/aichat 
```

### Add a package to the local repository if the package is not available in the Huber managed repository

```console
$ huber search --repo local
[INFO ] No packages found

$ huber search rancher/rke
[INFO ] No packages found

$ huber install rancher/rke
[INFO ] rancher/rke version not specified, getting the latest version (v1.7.2)
[INFO ] Installing package rancher/rke@latest/v1.7.2
[INFO ] Prefer downloading assets belonging to the specified stdlib: gnu
[INFO ] Downloading https://github.com/rancher/rke/releases/download/v1.7.2/rke_linux-amd64
[INFO ] Installed executables of rancher/rke:
    [
        "/home/davidko/.huber/bin/rke",
    ]
[INFO ] rancher/rke@latest/v1.7.2 installed

$ huber search --repo local
 Name         Description  Source 
 rancher/rke               https://github.com/rancher/rke 
```