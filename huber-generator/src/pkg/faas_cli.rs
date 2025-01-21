use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};
use maplit::hashmap;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "faas-cli".to_string(),
        source: PackageSource::Github {
            owner: "openfaas".to_string(),
            repo: "faas-cli".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/faas-cli".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "faas-cli".to_string() => "faas".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["{version}/faas-cli-arm64".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "faas-cli".to_string() => "faas".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/faas-cli-darwin".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "faas-cli".to_string() => "faas".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}/faas-cli.exe".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "faas-cli".to_string() => "faas".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
        ],
        ..Default::default()
    }
}
