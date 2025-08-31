use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, TS, EnumString, Display, PartialEq, Eq, Hash)]
#[ts(export)]
pub enum ModifierKey {
    Alt,
    Control,
    Ctrl,
    Shift,
    Meta,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize, // JSON
    TS,          // TypeScript
    EnumString,
    Display, // strum
)]
#[ts(export)]
pub enum ListenableChannel {
    #[serde(rename = "selected-profile-changed")]
    #[strum(serialize = "selected-profile-changed")]
    SelectedProfileChanged,
    #[serde(rename = "sequence-step")]
    #[strum(serialize = "sequence-step")]
    SequenceStep,
}
