use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "solidity".to_string(),
        source: PackageSource::Github {
            owner: "ethereum".to_string(),
            repo: "solidity".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["v{version}/solc-static-linux".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["v{version}/solc-macos".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["v{version}/solc-windows.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
