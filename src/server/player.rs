use crate::shared::player_collision::{Body, Orientation};
use crate::shared::{chunk::Chunk, grid::ChunkPos};
use std::collections::HashMap;
use std::sync::Arc;
pub struct ServerPlayer {
    id: u32,
    body: Body,
    orientation: Orientation,
    streamed_chunks: HashMap<ChunkPos, Arc<Chunk>>,
}
