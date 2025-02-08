use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

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
                    "linux_czkawka_cli".to_string(),
                    "linux_czkawka_gui".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "mac_czkawka_cli".to_string(),
                    "mac_czkawka_gui".to_string(),
                    "mac_krokiet_gui".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "windows_czkawka_cli.exe".to_string(),
                    "windows_krokiet_gui_linversion.exe".to_string(),
                    "windows_krokiet_gui_linversion_console.exe".to_string(),
                    "windows_krokiet_gui_winversion.exe".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
