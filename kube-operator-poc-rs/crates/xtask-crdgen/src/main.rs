use krangle_operator::controller;
// FIXME(tatu): This just feels fucking stupid to do here, exported library should cover this.
use kube::CustomResourceExt;

fn main() {
    println!(
        "{}",
        serde_yaml::to_string(&controller::Krangle::crd()).unwrap()
    )
}
