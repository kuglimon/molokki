use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Krangles define if something is Krangled
///
/// This is some quality description right here.
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[cfg_attr(test, derive(Default))]
#[kube(kind = "Krangle", group = "poc", version = "v1", namespaced)]
#[kube(status = "KrangleStatus", shortname = "kra")]
pub struct KrangleSpec {
    pub id: u64,
    // FIXME(tatu): Should be URL
    pub address: String,
    pub corrupted: bool,
    pub enabled: bool,
}

/// The status object of a `Krangle`
#[derive(Deserialize, Serialize, Clone, Default, Debug, JsonSchema)]
pub struct KrangleStatus {
    pub connected: bool,
    pub corrupted: bool,
    pub enabled: bool,
}
