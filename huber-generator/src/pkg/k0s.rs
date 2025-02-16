use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k0s".to_string(),
        source: PackageSource::Github {
            owner: "k0sproject".to_string(),
            repo: "k0s".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["k0s-v{version}+k0s.0-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm(PackageManagement {
                artifact_templates: vec!["k0s-v{version}+k0s.0-arm".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["k0s-v{version}+k0s.0-arm64".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
