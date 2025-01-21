use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "camel-k".to_string(),
        source: PackageSource::Github {
            owner: "apache".to_string(),
            repo: "camel-k".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["camel-k-client-{version}-linux-64bit.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["camel-k-client-{version}-mac-64bit.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "camel-k-client-{version}-windows-64bit.tar.gz".to_string()
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
