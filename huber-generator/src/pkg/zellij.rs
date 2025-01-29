use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "zellij".to_string(),
        source: PackageSource::Github {
            owner: "zellij-org".to_string(),
            repo: "zellij".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "zellij-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "zellij-aarch64-unknown-linux-musl.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["zellij-aarch64-apple-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec!["zellij-aarch64-apple-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
