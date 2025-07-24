use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub name: String,
    pub switch_key: String,
    pub macros: Vec<Macro>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            name: "Sin titulo".to_string(),
            switch_key: "".to_string(),
            macros: Vec::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Macro {
    pub name: String,
    pub trigger: KeyCombination,
    pub sequence: Vec<MacroAction>,
}

impl Default for Macro {
    fn default() -> Self {
        Self {
            name: "Sin titulo".to_string(),
            trigger: KeyCombination::default(),
            sequence: Vec::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct KeyCombination {
    pub modifiers: Vec<ModifierKey>,
    pub key: String,
}

impl std::fmt::Display for KeyCombination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mods = self
            .modifiers
            .iter()
            .map(|m| format!("{:?}", m))
            .collect::<Vec<_>>()
            .join("+");
        if mods.is_empty() {
            write!(f, "{}", self.key)
        } else {
            write!(f, "{}+{}", mods, self.key)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, EnumString, Display, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModifierKey {
    Alt,
    Control,
    Ctrl,
    Shift,
    Meta,
}

impl Default for ModifierKey {
    fn default() -> Self {
        Self::Alt
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum MacroAction {
    #[strum(to_string = "KeyDown")]
    KeyDown { key: String },
    #[strum(to_string = "KeyUp")]
    KeyUp { key: String },
    #[strum(to_string = "Delay")]
    Delay { ms: f64 },
}

impl MacroAction {
    pub fn action_discriminant(action: &MacroAction) -> &'static str {
        match action {
            MacroAction::KeyDown { .. } => "KeyDown",
            MacroAction::KeyUp { .. } => "KeyUp",
            MacroAction::Delay { .. } => "Delay",
        }
    }

    pub fn variant_to_action(variant: &str) -> MacroAction {
        match variant {
            "KeyDown" => MacroAction::KeyDown {
                key: String::from("A"),
            },
            "KeyUp" => MacroAction::KeyUp {
                key: String::from("A"),
            },
            "Delay" => MacroAction::Delay { ms: 100.0 },
            _ => unreachable!(),
        }
    }
}

impl Default for MacroAction {
    fn default() -> Self {
        MacroAction::KeyDown {
            key: "A".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub profiles: Vec<Profile>,
    pub active_profile: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        let profiles = Vec::new();
        let active_profile = None;

        Self {
            profiles,
            active_profile,
        }
    }
}

impl Config {
    pub fn get_active_profile(&self) -> Option<Profile> {
        self.active_profile?;
        self.profiles.get(self.active_profile.unwrap()).cloned()
    }

    pub fn find_macro(&self, name: &str) -> Option<Macro> {
        self.get_active_profile()?
            .macros
            .iter()
            .find(|m| m.name == name)
            .cloned()
    }
}
