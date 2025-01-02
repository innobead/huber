use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "skim".to_string(),
        source: PackageSource::Github {
            owner: "lotabout".to_string(),
            repo: "skim".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "skim-v{version}-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "skim-v{version}-armv7-unknown-linux-gnueabihf.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["skim-v{version}-x86_64-apple-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
