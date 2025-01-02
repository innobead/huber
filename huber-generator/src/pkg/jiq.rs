use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "jiq".to_string(),
        source: PackageSource::Github {
            owner: "fiatjaf".to_string(),
            repo: "jiq".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["jiq_linux_amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["jiq_darwin_amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["jiq_windows_amd64.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
