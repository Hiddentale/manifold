use super::world::World;
use crate::shared::protocol::{ClientIntent, JoinReply, JoinRequest, ServerConnection, ServerEvent};
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
    pub event_sender: Sender<ServerEvent>,
    pub intent_receiver: Receiver<ClientIntent>,
}

pub struct ServerHandle {
    pub join_handle: JoinHandle<()>,
    pub join_requester: Sender<JoinRequest>,
}

enum TickOutcome {
    Continue,
    Shutdown,
}

pub fn start_server(seed: u32) -> anyhow::Result<ServerHandle> {
    let mut simulation = Simulation {
        current_world: World::new(seed),
        players: Vec::new(),
        tick_count: 0,
    };

    let (join_requester, join_receiver) = channel::<JoinRequest>();
    let thread_builder = Builder::new().name("server".into());
    let join_handle = thread_builder.spawn(move || loop_server(join_receiver, &mut simulation))?;

    Ok(ServerHandle { join_handle, join_requester })
}

pub fn join_server() {}

fn loop_server(join_receiver: Receiver<JoinRequest>, simulation: &mut Simulation) {
    let mut next_tick = Instant::now() + TICK_DURATION;

    'server: loop {
        process_joins(&join_receiver, &mut simulation.players);
        loop {
            let time_left = next_tick.saturating_duration_since(Instant::now());
            if !simulation.players.is_empty() {
                for player in &simulation.players {
                    process_intents(time_left, player);
                }
            }
            // let tick_outcome = process_intents(time_left, intents);
            // if matches!(tick_outcome, TickOutcome::Shutdown) {
            //     break 'server;
            // } else if matches!(tick_outcome, TickOutcome::Continue) {
            //     break;
            // }
        }
        next_tick += TICK_DURATION;
        simulation.tick_count += 1;
    }
}

fn process_joins(join_receiver: &Receiver<JoinRequest>, client_connections: &mut Vec<ClientConnection>) {
    for join_request in join_receiver.try_iter() {
        let (intent_sender, intent_receiver) = channel::<ClientIntent>();
        let (event_sender, event_receiver) = channel::<ServerEvent>();
        let server_connection = ServerConnection {
            intent_sender,
            event_receiver,
        };
        join_request.temporary_sender.send(JoinReply::Accepted { server_connection }).unwrap();
        let client_connection = ClientConnection {
            intent_receiver,
            event_sender,
        };
        client_connections.push(client_connection);
    }
}

fn process_intents(time_left: Duration, intents: &ClientConnection) -> TickOutcome {
    match intents.recv_timeout(time_left) {
        Ok(ClientIntent::MovePlayer { position, velocity }) => TickOutcome::Continue,
        Ok(ClientIntent::BreakBlock { position }) => TickOutcome::Continue,
        Ok(ClientIntent::PlaceBlock { position }) => TickOutcome::Continue,
        Ok(ClientIntent::Shutdown) => TickOutcome::Shutdown,
        Err(RecvTimeoutError::Timeout) => TickOutcome::Continue,
        Err(RecvTimeoutError::Disconnected) => TickOutcome::Shutdown,
    }
}
