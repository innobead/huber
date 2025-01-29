use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "epinio".to_string(),
        source: PackageSource::Github {
            owner: "epinio".to_string(),
            repo: "epinio".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["epinio-linux-amd64".to_string()],
                                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["epinio-linux-arm64".to_string()],
                                ..Default::default()
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["epinio-darwin-amd64".to_string()],
                                ..Default::default()
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec!["epinio-windows-amd64".to_string()],
                                ..Default::default()
            }),
        ],
                        ..Default::default()
    }
}
