use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "wasmer".to_string(),
        source: PackageSource::Github {
            owner: "wasmerio".to_string(),
            repo: "wasmer".to_string(),
        },
        ..Default::default()
    }
}
