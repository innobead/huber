use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "wabt".to_string(),
        source: PackageSource::Github {
            owner: "WebAssembly".to_string(),
            repo: "wabt".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["wabt-{version}-ubuntu.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["wabt-{version}-macos-12.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec!["wabt-{version}-macos-14.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["wabt-{version}-windows.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
