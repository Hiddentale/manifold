use super::{chunk::Chunk, grid::ChunkPos};
use glam::DVec3;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

pub enum ClientIntent {
    MovePlayer { position: DVec3, velocity: u32 },
    BreakBlock { position: DVec3 },
    PlaceBlock { position: DVec3 },
    Shutdown,
}

pub enum ServerEvent {
    ChunkLoaded { position: ChunkPos, chunk: Arc<Chunk> },
    ChunkUnloaded { position: ChunkPos },
}

pub struct ServerConnection {
    pub intent_sender: Sender<ClientIntent>,
    pub event_receiver: Receiver<ServerEvent>,
}

pub struct JoinRequest {
    pub temporary_sender: Sender<JoinReply>,
}

pub enum JoinReply {
    Accepted { server_connection: ServerConnection },
    Denied,
}
