use crossterm::event::{self, KeyCode, KeyEvent};

use crate::base::{events::Event, ui::View};

pub(super) enum Action {
    Quit,
    Back,

    MenuDown,
    MenuUp,
    MenuSelect,

    SPInputToggleFocus,
    InputChar(char),
    DeletChar,

    Noop,
}

pub(super) fn key_event_handle(key: KeyEvent, view: &View) -> Action {
    match view {
        View::MainMenu => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
            KeyCode::Char('j') | KeyCode::Down => Action::MenuDown,
            KeyCode::Char('k') | KeyCode::Up => Action::MenuUp,
            KeyCode::Enter | KeyCode::Tab => Action::MenuSelect,
            _ => Action::Noop,
        },

        _ => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Action::Back,

            _ => Action::Noop,
        },
    }
}
