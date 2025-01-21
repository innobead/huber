use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ketch".to_string(),
        source: PackageSource::Github {
            owner: "shipa-corp".to_string(),
            repo: "ketch".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/ketch-linux-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/ketch-darwin-amd64".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
