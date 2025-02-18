prj_dir := justfile_directory()
build_dir := prj_dir / '.target'
huber_pkg_root_dir := prj_dir / 'generated-v1'
huber_exec := prj_dir / 'target/debug/huber'
cargo_opts := env('CARGO_OPTS', '')
github_token := env('GITHUB_TOKEN', '')
github_key := env('GITHUB_KEY', '')

# Install Rust componetns and tooling dependencies
build-deps:
    rustup component add rustfmt clippy
    cargo install cross
    cargo install default-target
    cargo install --git https://github.com/DevinR528/cargo-sort.git --tag v1.1.0 cargo-sort
    cargo install cargo-udeps
    cargo install mdbook mdbook-linkcheck mdbook-alerts mdbook-theme

# Build binaries
build target='' cmd_opts='':
    @rustup target add {{ if target != "" { target } else { shell("default-target") } }}
    @cargo {{ cargo_opts }} build {{ cmd_opts }} {{ if target != "" { "--target " + target } else { "" } }}

# Build binaries via Cross
build-cross target="" cmd_opts='':
    @cross {{ cargo_opts }} build {{ cmd_opts }} --target {{ if target != "" { target } else { shell("default-target") } }}

# Run tests
test:
    @cargo {{ cargo_opts }} test

# Format & Fix codes
ffix:
    @cargo-sort --workspace
    @cargo {{ cargo_opts }} +nightly fmt --all
    @cargo clippy --fix --no-deps --allow-dirty --allow-staged
    @cargo {{ cargo_opts }} fix --allow-dirty --allow-staged

# Find unused dependencies
udeps:
    @cargo +nightly udeps --all-targets

# Release binaries
release:
    @just build '' '--release'

# Clean build caches
clean:
    @cargo clean
    @rm -rf {{ build_dir }}

# Publish Huber to crates.io
publish:
    @cargo publish {{ cargo_opts }} --manifest-path={{ prj_dir }}/crates/common/Cargo.toml || true
    @sleep 10 && cargo publish {{ cargo_opts }} --manifest-path={{ prj_dir }}/crates/procmacro/Cargo.toml || true
    @sleep 10 && cargo publish {{ cargo_opts }} --manifest-path={{ prj_dir }}/crates/app/Cargo.toml || true

# (local dev) Setup development environment
setup-dev:
    @{{ prj_dir }}/hack/setup-dev.sh

# (local dev) Generate managed package list
generate force='true':
    @echo "! Must have GITHUB_TOKEN to automatically generate package description"
    @GITHUB_TOKEN={{ github_token }} FORCE={{ force }} cargo build {{ cargo_opts }} -vv --package=huber-generator
    @GITHUB_KEY={{ github_key }} just build && (HUBER_PKG_ROOT_DIR={{ huber_pkg_root_dir }} {{ huber_exec }} search | xargs -0 {{ prj_dir }}/hack/generate-huber-packages.sh)

# (local dev) Install binaries
install:
    @cargo install {{ cargo_opts }} --path {{ prj_dir }}/crates/app/ --bins
    @mkdir -p ~/.huber/bin && cp ~/.cargo/bin/huber ~/.huber/bin && {{ prj_dir }}/hack/add-huber-bin-to-env.sh

# (local dev) Run commands using the built Huber with the local package generated folder
run huber_cmd pkg_dir=huber_pkg_root_dir:
    HUBER_PKG_ROOT_DIR={{ pkg_dir }} {{ huber_exec }} {{ huber_cmd }}

# (local dev) Run commands using the installed Huber with the local package generated folder
run-installed huber_cmd pkg_dir=huber_pkg_root_dir:
    HUBER_PKG_ROOT_DIR={{ pkg_dir }} `which huber` {{ huber_cmd }}

doc:
    @mdbook build docs
