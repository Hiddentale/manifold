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
        BLOCKS[self as usize].opaque
    }

    pub fn is_solid(self) -> bool {
        BLOCKS[self as usize].solid
    }

    /// Bitmask where bit `i` is set if block type `i` is opaque.
    /// Used by shaders for GPU-side face culling.
    pub fn opaque_mask() -> u32 {
        let mut mask = 0u32;
        for i in 0..BLOCKS.len() {
            if BLOCKS[i].opaque {
                mask |= 1 << i;
            }
        }
        mask
    }
}

struct BlockProperties {
    block_type: BlockType,
    opaque: bool,
    solid: bool,
    hardness: u32,
    light_emission: u32,
    can_fall: bool,
    top_texture: u32,
    side_texture: u32,
}

const BLOCKS: [BlockProperties; 8] = [
    BlockProperties {
        block_type: BlockType::Air,
        opaque: false,
        solid: false,
        hardness: 0,
        light_emission: 0,
        can_fall: false,
        top_texture: 0,
        side_texture: 0,
    },
    BlockProperties {
        block_type: BlockType::Grass,
        opaque: true,
        solid: true,
        hardness: 0,
        light_emission: 0,
        can_fall: false,
        top_texture: 0,
        side_texture: 0,
    },
    BlockProperties {
        block_type: BlockType::Dirt,
        opaque: true,
        solid: true,
        hardness: 0,
        light_emission: 0,
        can_fall: false,
        top_texture: 0,
        side_texture: 0,
    },
    BlockProperties {
        block_type: BlockType::Stone,
        opaque: true,
        solid: true,
        hardness: 0,
        light_emission: 0,
        can_fall: false,
        top_texture: 0,
        side_texture: 0,
    },
    BlockProperties {
        block_type: BlockType::Water,
        opaque: false,
        solid: false,
        hardness: 0,
        light_emission: 0,
        can_fall: false,
        top_texture: 0,
        side_texture: 0,
    },
    BlockProperties {
        block_type: BlockType::Sand,
        opaque: true,
        solid: true,
        hardness: 0,
        light_emission: 0,
        can_fall: true,
        top_texture: 0,
        side_texture: 0,
    },
    BlockProperties {
        block_type: BlockType::Snow,
        opaque: true,
        solid: true,
        hardness: 0,
        light_emission: 0,
        can_fall: true,
        top_texture: 0,
        side_texture: 0,
    },
    BlockProperties {
        block_type: BlockType::Gravel,
        opaque: true,
        solid: true,
        hardness: 0,
        light_emission: 0,
        can_fall: true,
        top_texture: 0,
        side_texture: 0,
    },
];

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

    // Increase bitmask space if number of blocktypes exceeds this number.
    #[test]
    fn total_blocks_is_less_than_33() {
        assert!(BLOCKS.len() < 33);
    }
}
