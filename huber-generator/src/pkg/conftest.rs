use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "conftest".to_string(),
        source: PackageSource::Github {
            owner: "open-policy-agent".to_string(),
            repo: "conftest".to_string(),
        },
        ..Default::default()
    }
}
