use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "gitui".to_string(),
        source: PackageSource::Github {
            owner: "extrawurst".to_string(),
            repo: "gitui".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["gitui-mac.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["gitui-win.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
