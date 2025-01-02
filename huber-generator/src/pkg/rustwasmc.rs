use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "rustwasmc".to_string(),
        source: PackageSource::Github {
            owner: "second-state".to_string(),
            repo: "rustwasmc".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "rustwasmc-v{version}-x86_64-unknown-linux-gnu.tar.gz".to_string(),
                    "ssvmup-v{version}-x86_64-unknown-linux-gnu.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "rustwasmc-v{version}-aarch64-unknown-linux-gnu.tar.gz".to_string(),
                    "ssvmup-v{version}-aarch64-unknown-linux-gnu.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "rustwasmc-v{version}-x86_64-apple-darwin.tar.gz".to_string(),
                    "ssvmup-v{version}-x86_64-apple-darwin.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "rustwasmc-v{version}-x86_64-pc-windows-msvc.tar.gz".to_string(),
                    "ssvmup-v{version}-x86_64-pc-windows-msvc.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
