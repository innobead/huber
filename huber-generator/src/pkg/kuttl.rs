use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kuttl".to_string(),
        source: PackageSource::Github {
            owner: "kudobuilder".to_string(),
            repo: "kuttl".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::LinuxArm(PackageManagement {
                artifact_templates: vec![
                    "kubectl-kuttl_{version}_linux_armv6".to_string(),
                    "kuttl_{version}_linux_armv6.tar.gz
"
                    .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(Default::default()),
            PackageTargetType::MacOSArm64(Default::default()),
        ],
        ..Default::default()
    }
}
