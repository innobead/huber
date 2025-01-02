use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "terrascan".to_string(),
        source: PackageSource::Github {
            owner: "tenable".to_string(),
            repo: "terrascan".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["terrascan_{version}_Linux_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["terrascan_{version}_Linux_arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["terrascan_{version}_Darwin_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["terrascan_{version}_Windows_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
