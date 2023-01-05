use evdev::Key;

pub const fn scan_to_hid(key: &Key) -> u8 {
    match *key {
        // Reserved => 0
        // ErrorRollOver => 1
        // POSTFail => 2
        // ErrorUndefined => 3
        Key::KEY_A => 0x04,
        Key::KEY_B => 0x05,
        Key::KEY_C => 0x06,
        Key::KEY_D => 0x07,
        Key::KEY_E => 0x08,
        Key::KEY_F => 0x09,
        Key::KEY_G => 0x0A,
        Key::KEY_H => 0x0B,
        Key::KEY_I => 0x0C,
        Key::KEY_J => 0x0D,
        Key::KEY_K => 0x0E,
        Key::KEY_L => 0x0F,
        Key::KEY_M => 0x10,
        Key::KEY_N => 0x11,
        Key::KEY_O => 0x12,
        Key::KEY_P => 0x13,
        Key::KEY_Q => 0x14,
        Key::KEY_R => 0x15,
        Key::KEY_S => 0x16,
        Key::KEY_T => 0x17,
        Key::KEY_U => 0x18,
        Key::KEY_V => 0x19,
        Key::KEY_W => 0x1A,
        Key::KEY_X => 0x1B,
        Key::KEY_Y => 0x1C,
        Key::KEY_Z => 0x1D,
        Key::KEY_1 => 0x1E,
        Key::KEY_2 => 0x1F,
        Key::KEY_3 => 0x20,
        Key::KEY_4 => 0x21,
        Key::KEY_5 => 0x22,
        Key::KEY_6 => 0x23,
        Key::KEY_7 => 0x24,
        Key::KEY_8 => 0x25,
        Key::KEY_9 => 0x26,
        Key::KEY_0 => 0x27,
        Key::KEY_ENTER => 0x28,
        Key::KEY_ESC => 0x29,
        Key::KEY_BACKSPACE => 0x2A,
        Key::KEY_TAB => 0x2B,
        Key::KEY_SPACE => 0x2C,
        Key::KEY_MINUS => 0x2D,
        Key::KEY_EQUAL => 0x2E,
        Key::KEY_LEFTBRACE => 0x2F,
        Key::KEY_RIGHTBRACE => 0x30,
        Key::KEY_BACKSLASH => 0x31,
        // Non-US # and ~ => 0x32,
        Key::KEY_SEMICOLON => 0x33,
        Key::KEY_APOSTROPHE => 0x34,
        Key::KEY_GRAVE => 0x35,
        Key::KEY_COMMA => 0x36,
        Key::KEY_DOT => 0x37,
        Key::KEY_SLASH => 0x38,
        Key::KEY_CAPSLOCK => 0x39,
        Key::KEY_F1 => 0x3A,
        Key::KEY_F2 => 0x3B,
        Key::KEY_F3 => 0x3C,
        Key::KEY_F4 => 0x3D,
        Key::KEY_F5 => 0x3E,
        Key::KEY_F6 => 0x3F,
        Key::KEY_F7 => 0x40,
        Key::KEY_F8 => 0x41,
        Key::KEY_F9 => 0x42,
        Key::KEY_F10 => 0x43,
        Key::KEY_F11 => 0x44,
        Key::KEY_F12 => 0x45,
        Key::KEY_PRINT => 0x46,
        Key::KEY_SCROLLLOCK => 0x47,
        Key::KEY_PAUSE => 0x48,
        Key::KEY_INSERT => 0x49,
        Key::KEY_HOME => 0x4A,
        Key::KEY_PAGEUP => 0x4B,
        Key::KEY_DELETE => 0x4C,
        Key::KEY_END => 0x4D,
        Key::KEY_PAGEDOWN => 0x4E,
        Key::KEY_RIGHT => 0x4F,
        Key::KEY_LEFT => 0x50,
        Key::KEY_DOWN => 0x51,
        Key::KEY_UP => 0x52,
        Key::KEY_NUMLOCK => 0x53,
        Key::KEY_KPSLASH => 0x54,
        Key::KEY_KPASTERISK => 0x55,
        Key::KEY_KPMINUS => 0x56,
        Key::KEY_KPPLUS => 0x57,
        Key::KEY_KPENTER => 0x58,
        Key::KEY_KP1 => 0x59,
        Key::KEY_KP2 => 0x5A,
        Key::KEY_KP3 => 0x5B,
        Key::KEY_KP4 => 0x5C,
        Key::KEY_KP5 => 0x5D,
        Key::KEY_KP6 => 0x5E,
        Key::KEY_KP7 => 0x5F,
        Key::KEY_KP8 => 0x60,
        Key::KEY_KP9 => 0x61,
        Key::KEY_KP0 => 0x62,
        Key::KEY_KPDOT => 0x63,
        // non-us / and | => 0x64,
        Key::KEY_APPSELECT => 0x65,
        Key::KEY_POWER => 0x66,
        Key::KEY_KPEQUAL => 0x67,
        Key::KEY_F13 => 0x68,
        Key::KEY_F14 => 0x69,
        Key::KEY_F15 => 0x6A,
        Key::KEY_F16 => 0x6B,
        Key::KEY_F17 => 0x6C,
        Key::KEY_F18 => 0x6D,
        Key::KEY_F19 => 0x6E,
        Key::KEY_F20 => 0x6F,
        Key::KEY_F21 => 0x70,
        Key::KEY_F22 => 0x71,
        Key::KEY_F23 => 0x72,
        Key::KEY_F24 => 0x73,
        // execute
        Key::KEY_HELP => 0x75,
        Key::KEY_MENU => 0x76,
        Key::KEY_SELECT => 0x77,
        Key::KEY_STOP => 0x78,
        Key::KEY_AGAIN => 0x79,
        Key::KEY_UNDO => 0x7A,
        Key::KEY_CUT => 0x7B,
        Key::KEY_COPY => 0x7C,
        Key::KEY_PASTE => 0x7D,
        Key::KEY_FIND => 0x7E,
        Key::KEY_MUTE => 0x7F,
        Key::KEY_VOLUMEUP => 0x80,
        Key::KEY_VOLUMEDOWN => 0x81,
        Key::KEY_KPCOMMA => 0x85,
        Key::KEY_KPEQUAL => 0x86,
        Key::KEY_SYSRQ => 0x9A,
        _ => 0
    }
}

pub const fn modifiers(key: Key) -> u8 {
    match key {
        Key::KEY_LEFTCTRL => 1 << 0,
        Key::KEY_LEFTSHIFT => 1 << 1,
        Key::KEY_LEFTALT => 1 << 2,
        Key::KEY_LEFTMETA => 1 << 3,
        Key::KEY_RIGHTCTRL => 1 << 4,
        Key::KEY_RIGHTSHIFT => 1 << 5,
        Key::KEY_RIGHTALT => 1 << 6,
        Key::KEY_RIGHTMETA => 1 << 7,
        _ => 0
    }
}