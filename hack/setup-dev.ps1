function install_rust_dependencies {
  vcpkg install libarchive
  vcpkg integrate install

  if (!(Get-Command "cargo.exe" -ErrorAction SilentlyContinue))
  {
    Invoke-WebRequest -Uri "https://win.rustup.rs/" -OutFile "rustup-init.exe"
    .\rustup-init.exe
    cargo version
  }
}

install_rust_dependencies
