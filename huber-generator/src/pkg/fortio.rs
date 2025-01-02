use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fortio".to_string(),
        source: PackageSource::Github {
            owner: "fortio".to_string(),
            repo: "fortio".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["fortio-linux_x64-{version}.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["fortio-linux_x64-{version}.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["fortio_win_{version}.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
