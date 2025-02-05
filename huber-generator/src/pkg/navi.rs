use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "navi".to_string(),
        source: PackageSource::Github {
            owner: "denisidoro".to_string(),
            repo: "navi".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "navi-v{version}-aarch64-unknown-linux-gnu.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm(PackageManagement {
                artifact_templates: vec![
                    "navi-v{version}-armv7-unknown-linux-musleabihf.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(Default::default()),
            PackageTargetType::MacOSArm64(Default::default()),
            PackageTargetType::WindowsAmd64(Default::default()),
        ],
        ..Default::default()
    }
}
