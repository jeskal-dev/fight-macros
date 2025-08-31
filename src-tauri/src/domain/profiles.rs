use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::macros::Macro;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: u64,
    pub name: String,
    pub function_key: Option<String>,
    pub macros: Vec<Macro>,
}
