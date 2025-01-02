use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tecli".to_string(),
        source: PackageSource::Github {
            owner: "awslabs".to_string(),
            repo: "tecli".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["tecli-linux-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["tecli-darwin-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["tecli-windows-amd64.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
