pub mod block;
pub mod chunk;
pub mod grid;
pub mod metric;
pub mod player_collision;
pub mod protocol;

use glam::Vec3;

pub const WORLD_DISTANCE: i32 = 10;
pub const WORLD_UP_VECTOR: Vec3 = Vec3::new(0.0, 1.0, 0.0);
pub const CAMERA_FORWARD_OFFSET: f32 = 0.25;
pub const TERRAIN_MIN_CY: i32 = 0;
pub const TERRAIN_MAX_CY: i32 = 47;
