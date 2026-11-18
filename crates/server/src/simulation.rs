use super::world::World;
use shared::protocol::{ClientIntent, JoinReply, JoinRequest, ServerConnection, ServerEvent};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
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

#[must_use]
enum TickOutcome {
    Continue,
    Shutdown,
    Disconnected,
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

fn loop_server(join_receiver: Receiver<JoinRequest>, simulation: &mut Simulation) {
    let mut next_tick = Instant::now() + TICK_DURATION;
    loop {
        process_joins(&join_receiver, &mut simulation.players);
        let mut disconnected = Vec::new();
        for (index, player) in simulation.players.iter().enumerate() {
            match process_intents(player) {
                TickOutcome::Continue => (),
                TickOutcome::Disconnected => disconnected.push(index),
                TickOutcome::Shutdown => break,
            }
        }
        disconnect_players(disconnected, simulation);
        sleep_loop(&mut next_tick, simulation);
    }
}

fn disconnect_players(disconnected: Vec<usize>, simulation: &mut Simulation) {
    for index in disconnected.into_iter().rev() {
        simulation.players.remove(index);
    }
}

fn sleep_loop(next_tick: &mut Instant, simulation: &mut Simulation) {
    let time_left = next_tick.saturating_duration_since(Instant::now());
    std::thread::sleep(time_left);
    *next_tick += TICK_DURATION;
    simulation.tick_count += 1;
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
    // Someone could overload the server by sending millions of join requests.
}

const MAX_INTENTS_PER_TICK: u32 = 100;
fn process_intents(connection: &ClientConnection) -> TickOutcome {
    for _ in 0..MAX_INTENTS_PER_TICK {
        match connection.intent_receiver.try_recv() {
            Ok(ClientIntent::MovePlayer { position, velocity }) => { // Send ServerEvent to Client that player move is accepted
            }
            Ok(ClientIntent::BreakBlock { position }) => { // Send ServerEvent to Client that player block break is accepted
            }
            Ok(ClientIntent::PlaceBlock { position }) => { // Send ServerEvent to Client that player block place is accepted
            }
            Ok(ClientIntent::Shutdown) => return TickOutcome::Shutdown,
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => return TickOutcome::Disconnected,
        }
    }
    TickOutcome::Continue
}
