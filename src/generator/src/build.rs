#[macro_use]
extern crate maplit;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use hubcaps_ex::{Credentials, Github};

use huber_common::model::package::{Package, PackageIndex, PackageSource};
use huber_common::result::Result;

use crate::pkg::*;

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
        jiq::release(),
        jless::release(),
        onefetch::release(),
        renote::release(),
        gh::release(),
        dasel::release(),
        bat::release(),
        terraform::release(),
        packer::release(),
        hub::release(),
        syncthing::release(),
        powershell::release(),
        mkcert::release(),
        huber::release(),
        ali::release(),
        gping::release(),
        gitui::release(),
        ripgrep::release(),
        starship::release(),
        tokei::release(),
        exa::release(),
        fd::release(),
        procs::release(),
        k6::release(),
        fortio::release(),
        jwt_cli::release(),
        direnv::release(),
        tracee::release(),
        vegeta::release(),
        yq::release(),
        stern::release(),
        drone_cli::release(),
        img::release(),
        dive::release(),
        wasme::release(),
        hyperfine::release(),
        hetty::release(),
        czkawka::release(),
        cloak::release(),
        jq::release(),
        termshark::release(),
        volta::release(),
        just::release(),
        croc::release(),
        terrascan::release(),
        tecli::release(),
        nerdctl::release(),
        zoxide::release(),
        dust::release(),
        pueue::release(),
        coreutils::release(),
        hugo::release(),
        typos::release(),
        zellij::release(),
        xh::release(),
        loc::release(),
        choose::release(),
        delta::release(),
        dog::release(),
        dua_cli::release(),
        skim::release(),
        hexyl::release(),
        lsd::release(),
        fnm::release(),
        frum::release(),
        nat::release(),
        sad::release(),
        sd::release(),
        navi::release(),
        tealdeer::release(),
        bottom::release(),
        grex::release(),
        codeql::release(),
        viddy::release(),
        cosign::release(),
        saml2aws::release(),
        grpcurl::release(),
        // infra
        pulumi::release(),
        chisel::release(),
        go_http_tunnel::release(),
        shadowsocks::release(),
        norouter::release(),
        wstunnel::release(),
        doctl::release(),
        zola::release(),
        nushell::release(),
        dolt::release(),
        // app development
        okteto::release(),
        skaffold::release(),
        kpt::release(),
        oras::release(),
        waypoint::release(),
        tilt::release(),
        ko::release(),
        protoc::release(),
        copilot_cli::release(),
        rustwasmc::release(),
        // cloud native, kubernetes
        rancher::release(),
        rancher_cli::release(),
        velero::release(),
        helm::release(),
        helmfile::release(),
        kubefire::release(),
        kubectl::release(),
        kustomize::release(),
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
        conftest::release(),
        submariner::release(),
        kind::release(),
        krew::release(),
        minikube::release(),
        sonobuoy::release(),
        consul::release(),
        ctlptl::release(),
        arkade::release(),
        faas_cli::release(),
        fission::release(),
        k9s::release(),
        lens::release(),
        k0s::release(),
        kudo::release(),
        kuttl::release(),
        flux2::release(),
        argocd::release(),
        ketch::release(),
        kompose::release(),
        eksctl::release(),
        linkerd2_edge::release(),
        linkerd2_stable::release(),
        krustlet::release(),
        camel_k::release(),
        keptn::release(),
        kubevirt::release(),
        kubestr::release(),
        kube_linter::release(),
        natscli::release(),
        kwctl::release(),
        epinio::release(),
        hypper::release(),
        opni::release(),
        // runtime
        containerd::release(),
        firecracker::release(),
        nomad::release(),
        podman::release(),
        ignite::release(),
        wasmtime::release(),
        wasmer::release(),
        ssvm::release(),
        wasm_to_oci::release(),
        wabt::release(),
        // programing
        deno::release(),
        typescript::release(),
        node::release(),
        kotlin::release(),
        gradle::release(),
    ]
}

async fn update_description(pkg: &mut Package) -> Result<()> {
    println!("Updating the description of package: {}", pkg);
    let github = Github::new("huber", Credentials::Token(env::var("GITHUB_TOKEN")?))?;

    if let PackageSource::Github { owner, repo } = &pkg.source {
        let repo = github.repo(owner, repo).get().await?;
        pkg.description = repo.description;
    }

    Ok(())
}
