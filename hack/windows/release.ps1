cargo build --release --workspace --exclude=huber-generator
Copy-Item ./target/release/huber.exe ./target/huber-windows-amd64.exe
./hack/generate-checksum.ps1 ./target/release