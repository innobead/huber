use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kube-linter".to_string(),
        source: PackageSource::Github {
            owner: "stackrox".to_string(),
            repo: "kube-linter".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["kube-linter-linux.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["kube-linter-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(Default::default()),
        ],
        ..Default::default()
    }
}
