use huber_common::model::release::{Release, ReleaseManagement, ReleaseSource, ReleaseTargetType};

pub fn release() -> Release {
    Release {
        name: "velero".to_string(),
        version: "latest".to_string(),
        source: ReleaseSource::Github {
            owner: "vmware-tanzu".to_string(),
            repo: "velero".to_string(),
        },
        detail: None,
        targets: Some(vec![ReleaseTargetType::LinuxAmd64(ReleaseManagement {
            artifact_templates: Some(vec!["velero-{version}-linux-amd64.tar.gz".to_string()]),
            install_commands: None, // untar, walk through all executables, then install
            uninstall_commands: None,
            upgrade_commands: None,
        })]),
    }
}
