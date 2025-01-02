# Add a New Package

We use `ollam` as an example to show how to add a new package to the generator.

## Step 1: Create a new package module in `./huber-generator/src/pkg`

You can specify the exact artifact name template or use the default automatic artifact name recognition.

```rust
#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ollama".to_string(),
        source: PackageSource::Github {
            owner: "ollama".to_string(),
            repo: "ollama".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["ollama-linux-amd64.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["ollama-linux-arm64.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["ollama-darwin".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["ollama-windows-amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
```

If the artifact name includes [`env::consts::OS`](https://doc.rust-lang.org/std/env/consts/constant.OS.html) and [`env::consts::ARCH`](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html) or
compatibles values defined in [GOOS/GOARCH](https://pkg.go.dev/internal/platform), you
can use the default automatic artifact name recognition instead of specifying the artifact name template.

```rust
#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ollama".to_string(),
        source: PackageSource::Github {
            owner: "ollama".to_string(),
            repo: "ollama".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::MacOSAmd64(Default::default()),
            PackageTargetType::WindowsAmd64(Default::default()),
        ],
        ..Default::default()
    }
}
```

## Step 2: Declare the package module in `./huber-generator/src/pkg/mod.rs`

```rust
pub mod ollama;
```

## Step 3: Export the package to the release function in `./huber-generator/src/build.rs`

```rust

fn releases() -> Vec<Package> {
    vec![
        // ... existing packages
        // Add the new package here
        ollama::release(),
    ]
}
```

## Step 4: Run the generator

After running the following command, the generator will automatically generate the package information to the `./generated-v1` directory and update the `./docs/packages.md` file.

```shell
just generate
```

Finally, please create a pull request to merge the changes into the main branch. Thank you for contributing to Huber!