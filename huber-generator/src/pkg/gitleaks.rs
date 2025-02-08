use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "gitleaks".to_string(),
        source: PackageSource::Github {
            owner: "gitleaks".to_string(),
            repo: "gitleaks".to_string(),
        },
        ..Default::default()
    }
}
