#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BlockType {
    Air = 0,
    Grass = 1,
    Dirt = 2,
    Stone = 3,
    Water = 4,
    Sand = 5,
    Snow = 6,
    Gravel = 7,
}

impl BlockType {
    pub fn is_opaque(self) -> bool {
        !matches!(self, BlockType::Air)
    }

    /// Bitmask where bit `i` is set if block type `i` is opaque.
    /// Used by shaders for GPU-side face culling.
    pub fn opaque_mask() -> u32 {
        let mut mask = 0u32;
        for i in 0..=7u8 {
            // Safety: repr(u8) guarantees valid transmute for 0..=7
            let block: BlockType = unsafe { std::mem::transmute(i) };
            if block.is_opaque() {
                mask |= 1 << i;
            }
        }
        mask
    }
}

pub struct BlockProperties {
    opaque: bool,
    solid: bool,
    hardness: u32,
    light_emission: u32,
    falls: u32,
    top_texture: u32,
    side_texture: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_ids_match_format() {
        assert_eq!(BlockType::Air as u8, 0);
        assert_eq!(BlockType::Grass as u8, 1);
        assert_eq!(BlockType::Dirt as u8, 2);
        assert_eq!(BlockType::Stone as u8, 3);
        assert_eq!(BlockType::Water as u8, 4);
        assert_eq!(BlockType::Sand as u8, 5);
        assert_eq!(BlockType::Snow as u8, 6);
        assert_eq!(BlockType::Gravel as u8, 7);
    }
}
