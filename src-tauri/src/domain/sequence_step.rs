use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, TS, EnumString, Display)]
#[serde(rename_all = "lowercase", tag = "type")]
#[ts(export)]
pub enum SequenceStep {
    #[strum(to_string = "KeyDown")]
    KeyDown { id: u64, key: String },
    #[strum(to_string = "KeyUp")]
    KeyUp { id: u64, key: String },
    #[strum(to_string = "Delay")]
    Delay { id: u64, ms: u64 },
}
