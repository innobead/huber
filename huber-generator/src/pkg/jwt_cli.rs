use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "jwt-cli".to_string(),
        source: PackageSource::Github {
            owner: "mike-engel".to_string(),
            repo: "jwt-cli".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["jwt-linux.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["jwt-macOS.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["jwt-windows.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
