use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "frum".to_string(),
        source: PackageSource::Github {
            owner: "TaKO8Ki".to_string(),
            repo: "frum".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "frum-v{version}-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["frum-v{version}-x86_64-apple-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
