use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "pueue".to_string(),
        source: PackageSource::Github {
            owner: "Nukesor".to_string(),
            repo: "pueue".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "pueue-linux-x86_64".to_string(),
                    "pueued-linux-x86_64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "pueue-linux-aarch64".to_string(),
                    "pueued-linux-aarch64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "pueue-macos-x86_64".to_string(),
                    "pueued-macos-x86_64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "pueue-windows-x86_64.exe".to_string(),
                    "pueued-windows-x86_64.exe".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
