use phf::phf_map;
use rdev::Key;

use crate::domain::enums::ModifierKey;

static STR_TO_KEY: phf::Map<&'static str, Key> = phf_map! {
    // Letras
    "A" => Key::KeyA, "B" => Key::KeyB, "C" => Key::KeyC, "D" => Key::KeyD,
    "E" => Key::KeyE, "F" => Key::KeyF, "G" => Key::KeyG, "H" => Key::KeyH,
    "I" => Key::KeyI, "J" => Key::KeyJ, "K" => Key::KeyK, "L" => Key::KeyL,
    "M" => Key::KeyM, "N" => Key::KeyN, "O" => Key::KeyO, "P" => Key::KeyP,
    "Q" => Key::KeyQ, "R" => Key::KeyR, "S" => Key::KeyS, "T" => Key::KeyT,
    "U" => Key::KeyU, "V" => Key::KeyV, "W" => Key::KeyW, "X" => Key::KeyX,
    "Y" => Key::KeyY, "Z" => Key::KeyZ,

    // Números
    "0" => Key::Num0, ")" => Key::Num0,
    "1" => Key::Num1, "!" => Key::Num1,
    "2" => Key::Num2, "@" => Key::Num2,
    "3" => Key::Num3, "#" => Key::Num3,
    "4" => Key::Num4, "$" => Key::Num4,
    "5" => Key::Num5, "%" => Key::Num5,
    "6" => Key::Num6, "^" => Key::Num6,
    "7" => Key::Num7, "&" => Key::Num7,
    "8" => Key::Num8, "*" => Key::Num8,
    "9" => Key::Num9, "(" => Key::Num9,

    // Funciones
    "F1" => Key::F1,  "F2" => Key::F2,  "F3" => Key::F3,  "F4" => Key::F4,
    "F5" => Key::F5,  "F6" => Key::F6,  "F7" => Key::F7,  "F8" => Key::F8,
    "F9" => Key::F9,  "F10" => Key::F10, "F11" => Key::F11, "F12" => Key::F12,

    // Control
    "ENTER" => Key::Return, "\n" => Key::Return, "\r" => Key::Return,
    "ESC" => Key::Escape, "ESCAPE" => Key::Escape,
    "BACKSPACE" => Key::Backspace,
    "TAB" => Key::Tab,
    "SPACE" => Key::Space, " " => Key::Space,
    "CAPSLOCK" => Key::CapsLock,
    "SHIFT" => Key::ShiftLeft,
    "CTRL" => Key::ControlLeft, "CONTROL" => Key::ControlLeft,
    "ALT" => Key::Alt,
    "GUI" => Key::MetaLeft, "SUPER" => Key::MetaLeft, "WIN" => Key::MetaLeft,
    "WINDOWS" => Key::MetaLeft, "COMMAND" => Key::MetaLeft,

    // Navegación
    "UP" => Key::UpArrow, "UPARROW" => Key::UpArrow,
    "DOWN" => Key::DownArrow, "DOWNARROW" => Key::DownArrow,
    "LEFT" => Key::LeftArrow, "LEFTARROW" => Key::LeftArrow,
    "RIGHT" => Key::RightArrow, "RIGHTARROW" => Key::RightArrow,
    "PAGEUP" => Key::PageUp,
    "PAGEDOWN" => Key::PageDown,
    "HOME" => Key::Home,
    "END" => Key::End,
    "INSERT" => Key::Insert,
    "DELETE" => Key::Delete,

    // Puntuación
    "`" => Key::BackQuote, "~" => Key::BackQuote,
    "-" => Key::Minus, "_" => Key::Minus,
    "=" => Key::Equal, "+" => Key::Equal,
    "[" => Key::LeftBracket, "{" => Key::LeftBracket,
    "]" => Key::RightBracket, "}" => Key::RightBracket,
    "\\" => Key::BackSlash, "|" => Key::BackSlash,
    ";" => Key::SemiColon, ":" => Key::SemiColon,
    "'" => Key::Quote, "\"" => Key::Quote,
    "," => Key::Comma, "<" => Key::Comma,
    "." => Key::Dot, ">" => Key::Dot,
    "/" => Key::Slash, "?" => Key::Slash,

    // Teclado numérico
    "NUMLOCK" => Key::NumLock,
    "NUM_0" => Key::Kp0, "NUM0" => Key::Kp0,
    "NUM_1" => Key::Kp1, "NUM1" => Key::Kp1,
    "NUM_2" => Key::Kp2, "NUM2" => Key::Kp2,
    "NUM_3" => Key::Kp3, "NUM3" => Key::Kp3,
    "NUM_4" => Key::Kp4, "NUM4" => Key::Kp4,
    "NUM_5" => Key::Kp5, "NUM5" => Key::Kp5,
    "NUM_6" => Key::Kp6, "NUM6" => Key::Kp6,
    "NUM_7" => Key::Kp7, "NUM7" => Key::Kp7,
    "NUM_8" => Key::Kp8, "NUM8" => Key::Kp8,
    "NUM_9" => Key::Kp9, "NUM9" => Key::Kp9,
    "NUM_DIVIDE" => Key::KpDivide, "NUM/" => Key::KpDivide,
    "NUM_MULTIPLY" => Key::KpMultiply, "NUM*" => Key::KpMultiply,
    "NUM_ENTER" => Key::KpReturn,

    // Misc
    "PRINTSCREEN" => Key::PrintScreen,
    "SCROLLLOCK" => Key::ScrollLock,
    "PAUSE" => Key::Pause,
};

pub fn str_to_key(s: &str) -> Key {
    STR_TO_KEY
        .get(&*s.to_uppercase())
        .copied()
        .unwrap_or(Key::Unknown(0))
}

pub fn mod_to_key(key: &ModifierKey) -> Key {
    match key {
        ModifierKey::Alt => Key::Alt,
        ModifierKey::Control | ModifierKey::Ctrl => Key::ControlLeft,
        ModifierKey::Shift => Key::ShiftLeft,
        ModifierKey::Meta => Key::MetaLeft,
    }
}

pub fn key_matches(expected: &str, actual: rdev::Key) -> bool {
    let expected = str_to_key(expected);

    expected == actual
}
