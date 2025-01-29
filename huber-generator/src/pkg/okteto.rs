use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "okteto".to_string(),
        source: PackageSource::Github {
            owner: "okteto".to_string(),
            repo: "okteto".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["okteto-Linux-x86_64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["okteto-Linux-arm64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["okteto-Darwin-x86_64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["okteto.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
