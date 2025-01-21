use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "shisho".to_string(),
        source: PackageSource::Github {
            owner: "flatt-security".to_string(),
            repo: "shisho".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/build-x86_64-unknown-linux-gnu.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/build-x86_64-apple-darwin.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}/build-x86_64-pc-windows.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
