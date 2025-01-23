use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "xh".to_string(),
        source: PackageSource::Github {
            owner: "ducaale".to_string(),
            repo: "xh".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "xh-v{version}-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["xh-v{version}-x86_64-apple-darwin.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["xh-v{version}-x86_64-pc-windows-msvc.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
