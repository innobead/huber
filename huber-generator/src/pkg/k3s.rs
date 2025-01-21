use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k3s".to_string(),
        source: PackageSource::Github {
            owner: "rancher".to_string(),
            repo: "k3s".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/k3s".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["{version}/k3s-arm64".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
