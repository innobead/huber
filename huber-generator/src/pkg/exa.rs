use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "exa".to_string(),
        source: PackageSource::Github {
            owner: "ogham".to_string(),
            repo: "exa".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["exa-linux-x86_64-{version}.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["exa-macos-x86_64-{version}.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
