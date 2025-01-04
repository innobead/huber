use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use async_trait::async_trait;
use git2::{Cred, ErrorCode, FetchOptions, RemoteCallbacks, Repository};
use huber_common::model::package::Package;
use huber_common::model::release::Release;
use log::debug;
use octocrab::auth::Auth;
use octocrab::{Octocrab, OctocrabBuilder};

use crate::file::is_empty_dir;

#[async_trait]
pub trait GithubClientTrait {
    async fn get_latest_release(
        &self,
        owner: &str,
        repo: &str,
        pkg: &Package,
    ) -> anyhow::Result<Release>;
    async fn get_release(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
        pkg: &Package,
    ) -> anyhow::Result<Release>;
    async fn get_releases(
        &self,
        owner: &str,
        repo: &str,
        pkg: &Package,
    ) -> anyhow::Result<Vec<Release>>;
    async fn download_artifacts<P: AsRef<Path> + Send>(
        &self,
        release: &Release,
        dir: P,
    ) -> anyhow::Result<()>;
    async fn clone<P: AsRef<Path> + Send + Sync>(
        &self,
        owner: &str,
        repo: &str,
        dir: P,
    ) -> anyhow::Result<()>;
}

pub struct GithubClient {
    octocrab: Octocrab,
    ssh_key: Option<PathBuf>,
}

unsafe impl Send for GithubClient {}

unsafe impl Sync for GithubClient {}

impl GithubClient {
    pub fn new(auth: Auth, ssh_key: Option<PathBuf>) -> Self {
        Self {
            octocrab: {
                let builder = OctocrabBuilder::default();
                match auth {
                    Auth::PersonalToken(token) => builder.personal_token(token),
                    Auth::UserAccessToken(token) => builder.user_access_token(token),
                    _ => {
                        debug!("Use github client with no auth due to unsupported auth");
                        builder
                    }
                }
                .build()
                .expect("failed to build GitHub client")
            },
            ssh_key,
        }
    }

    fn clone_fresh<P: AsRef<Path> + Send>(&self, url: &str, dir: P) -> anyhow::Result<Repository> {
        let clone_repo_by_key = |key: &PathBuf| -> anyhow::Result<Repository> {
            if key.exists() {
                debug!("Cloning huber repo via SSH");

                // Prepare builder.
                let fetch_options = self.create_git_fetch_options(key.clone())?;
                let mut builder = git2::build::RepoBuilder::new();
                builder.fetch_options(fetch_options);

                return Ok(builder.clone(url, dir.as_ref())?);
            }

            Err(anyhow!("The configured github key not found, {:?}", key))
        };

        if let Some(key) = self.ssh_key.as_ref() {
            return clone_repo_by_key(key);
        }

        debug!("Cloning huber repo via https");
        //Note: if encountering authentication required, probably hit this issue https://github.com/rust-lang/git2-rs/issues/463
        match Repository::clone(url, &dir) {
            Err(err) => {
                if err.code() == ErrorCode::GenericError
                    && err
                        .message()
                        .contains("authentication required but no callback set")
                {
                    debug!("Failed to clone huber repo due to the SSH key required as per the user git config");
                    debug!("Using the default user key path to try cloning huber repo again");

                    let p = dirs::home_dir().unwrap().join(".ssh").join("id_rsa");
                    clone_repo_by_key(&p)
                } else {
                    Err(anyhow!(err))
                }
            }

            Ok(repo) => Ok(repo),
        }
    }

    fn fetch_merge_repo<P: AsRef<Path>>(&self, dir: P) -> anyhow::Result<()> {
        debug!("Merging huber repo update");

        let mut fetch_options = if let Some(key) = self.ssh_key.as_ref() {
            if key.exists() {
                debug!("Fetching huber repo via SSH");
                Some(self.create_git_fetch_options(key.clone())?)
            } else {
                None
            }
        } else {
            None
        };

        let repo = Repository::open(&dir)?;

        // fetch the origin
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["master"], fetch_options.as_mut(), None)?;
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let commit = repo.reference_to_annotated_commit(&fetch_head)?;

        // merge local, and checkout
        let reference_name = format!("refs/heads/{}", "master");
        let mut reference = repo.find_reference(&reference_name)?;
        let name = reference.name().expect("");
        repo.set_head(name)?;
        reference.set_target(commit.id(), "")?;

        Ok(repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?)
    }

    fn create_git_fetch_options<T: AsRef<Path> + 'static>(
        &self,
        key: T,
    ) -> anyhow::Result<FetchOptions> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            Cred::ssh_key(username_from_url.unwrap(), None, key.as_ref(), None)
        });

        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        Ok(fo)
    }
}

#[async_trait]
impl GithubClientTrait for GithubClient {
    async fn get_latest_release(
        &self,
        owner: &str,
        repo: &str,
        pkg: &Package,
    ) -> anyhow::Result<Release> {
        debug!("Getting the latest release of package {}", &pkg);

        let release = if pkg.target()?.tag_version_regex_template.is_none() {
            self.octocrab
                .repos(owner, repo)
                .releases()
                .get_latest()
                .await?
        } else {
            self.octocrab.repos(owner, repo).releases().list().send().await?.into_iter().find(|it| {
                pkg.parse_version_from_tag_name(&it.tag_name).is_ok()
            }).ok_or(anyhow!("Failed to find the matched latest version based on tag_version_regex_template {:?}", pkg))?
        };

        let mut release = Release::from(release);

        release.name = pkg.name.clone();
        release.package.name = pkg.name.clone();
        release.package.source = pkg.source.clone();
        release.package.targets = pkg.targets.clone();
        release.package.version = Some(release.version.clone());

        Ok(release)
    }

    async fn get_release(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
        pkg: &Package,
    ) -> anyhow::Result<Release> {
        debug!("Getting the specific release of package {}/{}", &pkg, tag);

        let release = self
            .octocrab
            .repos(owner, repo)
            .releases()
            .get_by_tag(tag)
            .await?;
        let mut release = Release::from(release);

        release.name = pkg.name.clone();
        release.package.name = pkg.name.clone();
        release.package.source = pkg.source.clone();
        release.package.targets = pkg.targets.clone();
        release.package.version = Some(release.version.clone());

        Ok(release)
    }

    async fn get_releases(
        &self,
        owner: &str,
        repo: &str,
        pkg: &Package,
    ) -> anyhow::Result<Vec<Release>> {
        debug!("Getting all releases of package {}", &pkg);

        let releases = self
            .octocrab
            .repos(owner, repo)
            .releases()
            .list()
            .send()
            .await?;
        let releases = releases
            .into_iter()
            .map(|it| {
                let mut release = Release::from(it);

                release.name = pkg.name.clone();
                release.package.name = pkg.name.clone();
                release.package.source = pkg.source.clone();
                release.package.targets = pkg.targets.clone();
                release.package.release_kind = release.kind;

                release
            })
            .collect();

        Ok(releases)
    }

    async fn download_artifacts<P: AsRef<Path> + Send>(
        &self,
        _release: &Release,
        _dir: P,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    async fn clone<P: AsRef<Path> + Send + Sync>(
        &self,
        owner: &str,
        repo: &str,
        dir: P,
    ) -> anyhow::Result<()> {
        debug!("Cloning huber github repo");

        let url = format!("https://github.com/{}/{}", owner, repo);

        if is_empty_dir(&dir) {
            self.clone_fresh(&url, &dir)?;
            return Ok(());
        }

        if let Err(e) = self.fetch_merge_repo(&dir) {
            debug!("Failed to fetch huber github repo, {:?}", e);

            let _ = remove_dir_all(&dir);
            self.clone_fresh(&url, &dir)?;
        }

        Ok(())
    }
}
