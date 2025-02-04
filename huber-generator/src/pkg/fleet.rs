use huber_common::model::package::{Package, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fleet".to_string(),
        source: PackageSource::Github {
            owner: "rancher".to_string(),
            repo: "fleet".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::WindowsAmd64(Default::default()),
        ],
        ..Default::default()
    }
}
