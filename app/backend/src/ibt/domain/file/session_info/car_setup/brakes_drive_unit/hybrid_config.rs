use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HybridConfig {
    pub mgu_k_deploy_mode: Option<String>,
}
