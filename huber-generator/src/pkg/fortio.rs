use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fortio".to_string(),
        source: PackageSource::Github {
            owner: "fortio".to_string(),
            repo: "fortio".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["fortio_win_{version}.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
