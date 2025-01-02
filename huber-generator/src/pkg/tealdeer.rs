use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tealdeer".to_string(),
        source: PackageSource::Github {
            owner: "dbrgn".to_string(),
            repo: "tealdeer".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "tealdeer-linux-x86_64-musl".to_string(),
                    "tldr-linux-x86_64-musl".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["tealdeer-macos-x86_64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["tealdeer-windows-x86_64-msvc.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
