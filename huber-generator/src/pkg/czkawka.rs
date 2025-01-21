use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "czkawka".to_string(),
        source: PackageSource::Github {
            owner: "qarmin".to_string(),
            repo: "czkawka".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "{version}/linux_czkawka_cli".to_string(),
                    "{version}/linux_czkawka_gui".to_string(),
                    "{version}/linux_czkawka_gui.AppImage".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/mac_czkawka_cli".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}/windows_czkawka_cli.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
