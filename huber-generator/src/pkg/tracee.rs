use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tracee".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "tracee".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["tracee-x86_64.v{version}.tar.gz".to_string()],
                scan_dirs: Some(vec!["dist".to_string()]),
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["tracee-aarch64.v{version}.tar.gz".to_string()],
                scan_dirs: Some(vec!["dist".to_string()]),
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
