use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "sd".to_string(),
        source: PackageSource::Github {
            owner: "chmln".to_string(),
            repo: "sd".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["sd-v{version}-x86_64-unknown-linux-musl".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["sd-v{version}-x86_64-apple-darwin".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
