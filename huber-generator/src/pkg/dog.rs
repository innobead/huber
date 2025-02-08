use huber::model::package::{Package, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dog".to_string(),
        source: PackageSource::Github {
            owner: "ogham".to_string(),
            repo: "dog".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::MacOSAmd64(Default::default()),
            PackageTargetType::WindowsAmd64(Default::default()),
        ],
        ..Default::default()
    }
}
