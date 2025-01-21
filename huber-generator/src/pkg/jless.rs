use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "jless".to_string(),
        source: PackageSource::Github {
            owner: "PaulJuliusMartinez".to_string(),
            repo: "jless".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "jless-v{version}-x86_64-unknown-linux-gnu.zip".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["jless-v{version}-x86_64-apple-darwin.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
