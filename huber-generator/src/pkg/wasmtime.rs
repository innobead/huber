use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "wasmtime".to_string(),
        source: PackageSource::Github {
            owner: "bytecodealliance".to_string(),
            repo: "wasmtime".to_string(),
        },
        ..Default::default()
    }
}
