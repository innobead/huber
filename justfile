prj_dir := justfile_directory()
build_cache_dir := prj_dir / '.cache'
build_dir := prj_dir / '.target'

huber_artifact_script := prj_dir / 'hack/huber-artifact-name.sh'
managed_pkg_root_dir := prj_dir / 'generated'
huber_exec := prj_dir / 'target/debug/huber'

cargo_opts := env('CARGO_OPTS', '')
github_token := env('GITHUB_TOKEN', '')
github_key := env('GITHUB_KEY', '')

# Build binaries
build cmd_opts='': fix fmt
    cargo {{ cargo_opts }} build {{ cmd_opts }}

# Run tests
test:
    @cargo {{ cargo_opts }} test

# Format & Lint codes
fmt:
    @rustup component add rustfmt clippy
    @cargo {{ cargo_opts }} fmt

# Fix code
fix:
    @cargo {{ cargo_opts }} fix --allow-dirty --allow-staged

# Release binaries
release:
    @just build '--release'
    @mkdir -p {{ build_dir }} && cp {{ prj_dir }}/target/release/huber {{ build_dir }}/`{{ huber_artifact_script }}`
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
    @GITHUB_TOKEN={{ github_token }} cargo build {{ cargo_opts }} -vv --package=huber-generator
    @GITHUB_KEY={{ github_key }} just build && (MANAGED_PKG_ROOT_DIR={{ managed_pkg_root_dir }} FORCE_GENERATE={{ force_generate }} {{ huber_exec }} search | xargs -0 {{ prj_dir }}/hack/generate-packages.md.sh)

# (local dev) Build binaries for linux multiple architectures
build-multiarch platforms='linux/arm64':
    PLATFORMS={{ platforms }} BUILD_TARGET=debug MAKE_TARGET="test build" {{ prj_dir }}/hack/build-multiarch.sh

# (local dev) Release binaries for linux multiple archite
release-multiarch platforms='linux/arm64':
    PLATFORMS={{ platforms }} BUILD_TARGET=release OUTPUT_DIR={{ build_cache_dir }} MAKE_TARGET=release {{ prj_dir }}/hack/build-multiarch.sh
    mkdir -p {{ build_dir }} && cp {{ build_cache_dir }}/target/huber-* {{ build_dir }}/
    just checksum

# (local dev) Setup development environment
setup-dev:
    @{{ prj_dir }}/hack/setup-dev.sh

# (local dev) Install binaries
install:
    @cargo install {{ cargo_opts }} --path {{ prj_dir }}/crates/app/ --bins
    @mkdir -p ~/.huber/bin && cp ~/.cargo/bin/huber ~/.huber/bin && {{ prj_dir }}/hack/update-env.sh

# (local dev) Verify Huber commands via the local package generated folder
verify huber_cmd pkg_dir=managed_pkg_root_dir:
    MANAGED_PKG_ROOT_DIR={{ managed_pkg_root_dir }} {{ huber_exec }} {{ huber_cmd }}

# (local dev) Verify the installed Huber commands compatible with the new local package generated folder
verify-compatible huber_cmd pkg_dir=managed_pkg_root_dir:
    MANAGED_PKG_ROOT_DIR={{ managed_pkg_root_dir }} `which huber` {{ huber_cmd }}
