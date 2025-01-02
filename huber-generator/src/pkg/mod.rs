use huber_common::model::package::PackageTargetType;

fn default_targets() -> Vec<PackageTargetType> {
    vec![
        PackageTargetType::LinuxAmd64(Default::default()),
        PackageTargetType::LinuxArm64(Default::default()),
        PackageTargetType::LinuxArm(Default::default()),
        PackageTargetType::MacOSAmd64(Default::default()),
        PackageTargetType::MacOSArm64(Default::default()),
        PackageTargetType::WindowsAmd64(Default::default()),
    ]
}

fn default_targets_no_arm() -> Vec<PackageTargetType> {
    vec![
        PackageTargetType::LinuxAmd64(Default::default()),
        PackageTargetType::LinuxArm64(Default::default()),
        PackageTargetType::MacOSAmd64(Default::default()),
        PackageTargetType::MacOSArm64(Default::default()),
        PackageTargetType::WindowsAmd64(Default::default()),
    ]
}

fn default_targets_no_arm_windows() -> Vec<PackageTargetType> {
    vec![
        PackageTargetType::LinuxAmd64(Default::default()),
        PackageTargetType::LinuxArm64(Default::default()),
        PackageTargetType::MacOSAmd64(Default::default()),
        PackageTargetType::MacOSArm64(Default::default()),
    ]
}

pub mod ali;
pub mod argocd;
pub mod arkade;
pub mod axelard;
pub mod bat;
pub mod bottom;
pub mod buf;
pub mod camel_k;
pub mod chisel;
pub mod choose;
pub mod cloak;
pub mod codeql;
pub mod conftest;
pub mod consul;
pub mod containerd;
pub mod copilot_cli;
pub mod coreutils;
pub mod cosign;
pub mod croc;
pub mod ctlptl;
pub mod czkawka;
pub mod dasel;
pub mod delta;
pub mod deno;
pub mod direnv;
pub mod dive;
pub mod doctl;
pub mod dog;
pub mod dolt;
pub mod dua_cli;
pub mod dust;
pub mod eksctl;
pub mod exa;
pub mod faas_cli;
pub mod fd;
pub mod firecracker;
pub mod fission;
pub mod fleet;
pub mod flux2;
pub mod fnm;
pub mod fortio;
pub mod frum;
pub mod gh;
pub mod gitui;
pub mod go;
pub mod go_http_tunnel;
pub mod gping;
pub mod gradle;
pub mod grex;
pub mod grpcurl;
pub mod helm;
pub mod helmfile;
pub mod hetty;
pub mod hexyl;
pub mod hub;
pub mod huber;
pub mod hugo;
pub mod hyperfine;
pub mod img;
pub mod istio;
pub mod jiq;
pub mod jless;
pub mod jq;
pub mod just;
pub mod jwt_cli;
pub mod k0s;
pub mod k3d;
pub mod k3s;
pub mod k3sup;
pub mod k6;
pub mod k9s;
pub mod keptn;
pub mod ketch;
pub mod kind;
pub mod ko;
pub mod kompose;
pub mod kotlin;
pub mod kpt;
pub mod krew;
pub mod kube_bench;
pub mod kube_linter;
pub mod kubectl;
pub mod kubefire;
pub mod kubestr;
pub mod kubevirt;
pub mod kudo;
pub mod kustomize;
pub mod kuttl;
pub mod kwctl;
pub mod lens;
pub mod linkerd2_edge;
pub mod linkerd2_stable;
pub mod loc;
pub mod lsd;
pub mod minikube;
pub mod mkcert;
pub mod nat;
pub mod natscli;
pub mod navi;
pub mod nerdctl;
pub mod node;
pub mod nomad;
pub mod norouter;
pub mod nushell;
pub mod octant;
pub mod okteto;
pub mod ollama;
pub mod onefetch;
pub mod opa;
pub mod oras;
pub mod pack;
pub mod packer;
pub mod podman;
pub mod powershell;
pub mod procs;
pub mod protoc;
pub mod pueue;
pub mod pulumi;
pub mod rancher;
pub mod rancher_cli;
pub mod renote;
pub mod rio;
pub mod ripgrep;
pub mod rke;
pub mod rke2;
pub mod rustwasmc;
pub mod sad;
pub mod saml2aws;
pub mod sd;
pub mod shadowsocks;
pub mod shisho;
pub mod skaffold;
pub mod skim;
pub mod solidity;
pub mod sonobuoy;
pub mod ssvm;
pub mod starship;
pub mod stern;
pub mod submariner;
pub mod syncthing;
pub mod tealdeer;
pub mod tecli;
pub mod termshark;
pub mod terraform;
pub mod terrascan;
pub mod tilt;
pub mod tokei;
pub mod tracee;
pub mod trivy;
pub mod typescript;
pub mod typos;
pub mod vegeta;
pub mod velero;
pub mod viddy;
pub mod volta;
pub mod wabt;
pub mod wasm_to_oci;
pub mod wasme;
pub mod wasmer;
pub mod wasmtime;
pub mod wstunnel;
pub mod xh;
pub mod yq;
pub mod zellij;
pub mod zola;
pub mod zoxide;
