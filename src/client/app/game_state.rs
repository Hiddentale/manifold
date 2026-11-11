use std::path::PathBuf;
use crate::server::storage::world_meta::WorldMeta;

pub enum GameState {
    TitleScreen,
    WorldSelect {
        worlds: Vec<(PathBuf, WorldMeta)>,
    },
    CreateWorld {
        name: String,
        seed_text: String,
    },
    PreGenerating {
        world_dir: PathBuf,
        loaded: usize,
        total: usize,
    },
    EnteringWorld {
        world_dir: PathBuf,
    },
    Playing,
}

impl GameState {
    pub fn is_menu(&self) -> bool {
        !matches!(self, GameState::Playing)
    }
}
