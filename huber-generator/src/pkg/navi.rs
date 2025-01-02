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
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "navi-v{version}-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["navi-v{version}-aarch64-linux-android.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["navi-v{version}-x86_64-apple-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec!["navi-v{version}-aarch64-apple-ios.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["navi-v{version}-x86_64-pc-windows-gnu.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
