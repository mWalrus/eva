use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct ExKeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl ExKeyEvent {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

impl PartialEq for ExKeyEvent {
    fn eq(&self, other: &Self) -> bool {
        let ev: KeyEvent = self.into();
        let other: KeyEvent = other.into();
        ev == other
    }
}

impl From<&ExKeyEvent> for KeyEvent {
    fn from(other: &ExKeyEvent) -> Self {
        Self::new(other.code, other.modifiers)
    }
}

pub struct ExKeyList {
    pub up: ExKeyEvent,
    pub down: ExKeyEvent,
    pub jump_up: ExKeyEvent,
    pub jump_down: ExKeyEvent,
    pub top: ExKeyEvent,
    pub bottom: ExKeyEvent,
    pub quit: ExKeyEvent,
}

pub fn key_match(key: &KeyEvent, bind: &ExKeyEvent) -> bool {
    key.code == bind.code && key.modifiers == bind.modifiers
}

impl Default for ExKeyList {
    fn default() -> Self {
        Self {
            up: ExKeyEvent::new(KeyCode::Up, KeyModifiers::empty()),
            down: ExKeyEvent::new(KeyCode::Down, KeyModifiers::empty()),
            jump_up: ExKeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT),
            jump_down: ExKeyEvent::new(KeyCode::Tab, KeyModifiers::empty()),
            top: ExKeyEvent::new(KeyCode::Char('t'), KeyModifiers::empty()),
            bottom: ExKeyEvent::new(KeyCode::Char('b'), KeyModifiers::empty()),
            quit: ExKeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty()),
        }
    }
}
