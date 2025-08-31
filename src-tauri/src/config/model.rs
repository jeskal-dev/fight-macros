use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use crate::domain::{macros::KeyCombination, sequence_step::SequenceStep};

#[derive(Debug, Default, Clone)]
pub struct FlatStorage {
    pub profiles: AHashMap<u64, StoredProfile>,
    pub macros: AHashMap<u64, StoredMacro>,
    pub steps: AHashMap<u64, SequenceStep>,
    pub selected_profile_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredProfile {
    pub id: u64,
    pub name: String,
    pub function_key: Option<String>,
    pub macro_ids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMacro {
    pub id: u64,
    pub name: String,
    pub trigger: KeyCombination,
    pub sequence_step_ids: Vec<u64>,
}

impl FlatStorage {
    pub fn get_active_profile(&self) -> Option<&StoredProfile> {
        if let Some(id) = self.selected_profile_id {
            self.profiles.get(&id)
        } else {
            None
        }
    }

    pub fn find_macro(&self, id: &u64) -> Option<&StoredMacro> {
        self.macros.get(id)
    }

    pub fn find_profile(&self, id: &u64) -> Option<&StoredProfile> {
        self.profiles.get(id)
    }
}
