use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "conftest".to_string(),
        source: PackageSource::Github {
            owner: "open-policy-agent".to_string(),
            repo: "conftest".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["conftest_{version}_Linux_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["conftest_{version}_Linux_arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["conftest_{version}_Darwin_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["conftest_{version}_Windows_x86_64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
