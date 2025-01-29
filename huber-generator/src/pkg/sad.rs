use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "sad".to_string(),
        source: PackageSource::Github {
            owner: "ms-jpq".to_string(),
            repo: "sad".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["x86_64-unknown-linux-gnu.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["x86_64-apple-darwin.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
