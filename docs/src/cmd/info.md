# The `info` Command

The `info` command shows package information.

```console
$ huber info --help
Shows package information

Usage: huber info [OPTIONS] <NAME>

Arguments:
  <NAME>  Package name

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

### Show package information

```console
$ huber info k9s
 Name  Version  Source             Targets 
 k9s   v0.32.7  Github:            - LinuxAmd64: 
                  owner: derailed      artifact_templates: 
                  repo: k9s            - k9s_Linux_amd64.tar.gz 
                                   - LinuxArm64: 
                                       artifact_templates: 
                                       - k9s_Linux_arm64.tar.gz 
                                   - MacOSAmd64: 
                                       artifact_templates: 
                                       - k9s_Darwin_amd64.tar.gz 
                                   - MacOSArm64: 
                                       artifact_templates: 
                                       - k9s_Darwin_arm64.tar.gz 
                                   - WindowsAmd64: 
                                       artifact_templates: 
                                       - k9s_Windows_amd64.zip 
```