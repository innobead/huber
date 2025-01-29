use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "huber".to_string(),
        source: PackageSource::Github {
            owner: "innobead".to_string(),
            repo: "huber".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "huber-x86_64-unknown-linux-gnu".to_string(),
                    "huber-x86_64-unknown-linux-musl".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "huber-aarch64-unknown-linux-gnu".to_string(),
                    "huber-aarch64-unknown-linux-musl".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm(PackageManagement {
                artifact_templates: vec!["huber-arm-unknown-linux-gnueabihf".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["huber-x86_64-apple-darwin".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec!["huber-aarch64-apple-darwin".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "huber-x86_64-pc-windows-gnu".to_string(),
                    "huber-x86_64-pc-windows-msvc".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
