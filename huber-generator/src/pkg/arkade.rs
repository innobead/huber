use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "arkade".to_string(),
        source: PackageSource::Github {
            owner: "alexellis".to_string(),
            repo: "arkade".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["arkade".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["arkade-arm64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm(PackageManagement {
                artifact_templates: vec!["arkade-armhf".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["arkade-darwin".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec!["arkade-darwin-arm64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["arkade.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
