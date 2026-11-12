use super::player::LocalPlayer;
use super::world_replica::WorldReplica;
use crate::shared::protocol::{ClientIntent, ServerEvent};

use std::sync::mpsc::{Receiver, Sender};

pub struct Session {
    pub local_player: LocalPlayer,
    pub physics_accumulator: f32,
    pub world_replica: WorldReplica,
    pub networking_handle: (Sender<ClientIntent>, Receiver<ServerEvent>),
}

impl Session {
    pub fn new(sender: Sender<ClientIntent>, receiver: Receiver<ServerEvent>) -> Self {
        Session {
            local_player: LocalPlayer::new(),
            physics_accumulator: 0.0,
            world_replica: WorldReplica::new(),
            networking_handle: (sender, receiver),
        }
    }
}
