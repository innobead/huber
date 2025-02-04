use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fnm".to_string(),
        source: PackageSource::Github {
            owner: "Schniz".to_string(),
            repo: "fnm".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["fnm-linux.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["fnm-arm64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["fnm-macos.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["fnm-windows.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
