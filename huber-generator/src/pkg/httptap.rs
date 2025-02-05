use huber_common::model::package::{Package, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "httptap".to_string(),
        source: PackageSource::Github {
            owner: "monasticacademy".to_string(),
            repo: "httptap".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
        ],
        ..Default::default()
    }
}
