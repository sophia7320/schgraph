use crossterm::event::{self, KeyCode, KeyEvent};

use crate::base::{events::Event, ui::View};

pub(super) enum Action {
    Quit,
    Back,

    MenuDown,
    MenuUp,

    SPInputToggleFocus,
    InputChar(char),
    DeletChar,

    Noop,
}

pub(super) fn key_event_handle(key: KeyEvent, view: &View) -> Action {
    match view {
        View::MainMenu => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,

            _ => Action::Noop,
        },

        _ => Action::Noop,
    }
}
