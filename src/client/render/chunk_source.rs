use crate::shared::{chunk::Chunk, grid::ChunkPos};

pub trait ChunkSource {
    fn get_chunk_at(&self, position: ChunkPos) -> Option<&Chunk>;
}
