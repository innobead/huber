use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hugo".to_string(),
        source: PackageSource::Github {
            owner: "gohugoio".to_string(),
            repo: "hugo".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "hugo_{version}_darwin-universal.tar.gz".to_string(),
                    "hugo_extended_{version}_darwin-universal.tar.gz".to_string(),
                    "hugo_extended_withdeploy_{version}_darwin-universal.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(Default::default()),
        ],
        ..Default::default()
    }
}
