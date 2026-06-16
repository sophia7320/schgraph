use crossterm::event::{KeyCode, KeyEvent};

use crate::base::ui::View;

pub(super) enum Action {
    Quit,
    Back,

    MenuDown,
    MenuUp,
    MenuSelect,

    SPInputToggleFocus,
    InputChar(char),
    DeletChar,
    IncreaseSelectTab,
    DecreaseSelectTab,

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

        View::ShortestPath => match key.code {
            KeyCode::Char(c) if c.is_ascii_digit() => Action::InputChar(c),
            KeyCode::Backspace => Action::DeletChar,
            KeyCode::Tab => Action::SPInputToggleFocus,
            KeyCode::Char('h') => Action::DecreaseSelectTab,
            KeyCode::Char('l') => Action::IncreaseSelectTab,
            KeyCode::Char('q') | KeyCode::Esc => Action::Back,

            _ => Action::Noop,
        },

        _ => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Action::Back,

            _ => Action::Noop,
        },
    }
}
