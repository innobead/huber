use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "renote".to_string(),
        source: PackageSource::Github {
            owner: "innobead".to_string(),
            repo: "renote".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/renote-linux-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/renote-darwin-amd64".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
