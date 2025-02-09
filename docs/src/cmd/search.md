# The `search` Command

The `search` command searches for a package by name or regex pattern.

```console
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
      --repo <REPO>
          Search in a specific repository
      --huber-dir <HUBER_DIR>
          Huber directory [default: /home/davidko/.huber]
      --github-base-uri <GITHUB_BASE_URI>
          GitHub base URI [env: GITHUB_BASE_URI=] [default: https://api.github.com]
  -h, --help
          Print help
```

# Examples

## Search packages by pattern

```console
$ huber search --pattern wasm
 Name         Description                                                                                              Source 
 rustwasmc    Tool for building Rust functions for Node.js. Combine the performance of Rust, safety and portabilit...  https://github.com/second-state/rustwasmc 
 wasm-to-oci  Use OCI registries to distribute Wasm modules                                                            https://github.com/engineerd/wasm-to-oci 
 wasme        Web Assembly tools and SDKs for extending cloud-native infrastructure                                    https://github.com/solo-io/wasm 
 wasmer       🚀 Fast, secure, lightweight containers based on WebAssembly                                             https://github.com/wasmerio/wasmer 
 wasmtime     A lightweight WebAssembly runtime that is fast, secure, and standards-compliant                          https://github.com/bytecodealliance/wasmtime 
```

## Search a package by name. Using the `--all` flag to show all the released versions.

```console
$ huber search wasmtime
 Name      Description                                                                      Source 
 wasmtime  A lightweight WebAssembly runtime that is fast, secure, and standards-compliant  https://github.com/bytecodealliance/wasmtime 
```

```console
$ huber search wasmtime --all
 Version  Kind 
 dev      PreRelease 
 v29.0.1  Release 
 v29.0.0  Release 
 v28.0.1  Release 
 v28.0.0  Release 
 v27.0.0  Release 
 v26.0.1  Release 
 v26.0.0  Release 
 v25.0.3  Release 
 v25.0.2  Release 
 v25.0.1  Release 
 v25.0.0  Release 
 v24.0.2  Release 
 v24.0.1  Release 
 v24.0.0  Release 
 ...
```