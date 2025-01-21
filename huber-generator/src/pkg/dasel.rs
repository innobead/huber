use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dasel".to_string(),
        source: PackageSource::Github {
            owner: "TomWright".to_string(),
            repo: "dasel".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/dasel_linux_amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/dasel_macos_amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}/dasel_windows_amd64.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
