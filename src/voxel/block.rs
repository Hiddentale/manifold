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
    pub fn is_solid(self) -> bool {
        BLOCKS[self as usize].solid
    }
}

pub fn construct_gpu_block_properties(block_properties: &BlockProperties) -> GpuBlockProperties {
    GpuBlockProperties {
        opaque: block_properties.opaque as u32,
        solid: block_properties.solid as u32,
        hardness: block_properties.hardness,
        light_emission: block_properties.light_emission,
        can_fall: block_properties.can_fall as u32,
        top_texture: block_properties.top_texture,
        side_texture: block_properties.side_texture,
    }
}

#[repr(C)]
pub struct GpuBlockProperties {
    opaque: u32,
    solid: u32,
    hardness: u32,
    light_emission: u32,
    can_fall: u32,
    top_texture: u32,
    side_texture: u32,
}

const _: () = assert!(
    size_of::<GpuBlockProperties>() == 28,
    "GPU Block Properties layout has changed, update the matching struct in the shader file"
);

pub struct BlockProperties {
    block_type: BlockType,
    opaque: bool,
    solid: bool,
    hardness: u32,
    light_emission: u32,
    can_fall: bool,
    top_texture: u32,
    side_texture: u32,
}

pub const BLOCKS: [BlockProperties; 8] = [
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

const _: () = {
    let mut i: usize = 0;
    while i < BLOCKS.len() {
        assert!(
            BLOCKS[i].block_type as usize == i,
            "BLOCKS has changed, Blocktype enum values don't match BLOCKS index positions anymore"
        );
        i += 1;
    }
};

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

    #[test]
    fn conversion_copies_true_to_1_and_false_to_0() {
        let block_properties = BlockProperties {
            block_type: BlockType::Air,
            opaque: true,
            solid: false,
            hardness: 0,
            light_emission: 0,
            can_fall: false,
            top_texture: 0,
            side_texture: 0,
        };
        let gpu_block_properties = construct_gpu_block_properties(&block_properties);
        assert_eq!(gpu_block_properties.opaque, 1);
        assert_eq!(gpu_block_properties.solid, 0);
    }
}
