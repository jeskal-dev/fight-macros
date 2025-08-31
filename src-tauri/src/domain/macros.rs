use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::{enums::ModifierKey, sequence_step::SequenceStep};

#[derive(Debug, Deserialize, Serialize, Clone, Default, TS, PartialEq, Eq, Hash)]
#[ts(export)]
pub struct KeyCombination {
    pub modifiers: Vec<ModifierKey>,
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Macro {
    pub id: u64,
    pub name: String,
    pub trigger: KeyCombination,
    pub sequence: Vec<SequenceStep>,
}
