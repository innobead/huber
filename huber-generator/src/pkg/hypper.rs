use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hypper".to_string(),
        source: PackageSource::Github {
            owner: "rancher-sandbox".to_string(),
            repo: "hypper".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["hypper-v0.2.0-Linux-x86_64.tar.gz".to_string()],
                                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["hypper-v0.2.0-Linux-arm64.tar.gz".to_string()],
                                ..Default::default()
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["hypper-v0.2.0-Darwin-x86_64.tar.gz".to_string()],
                                ..Default::default()
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec!["hypper-v0.2.0-Windows-x86_64.tar.gz".to_string()],
                                ..Default::default()
            }),
        ],
                        ..Default::default()
    }
}
