use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "mkcert".to_string(),
        source: PackageSource::Github {
            owner: "FiloSottile".to_string(),
            repo: "mkcert".to_string(),
        },
        ..Default::default()
    }
}
