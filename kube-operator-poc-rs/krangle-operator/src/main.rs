use kube::CustomResourceExt;
mod controller;

fn main() {
    print!(
        "{}",
        serde_yaml::to_string(&controller::Krangle::crd()).unwrap()
    )
}
