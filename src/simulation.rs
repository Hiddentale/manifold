use crate::voxel::world::World;
pub mod protocol;

struct Simulation {
    current_world: World,
    players: Vec<u32>,
    tick_counter: u64,
}
