use super::chunk_changes::ClientChunkChanges;
use crate::shared::protocol::Event;
use crate::shared::{
    chunk::{Chunk, ChunkLookup},
    grid::ChunkPos,
};
use std::collections::HashMap;
use std::sync::Arc;

/// The client's replica of server-sent chunks
pub struct WorldReplica {
    chunks: HashMap<ChunkPos, Arc<Chunk>>,
}

impl WorldReplica {
    pub fn new() -> WorldReplica {
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

impl ChunkLookup for WorldReplica {
    fn get_chunk_at(&self, position: &ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&position).map(Arc::as_ref)
    }
}
