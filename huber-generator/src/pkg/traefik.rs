use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "traefik".to_string(),
        source: PackageSource::Github {
            owner: "traefik".to_string(),
            repo: "traefik".to_string(),
        },
        ..Default::default()
    }
}
