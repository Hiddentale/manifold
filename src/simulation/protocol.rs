use crate::voxel::{chunk::Chunk, grid::ChunkPos};
use glam::DVec3;
use std::sync::Arc;

pub enum Intent {
    MovePlayer { position: DVec3, velocity: u32 },
    BreakBlock { position: DVec3 },
    PlaceBlock { position: DVec3 },
}

pub enum Event {
    ChunkLoaded { position: ChunkPos, chunk: Arc<Chunk> },
    ChunkUnloaded { position: ChunkPos },
}
