use huber::model::package::{Package, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kubefire".to_string(),
        source: PackageSource::Github {
            owner: "innobead".to_string(),
            repo: "kubefire".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
        ],
        ..Default::default()
    }
}
