use huber_common::model::package::{Package, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "jless".to_string(),
        source: PackageSource::Github {
            owner: "PaulJuliusMartinez".to_string(),
            repo: "jless".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::MacOSAmd64(Default::default()),
            PackageTargetType::MacOSArm64(Default::default()),
        ],
        ..Default::default()
    }
}
