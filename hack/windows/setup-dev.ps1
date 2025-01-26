function install_rust_dependencies {
  vcpkg integrate install

  if (!(Get-Command "cargo.exe" -ErrorAction SilentlyContinue))
  {
    Invoke-WebRequest -Uri "https://win.rustup.rs/" -OutFile "rustup-init.exe"
    .\rustup-init.exe
    cargo version
  }

  rustup component add rustfmt clippy
  cargo install default-target
  cargo install --git https://github.com/DevinR528/cargo-sort.git --tag v1.1.0 cargo-sort
  cargo install cargo-udeps
}

install_rust_dependencies
