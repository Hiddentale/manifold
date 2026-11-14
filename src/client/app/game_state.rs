use super::menu::MenuState;
use crate::client::session::client_session::Session;

pub enum GameState {
    Menu(MenuState),
    Playing(Session),
}

impl GameState {
    pub fn is_menu(&self) -> bool {
        matches!(self, GameState::Menu(_))
    }
}
