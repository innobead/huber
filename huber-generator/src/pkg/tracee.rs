use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tracee".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "tracee".to_string(),
        },

        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["{version}/tracee.tar.gz".to_string()],
            executable_mappings: None,
            tag_version_regex_template: None,
            scan_dirs: Some(vec!["dist".to_string()]),
        })],
        ..Default::default()
    }
}
