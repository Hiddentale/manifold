use super::world::World;
use crate::shared::protocol::{ClientIntent, ServerConnection, ServerEvent};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread::Builder;

struct Simulation {
    current_world: World,
    players: Vec<u32>,
    tick_count: u64,
}

pub fn start_server(seed: u32) -> ServerConnection {
    let mut simulation = Simulation {
        current_world: World::new(seed),
        players: Vec::new(),
        tick_count: 0,
    };
    let (intent_sender, intent_receiver) = channel::<ClientIntent>();
    let (event_sender, event_receiver) = channel::<ServerEvent>();
    let thread_builder = Builder::new().name("server".into());
    let join_handle = thread_builder
        .spawn(move || {
            loop {
                match intent_receiver.try_recv() {
                    Ok(ClientIntent::Shutdown) => break,
                    Err(TryRecvError::Empty) => (),
                    Err(TryRecvError::Disconnected) => break,
                    _ => (),
                }
                simulation.tick_count += 1;
                std::thread::sleep(std::time::Duration::from_millis(10));
                println!("{:?}", simulation.tick_count)
            }
            println!("{:?}", simulation.tick_count)
        })
        .unwrap();
    ServerConnection {
        join_handle,
        intent_sender,
        event_receiver,
    }
}

fn initialize_server_handler() -> (Sender<ClientIntent>, Receiver<ClientIntent>, Sender<ServerEvent>, Receiver<ServerEvent>) {
    let (intent_sender, intent_receiver) = channel::<ClientIntent>();
    let (event_sender, event_receiver) = channel::<ServerEvent>();
    (intent_sender, intent_receiver, event_sender, event_receiver)
}
