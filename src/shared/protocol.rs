use super::{chunk::Chunk, grid::ChunkPos};
use glam::DVec3;
use std::sync::Arc;

#[derive(PartialEq)]
pub enum ClientIntent {
    MovePlayer { position: DVec3, velocity: u32 },
    BreakBlock { position: DVec3 },
    PlaceBlock { position: DVec3 },
}

#[derive(PartialEq)]
pub enum ServerEvent {
    ChunkLoaded { position: ChunkPos, chunk: Arc<Chunk> },
    ChunkUnloaded { position: ChunkPos },
}
