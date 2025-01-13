use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "nerdctl".to_string(),
        source: PackageSource::Github {
            owner: "containerd".to_string(),
            repo: "nerdctl".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["nerdctl-{version}-linux-amd64.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["nerdctl-{version}-linux-arm64.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}
