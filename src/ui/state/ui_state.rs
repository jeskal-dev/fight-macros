use crate::config::types::Profile;

#[derive(Debug, Clone, Default)]
pub struct UIState {
    pub profiles: Vec<Profile>,
    pub current_profile: Option<usize>,
    pub active_profile: Option<usize>,
    pub current_macro: Option<usize>,
    pub modal: Modal,
    pub modal_open: bool,
    pub picking_switch_key: Option<usize>,
}

impl UIState {
    pub fn new(profiles: Vec<Profile>, active_profile: Option<usize>) -> Self {
        Self {
            profiles,
            current_profile: None,
            active_profile,
            current_macro: None,
            modal: Modal::default(),
            modal_open: false,
            picking_switch_key: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Modal {
    #[default]
    None,
    TriggerEditor(usize),
}
