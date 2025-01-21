use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "natscli".to_string(),
        source: PackageSource::Github {
            owner: "nats-io".to_string(),
            repo: "natscli".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["nats-{version}-linux-amd64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["nats-{version}-linux-arm64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["nats-{version}-darwin-amd64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["nats-{version}-windows-amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
