use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kpt".to_string(),
        source: PackageSource::Github {
            owner: "GoogleContainerTools".to_string(),
            repo: "kpt".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["kpt_linux_amd64-{version}.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["kpt_darwin_amd64-{version}.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["kpt_windows_amd64-{version}.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
