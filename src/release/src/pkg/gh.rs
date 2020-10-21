use huber_common::model::release::{Release, ReleaseType, ReleaseTargetType, ReleaseManagement};

pub fn release() -> Release {
    Release {
        name: "gh".to_string(),
        version: "latest".to_string(),
        type_: ReleaseType::Github {
            owner: "cli".to_string(),
            repo: "cli".to_string(),
        },
        detail: None,
        targets: Some(vec![
            ReleaseTargetType::LinuxAmd64Ubuntu(ReleaseManagement {
                artifact_template: "".to_string(),
                install_commands: vec![
                    "sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-key C99B11DEB97541F0".to_string(),
                    "sudo apt-add-repository https://cli.github.com/packages".to_string(),
                    "sudo apt update".to_string(),
                    "sudo apt install gh".to_string(),
                ],
                uninstall_commands: vec![
                    "sudo apt uninstall gh".to_string(),
                ],
                upgrade_commands: vec![
                    "sudo apt update".to_string(),
                    "sudo apt install gh".to_string(),
                ],
            }),
            ReleaseTargetType::LinuxAmd64CentOs(ReleaseManagement {
                artifact_template: "".to_string(),
                install_commands: vec![
                    "sudo dnf config-manager --add-repo https://cli.github.com/packages/rpm/gh-cli.repo".to_string(),
                    "sudo dnf install gh".to_string(),
                ],
                uninstall_commands: vec![
                    "sudo dnf remove gh".to_string(),
                ],
                upgrade_commands: vec![
                    "sudo dnf update gh".to_string(),
                ],
            })
            ,
            ReleaseTargetType::LinuxAmd64OpenSuse(ReleaseManagement {
                artifact_template: "".to_string(),
                install_commands: vec![
                    "sudo zypper addrepo https://cli.github.com/packages/rpm/gh-cli.repo".to_string(),
                    "sudo zypper ref".to_string(),
                    "sudo zypper install gh".to_string(),
                ],
                uninstall_commands: vec![
                    "sudo zypper rm gh".to_string(),
                ],
                upgrade_commands: vec![
                    "sudo zypper ref".to_string(),
                    "sudo zypper update gh".to_string(),
                ],
            }),
            ReleaseTargetType::MacOS(ReleaseManagement {
                artifact_template: "".to_string(),
                install_commands: vec![
                    "brew install gh".to_string(),
                ],
                uninstall_commands: vec![
                    "brew uninstall gh".to_string(),
                ],
                upgrade_commands: vec![
                    "brew install gh".to_string(), ],
            })
        ]),
    }
}