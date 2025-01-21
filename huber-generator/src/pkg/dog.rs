use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dog".to_string(),
        source: PackageSource::Github {
            owner: "ogham".to_string(),
            repo: "dog".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["dog-v{version}-x86_64-unknown-linux-gnu.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["dog-v{version}-x86_64-apple-darwin.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["dog-v{version}-x86_64-pc-windows-msvc.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
