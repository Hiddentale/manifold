use super::world_replica::WorldReplica;
use super::player::LocalPlayer;
use crate::shared::protocol::{Event, Intent};

use std::sync::mpsc::{channel, Sender, Receiver};

pub struct Session {
    local_player: LocalPlayer,
    world_replica: WorldReplica,
    simulation_handle: (Sender<Intent>, Receiver<Event>)
}

impl Session {
    pub fn new(&self) -> Self {
        Session {
            local_player: LocalPlayer::new(),
            world_replica: WorldReplica::new(),
            simulation_handle: channel::<Intent>(),
        }
    }
}
