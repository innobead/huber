use huber_common::model::package::{Package, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kotlin".to_string(),
        source: PackageSource::Github {
            owner: "JetBrains".to_string(),
            repo: "kotlin".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::MacOSAmd64(Default::default()),
            PackageTargetType::MacOSArm64(Default::default()),
            PackageTargetType::WindowsAmd64(Default::default()),
        ],
        ..Default::default()
    }
}
