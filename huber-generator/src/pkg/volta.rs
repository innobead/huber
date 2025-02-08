use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "volta".to_string(),
        source: PackageSource::Github {
            owner: "volta-cli".to_string(),
            repo: "volta".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["volta-{version}-linux.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["volta-{version}-macos.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["volta-{version}-windows.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
