use huber::model::package::{Package, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "local-ai".to_string(),
        source: PackageSource::Github {
            owner: "mudler".to_string(),
            repo: "LocalAI".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::MacOSAmd64(Default::default()),
            PackageTargetType::MacOSArm64(Default::default()),
        ],
        ..Default::default()
    }
}
