use crate::v1::JsCallback;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "Optimization", rename_all = "camelCase")]
pub struct OptimizationConfig {
    #[serde(default)]
    pub minimize: bool,

    #[serde(skip)]
    pub minimizer: Option<JsCallback<String, String>>,

    #[serde(default)]
    pub split_chunks: Option<()>,
}
