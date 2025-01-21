use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "coreutils".to_string(),
        source: PackageSource::Github {
            owner: "uutils".to_string(),
            repo: "coreutils".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "coreutils-{version}-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "coreutils-{version}-aarch64-unknown-linux-gnu.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "coreutils-{version}-x86_64-apple-darwin.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "coreutils-{version}-x86_64-pc-windows-msvc.zip".to_string()
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
