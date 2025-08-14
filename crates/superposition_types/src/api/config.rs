use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug)]
pub struct ContextPayload {
    pub context: Map<String, Value>,
}
