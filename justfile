prj_dir := justfile_directory()
build_cache_dir := prj_dir / '.cache'
build_dir := prj_dir / '.target'
generate_artifact_name := prj_dir / 'hack/generate-artifact-name.sh'
managed_pkg_root_dir := prj_dir / 'generated'
huber_exec := prj_dir / 'target/debug/huber'
cargo_opts := env('CARGO_OPTS', '')
github_token := env('GITHUB_TOKEN', '')
github_key := env('GITHUB_KEY', '')

# Build binaries
build cmd_opts='': fix fmt
    @cargo {{ cargo_opts }} build {{ cmd_opts }}

# Run tests
test:
    @cargo {{ cargo_opts }} test

# Format & Lint codes
fmt:
    @rustup component add rustfmt clippy
    @cargo install --git https://github.com/DevinR528/cargo-sort.git --tag v1.1.0 cargo-sort
    @cargo-sort --workspace
    @cargo {{ cargo_opts }} +nightly fmt --all
    @cargo clippy --fix --allow-dirty --allow-staged

# Fix code
fix:
    @cargo {{ cargo_opts }} fix --allow-dirty --allow-staged

# Release binaries
release:
    @just build '--release'
    @mkdir -p {{ build_dir }} && cp {{ prj_dir }}/target/release/huber {{ build_dir }}/`{{ generate_artifact_name }}`
    @just _checksum

# Generate checksum files for built executables
_checksum:
    @{{ prj_dir }}/hack/generate-checksum.sh {{ build_dir }}

# Clean build caches
clean:
    @cargo clean
    @rm -rf {{ build_cache_dir }} {{ build_dir }}

# Publish Huber to crates.io
publish:
    @cargo publish {{ cargo_opts }} --manifest-path={{ prj_dir }}/crates/common/Cargo.toml || true
    @sleep 10 && cargo publish {{ cargo_opts }} --manifest-path={{ prj_dir }}/crates/procmacro/Cargo.toml || true
    @sleep 10 && cargo publish {{ cargo_opts }} --manifest-path={{ prj_dir }}/crates/app/Cargo.toml || true

# (local dev) Generate managed package list
generate force_generate='false':
    @echo "! Must have GITHUB_TOKEN to automatically generate package description"
    @GITHUB_TOKEN={{ github_token }} FORCE_GENERATE={{ force_generate }} cargo build {{ cargo_opts }} -vv --package=huber-generator
    @GITHUB_KEY={{ github_key }} just build && (MANAGED_PKG_ROOT_DIR={{ managed_pkg_root_dir }} {{ huber_exec }} search | xargs -0 {{ prj_dir }}/hack/generate-huber-managed-packages.sh)

# (local dev) Build binaries for linux multiple architectures
build-multiarch platforms='linux/arm64':
    PLATFORMS={{ platforms }} BUILD_TARGET=debug JUST_TARGET="test build" {{ prj_dir }}/hack/build-multiarch.sh

# (local dev) Release binaries for linux multiple archite
release-multiarch platforms='linux/arm64':
    PLATFORMS={{ platforms }} BUILD_TARGET=release OUTPUT_DIR={{ build_cache_dir }} JUST_TARGET=release {{ prj_dir }}/hack/build-multiarch.sh
    mkdir -p {{ build_dir }} && cp {{ build_cache_dir }}/target/huber-* {{ build_dir }}/

# (local dev) Setup development environment
setup-dev:
    @{{ prj_dir }}/hack/setup-dev.sh

# (local dev) Install binaries
install:
    @cargo install {{ cargo_opts }} --path {{ prj_dir }}/crates/app/ --bins
    @mkdir -p ~/.huber/bin && cp ~/.cargo/bin/huber ~/.huber/bin && {{ prj_dir }}/hack/add-huber-bin-to-env.sh

# (local dev) Run commands using the built Huber with the local package generated folder
@run huber_cmd pkg_dir=managed_pkg_root_dir:
    MANAGED_PKG_ROOT_DIR={{ pkg_dir }} {{ huber_exec }} {{ huber_cmd }}

# (local dev) Run commands using the installed Huber with the local package generated folder
run-installed huber_cmd pkg_dir=managed_pkg_root_dir:
    @MANAGED_PKG_ROOT_DIR={{ pkg_dir }} `which huber` {{ huber_cmd }}
