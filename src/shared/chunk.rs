use super::block::BlockType;
use super::grid::ChunkPos;
use crate::shared::grid::world_to_chunk_local;
use glam::DVec3;

pub const CHUNK_SIZE: usize = 16;
const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Clone)]
pub struct Chunk {
    blocks: [BlockType; CHUNK_VOLUME],
}

impl Chunk {
    pub fn new(fill: BlockType) -> Self {
        Self {
            blocks: [fill; CHUNK_VOLUME],
        }
    }

    pub fn get_block_at(&self, x: usize, y: usize, z: usize) -> BlockType {
        self.blocks[coords_to_index(x, y, z)]
    }

    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        self.blocks[coords_to_index(x, y, z)] = block;
    }

    /// Raw block data as bytes for GPU upload.
    pub fn as_bytes(&self) -> &[u8; CHUNK_VOLUME] {
        unsafe { &*(self.blocks.as_ptr() as *const [u8; CHUNK_VOLUME]) }
    }

    /// True if every block in this chunk is `BlockType::Air`.
    pub fn contains_only_air(&self) -> bool {
        let blocks = self.as_bytes();
        blocks.iter().all(|&blocktype| blocktype == BlockType::Air as u8)
    }

    pub fn contains_no_air(&self) -> bool {
        let blocks = self.as_bytes();
        blocks.iter().all(|&blocktype| blocktype != BlockType::Air as u8)
    }
}

pub trait ChunkLookup {
    fn get_chunk_at(&self, position: &ChunkPos) -> Option<&Chunk>;

    fn block_solid_at(&self, block_x: i32, block_y: i32, block_z: i32) -> bool {
        let (chunk_pos, x, y, z) = world_to_chunk_local(DVec3::new(block_x as f64, block_y as f64, block_z as f64));
        if let Some(chunk) = self.get_chunk_at(&chunk_pos) {
            chunk.get_block_at(x, y, z).is_solid()
        } else {
            true
        }
    }
}

fn coords_to_index(x: usize, y: usize, z: usize) -> usize {
    x + z * CHUNK_SIZE + y * CHUNK_SIZE * CHUNK_SIZE
}
