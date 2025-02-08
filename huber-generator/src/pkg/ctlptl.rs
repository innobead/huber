use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ctlptl".to_string(),
        source: PackageSource::Github {
            owner: "tilt-dev".to_string(),
            repo: "ctlptl".to_string(),
        },
        ..Default::default()
    }
}
