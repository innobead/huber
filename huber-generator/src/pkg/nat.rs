use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "nat".to_string(),
        source: PackageSource::Github {
            owner: "willdoescode".to_string(),
            repo: "nat".to_string(),
        },
        detail: None,
        targets: vec![PackageTargetType::MacOSAmd64(PackageManagement {
            artifact_templates: vec!["{version}/natls_osx_binary".to_string()],
            executable_templates: None,
            executable_mappings: None,
            tag_version_regex_template: None,
            scan_dirs: None,
        })],
        version: None,
        description: None,
        release_kind: None,
    }
}
