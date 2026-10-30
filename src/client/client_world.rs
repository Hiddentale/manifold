use crate::simulation::protocol::Event;
use crate::voxel::world::ChunkChanges;
use crate::voxel::{chunk::Chunk, chunk_generator::ChunkGenerator, grid::ChunkPos};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ClientWorld {
    chunks: HashMap<ChunkPos, Arc<Chunk>>,
}

impl ClientWorld {
    pub fn new(&mut self) -> ClientWorld {
        Self { chunks: HashMap::new() }
    }
    pub fn apply(&mut self, event: Event, chunk_changes: &mut ChunkChanges) {
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
