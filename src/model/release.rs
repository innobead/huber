use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ReleaseType {
    Github {
        owner: String,
        repo: String,
        url: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ActionType {
    Shell { exec: String, args: Vec<String> },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ReleaseDetailType {
    // Github { release: GithubRelease }
    Github,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Action {
    pub(crate) install: ActionType,
    pub(crate) uninstall: ActionType,
    pub(crate) pre_install: Option<ActionType>,
    pub(crate) post_install: Option<ActionType>,
    pub(crate) pre_uninstall: Option<ActionType>,
    pub(crate) post_uninstall: Option<ActionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Release {
    pub(crate) type_: ReleaseType,
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) detail: Option<ReleaseDetailType>,
    pub(crate) action: Option<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ReleaseInstance {
    release: Release,
}
