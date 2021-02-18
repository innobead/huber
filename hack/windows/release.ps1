cargo build --release --workspace --exclude=huber-generator
Rename-Item ./target/release/huber.exe huber-windows-amd64.exe
./hack/windows/generate-checksum.ps1 ./target/release