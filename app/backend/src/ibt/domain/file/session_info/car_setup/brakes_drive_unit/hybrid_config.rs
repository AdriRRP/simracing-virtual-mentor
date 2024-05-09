use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HybridConfig {
    pub mgu_k_deploy_mode: Option<String>,
}
