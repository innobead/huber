use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tokei".to_string(),
        source: PackageSource::Github {
            owner: "XAMPPRocky".to_string(),
            repo: "tokei".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["tokei-x86_64-unknown-linux-gnu.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["tokei-aarch64-unknown-linux-gnu.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["tokei-x86_64-apple-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["tokei-x86_64-pc-windows-msvc.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
