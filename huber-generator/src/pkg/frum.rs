use huber::model::package::{Package, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "frum".to_string(),
        source: PackageSource::Github {
            owner: "TaKO8Ki".to_string(),
            repo: "frum".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::MacOSAmd64(Default::default()),
        ],
        ..Default::default()
    }
}
