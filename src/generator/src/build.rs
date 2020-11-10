#[macro_use]
extern crate maplit;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use huber_common::model::package::{Package, PackageIndex, PackageSource};
use huber_common::result::Result;

use crate::pkg::*;
use hubcaps::{Credentials, Github};

mod pkg;

#[tokio::main]
async fn main() -> Result<()> {
    let generated_dir = &Path::new(env::var("CARGO_MANIFEST_DIR")?.as_str())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("generated")
        .join("packages");

    // clean up and prepare
    fs::remove_dir_all(generated_dir.clone()).unwrap();
    fs::create_dir_all(generated_dir.clone()).unwrap();

    // generate release manifests, index file
    let index_file = Path::new(generated_dir)
        .parent()
        .unwrap()
        .join("index.yaml");
    let mut index_file = File::create(index_file)?;
    writeln!(index_file, "{}", "# This is generated. Don't modify.")?;

    let mut pkg_indexes: Vec<PackageIndex> = vec![];

    for mut r in releases().into_iter() {
        update_description(&mut r).await?;

        let str = format!(
            "# This is generated. Don't modify.\n{}",
            serde_yaml::to_string(&r)?
        );

        pkg_indexes.push(PackageIndex {
            name: r.name.clone(),
            owner: r.source.owner(),
            source: r.source.to_string(),
        });

        let pkg_file = Path::new(generated_dir)
            .join(r.name.clone())
            .with_extension("yaml");

        File::create(pkg_file)?.write_all(str.as_bytes())?;
    }

    pkg_indexes.sort_by(|x, y| x.name.partial_cmp(&y.name).unwrap());

    writeln!(
        index_file,
        "{}",
        serde_yaml::to_string(&pkg_indexes).unwrap()
    )?;

    Ok(())
}

fn releases() -> Vec<Package> {
    vec![
        // tools
        gh::release(),
        dasel::release(),
        bat::release(),
        terraform::release(),
        packer::release(),
        hub::release(),
        // infra
        pulumi::release(),
        // app development
        okteto::release(),
        skaffold::release(),
        kpt::release(),
        oras::release(),
        waypoint::release(),
        // cloud native, kubernetes
        velero::release(),
        helm::release(),
        kubefire::release(),
        k3s::release(),
        k3sup::release(),
        k3ai::release(),
        k3d::release(),
        rke::release(),
        rke2::release(),
        rio::release(),
        istio::release(),
        fleet::release(),
        kube_bench::release(),
        trivy::release(),
        octant::release(),
        pack::release(),
        opa::release(),
        submariner::release(),
        kind::release(),
        krew::release(),
        minikube::release(),
        sonobuoy::release(),
        consul::release(),
        // runtime
        containerd::release(),
        firecracker::release(),
        nomad::release(),
        // programing
        deno::release(),
        typescript::release(),
    ]
}

async fn update_description(pkg: &mut Package) -> Result<()> {
    let github = Github::new("huber", Credentials::Token(env::var("GITHUB_TOKEN")?))?;

    if let PackageSource::Github { owner, repo } = &pkg.source {
        let repo = github.repo(owner, repo).get().await?;
        pkg.description = repo.description;
    }

    Ok(())
}
