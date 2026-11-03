use crate::shared::grid::ChunkPos;

pub struct ClientChunkChanges {
    pub loaded_chunks: Vec<ChunkPos>,
    pub unloaded_chunks: Vec<ChunkPos>,
}
