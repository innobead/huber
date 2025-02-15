use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "firecracker".to_string(),
        source: PackageSource::Github {
            owner: "firecracker-microvm".to_string(),
            repo: "firecracker".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["firecracker-v{version}-x86_64.tgz".to_string()],
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: Some(vec!["release-v{version}-{arch}".to_string()]),
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["firecracker-v{version}-aarch64.tgz".to_string()],
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: Some(vec!["release-v{version}-{arch}".to_string()]),
            }),
        ],
        ..Default::default()
    }
}
