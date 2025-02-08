use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k3s".to_string(),
        source: PackageSource::Github {
            owner: "k3s-io".to_string(),
            repo: "k3s".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["k3s".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["k3s-arm64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm(PackageManagement {
                artifact_templates: vec!["k3s-armhf".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
