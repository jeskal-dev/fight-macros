use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub name: String,
    pub switch_key: String,
    pub macros: Vec<Macro>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Macro {
    pub name: String,
    pub trigger: KeyCombination,
    pub sequence: Vec<MacroAction>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyCombination {
    pub modifiers: Vec<ModifierKey>,
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ModifierKey {
    Alt,
    Control,
    Ctrl,
    Shift,
    Meta,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum MacroAction {
    KeyDown { key: String },
    KeyUp { key: String },
    Delay { ms: f64 },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub active_profile: String,
}

impl Default for Config {
    fn default() -> Self {
        let profiles = HashMap::new();
        let active_profile = "".to_string();

        Self {
            profiles,
            active_profile,
        }
    }
}

impl Config {
    pub fn get_active_profile(&self) -> Option<&Profile> {
        self.profiles.get(&self.active_profile)
    }

    pub fn find_macro(&self, name: &str) -> Option<&Macro> {
        self.get_active_profile()?
            .macros
            .iter()
            .find(|m| m.name == name)
    }
}
