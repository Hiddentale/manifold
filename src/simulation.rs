use crate::voxel::world::World;

struct Simulation {
    current_world: World,
    players: Vec<u32>,
    tick_counter: u64,
}
