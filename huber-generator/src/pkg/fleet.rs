use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fleet".to_string(),
        source: PackageSource::Github {
            owner: "rancher".to_string(),
            repo: "fleet".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["fleet-linux-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["fleet-linux-arm64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["fleet-darwin-arm64".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
