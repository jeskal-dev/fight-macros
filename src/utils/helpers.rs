use std::collections::HashSet;

use rdev::Key;

use crate::{
    config::types::{KeyCombination, ModifierKey},
    input::combo,
};

pub fn mod_to_key(key: &ModifierKey) -> Key {
    match key {
        ModifierKey::Alt => Key::Alt,
        ModifierKey::Control | ModifierKey::Ctrl => Key::ControlLeft,
        ModifierKey::Shift => Key::ShiftLeft,
        ModifierKey::Meta => Key::MetaLeft,
    }
}

pub fn str_to_key(s: &str) -> Key {
    match s.to_uppercase().as_str() {
        // Letras
        "A" => Key::KeyA,
        "B" => Key::KeyB,
        "C" => Key::KeyC,
        "D" => Key::KeyD,
        "E" => Key::KeyE,
        "F" => Key::KeyF,
        "G" => Key::KeyG,
        "H" => Key::KeyH,
        "I" => Key::KeyI,
        "J" => Key::KeyJ,
        "K" => Key::KeyK,
        "L" => Key::KeyL,
        "M" => Key::KeyM,
        "N" => Key::KeyN,
        "O" => Key::KeyO,
        "P" => Key::KeyP,
        "Q" => Key::KeyQ,
        "R" => Key::KeyR,
        "S" => Key::KeyS,
        "T" => Key::KeyT,
        "U" => Key::KeyU,
        "V" => Key::KeyV,
        "W" => Key::KeyW,
        "X" => Key::KeyX,
        "Y" => Key::KeyY,
        "Z" => Key::KeyZ,

        // Números
        "0" | ")" => Key::Num0,
        "1" | "!" => Key::Num1,
        "2" | "@" => Key::Num2,
        "3" | "#" => Key::Num3,
        "4" | "$" => Key::Num4,
        "5" | "%" => Key::Num5,
        "6" | "^" => Key::Num6,
        "7" | "&" => Key::Num7,
        "8" | "*" => Key::Num8,
        "9" | "(" => Key::Num9,

        // Teclas de función
        "F1" => Key::F1,
        "F2" => Key::F2,
        "F3" => Key::F3,
        "F4" => Key::F4,
        "F5" => Key::F5,
        "F6" => Key::F6,
        "F7" => Key::F7,
        "F8" => Key::F8,
        "F9" => Key::F9,
        "F10" => Key::F10,
        "F11" => Key::F11,
        "F12" => Key::F12,

        // Teclas de control
        "ENTER" | "\n" | "\r" => Key::Return,
        "ESC" | "ESCAPE" => Key::Escape,
        "BACKSPACE" => Key::Backspace,
        "TAB" => Key::Tab,
        "SPACE" | " " => Key::Space,
        "CAPSLOCK" => Key::CapsLock,
        "SHIFT" => Key::ShiftLeft,
        "CTRL" | "CONTROL" => Key::ControlLeft,
        "ALT" => Key::Alt,
        "GUI" | "SUPER" | "WIN" | "WINDOWS" | "COMMAND" => Key::MetaLeft,

        // Teclas de navegación
        "UP" | "UPARROW" => Key::UpArrow,
        "DOWN" | "DOWNARROW" => Key::DownArrow,
        "LEFT" | "LEFTARROW" => Key::LeftArrow,
        "RIGHT" | "RIGHTARROW" => Key::RightArrow,
        "PAGEUP" => Key::PageUp,
        "PAGEDOWN" => Key::PageDown,
        "HOME" => Key::Home,
        "END" => Key::End,
        "INSERT" => Key::Insert,
        "DELETE" => Key::Delete,

        // Teclas de puntuación
        "`" | "~" => Key::BackQuote,
        "-" | "_" => Key::Minus,
        "=" | "+" => Key::Equal,
        "[" | "{" => Key::LeftBracket,
        "]" | "}" => Key::RightBracket,
        "\\" | "|" => Key::BackSlash,
        ";" | ":" => Key::SemiColon,
        "'" | "\"" => Key::Quote,
        "," | "<" => Key::Comma,
        "." | ">" => Key::Dot,
        "/" | "?" => Key::Slash,

        // Teclas numéricas
        "NUMLOCK" => Key::NumLock,
        "NUM_0" => Key::Kp0,
        "NUM_1" => Key::Kp1,
        "NUM_2" => Key::Kp2,
        "NUM_3" => Key::Kp3,
        "NUM_4" => Key::Kp4,
        "NUM_5" => Key::Kp5,
        "NUM_6" => Key::Kp6,
        "NUM_7" => Key::Kp7,
        "NUM_8" => Key::Kp8,
        "NUM_9" => Key::Kp9,
        "NUM_DIVIDE" | "NUM/" => Key::KpDivide,
        "NUM_MULTIPLY" | "NUM*" => Key::KpMultiply,
        "NUM_ENTER" => Key::KpReturn,

        // Otras teclas
        "PRINTSCREEN" => Key::PrintScreen,
        "SCROLLLOCK" => Key::ScrollLock,
        "PAUSE" => Key::Pause,

        // Tecla desconocida
        _ => Key::Unknown(0),
    }
}

pub fn key_matches(expected: &str, actual: rdev::Key) -> bool {
    let expected = str_to_key(expected);

    expected == actual
}

pub fn is_combo_completed(combo: &KeyCombination, state: &combo::KeyState) -> bool {
    let mut required_keys = HashSet::new();

    let base_key = str_to_key(&combo.key);
    required_keys.insert(base_key);
    if !state.pressed().contains(&base_key) {
        return false;
    }
    for m in &combo.modifiers {
        let modifier_key = mod_to_key(m);
        required_keys.insert(modifier_key);
    }

    required_keys.is_subset(state.pressed())
}
