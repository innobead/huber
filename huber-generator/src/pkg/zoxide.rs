use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "zoxide".to_string(),
        source: PackageSource::Github {
            owner: "ajeetdsouza".to_string(),
            repo: "zoxide".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "zoxide-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["zoxide-x86_64-apple-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["zoxide-x86_64-pc-windows-msvc.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
