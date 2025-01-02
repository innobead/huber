use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dolt".to_string(),
        source: PackageSource::Github {
            owner: "dolthub".to_string(),
            repo: "dolt".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["dolt-linux-amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["dolt-darwin-amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["dolt-windows-amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
