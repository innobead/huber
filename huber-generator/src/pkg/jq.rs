use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};
use maplit::hashmap;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "jq".to_string(),
        source: PackageSource::Github {
            owner: "stedolan".to_string(),
            repo: "jq".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/jq-linux64".to_string()],
                executable_mappings: Some(hashmap! {
                    "jq-linux64".to_string() => "jq".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/jq-osx-amd64".to_string()],
                executable_mappings: Some(hashmap! {
                    "jq-osx-amd64".to_string() => "jq".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}/jq-win64.exe".to_string()],
                executable_mappings: Some(hashmap! {
                    "jq-win64.exe".to_string() => "jq.exe".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
        ],
        ..Default::default()
    }
}
