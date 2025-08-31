use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::profiles::Profile;

#[derive(Debug, Deserialize, Serialize, Clone, TS, Default)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Config {
    pub profiles: Vec<Profile>,
    pub selected_profile_id: Option<u64>,
}
