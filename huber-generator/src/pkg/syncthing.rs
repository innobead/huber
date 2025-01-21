use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "syncthing".to_string(),
        source: PackageSource::Github {
            owner: "syncthing".to_string(),
            repo: "syncthing".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["syncthing-linux-amd64-v{version}.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["syncthing-linux-arm64-v{version}.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["syncthing-macos-amd64-v{version}.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["syncthing-windows-amd64-v{version}.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
