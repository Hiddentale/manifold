use super::world::World;
use crate::shared::protocol::{ClientIntent, ServerEvent};
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::thread::{Builder, JoinHandle};
use std::time::{Duration, Instant};

const NANOSECONDS_PER_SECOND: u64 = 64;
const AMOUNT_OF_NANOSECONDS: u64 = 1_000_000_000;
const TICK_DURATION: Duration = Duration::from_nanos(AMOUNT_OF_NANOSECONDS / NANOSECONDS_PER_SECOND);

struct Simulation {
    current_world: World,
    players: Vec<ClientConnection>,
    tick_count: u64,
}

pub struct ClientConnection {
    pub player_id: u32,
    pub event_sender: Sender<ServerEvent>,
    pub intent_receiver: Receiver<ClientIntent>,
}

pub struct ServerConnection {
    pub join_handle: JoinHandle<()>,
    pub intent_sender: Sender<ClientIntent>,
    pub event_receiver: Receiver<ServerEvent>,
}

pub fn start_server(seed: u32) -> anyhow::Result<ServerConnection> {
    let mut simulation = Simulation {
        current_world: World::new(seed),
        players: Vec::new(),
        tick_count: 0,
    };
    let (intent_sender, intent_receiver) = channel::<ClientIntent>();
    let (event_sender, event_receiver) = channel::<ServerEvent>();
    let thread_builder = Builder::new().name("server".into());
    let join_handle = thread_builder.spawn(move || loop_server(intent_receiver, event_sender, &mut simulation))?;
    Ok(ServerConnection {
        join_handle,
        intent_sender,
        event_receiver,
    })
}

pub fn join_server() {}

fn loop_server(intents: Receiver<ClientIntent>, event_sender: Sender<ServerEvent>, simulation: &mut Simulation) {
    let mut next_tick = Instant::now() + TICK_DURATION;
    'server: loop {
        loop {
            let time_left = next_tick.saturating_duration_since(Instant::now());
            match intents.recv_timeout(time_left) {
                Ok(ClientIntent::MovePlayer { position, velocity }) => break,
                Ok(ClientIntent::BreakBlock { position }) => break,
                Ok(ClientIntent::PlaceBlock { position }) => break,
                Ok(ClientIntent::Shutdown) => break 'server,
                Err(RecvTimeoutError::Timeout) => break,
                Err(RecvTimeoutError::Disconnected) => break 'server,
                _ => (),
            }
        }
        next_tick += TICK_DURATION;
        simulation.tick_count += 1;
    }
}
