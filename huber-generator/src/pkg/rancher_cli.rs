use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "rancher-cli".to_string(),
        source: PackageSource::Github {
            owner: "rancher".to_string(),
            repo: "cli".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["rancher-linux-amd64-v{version}.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["rancher-darwin-amd64-v{version}.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["rancher-windows-amd64-v{version}.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
