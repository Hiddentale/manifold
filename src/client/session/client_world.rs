use super::chunk_changes::ClientChunkChanges;
use crate::client::render::chunk_source::ChunkSource;
use crate::shared::protocol::Event;
use crate::shared::{chunk::Chunk, grid::ChunkPos};
use std::collections::HashMap;
use std::sync::Arc;

/// All the data a player's PC can see
pub struct ClientWorld {
    chunks: HashMap<ChunkPos, Arc<Chunk>>,
}

impl ClientWorld {
    pub fn new() -> ClientWorld {
        Self { chunks: HashMap::new() }
    }
    pub fn apply(&mut self, event: Event, chunk_changes: &mut ClientChunkChanges) {
        match event {
            Event::ChunkLoaded { position, chunk } => {
                self.chunks.insert(position, chunk);
                chunk_changes.loaded_chunks.push(position);
            }
            Event::ChunkUnloaded { position } => {
                self.chunks.remove(&position);
                chunk_changes.unloaded_chunks.push(position);
            }
        }
    }
}

impl ChunkSource for ClientWorld {
    fn get_chunk_at(&self, position: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&position).map(Arc::as_ref)
    }
}
