use super::world::World;
use crate::shared::protocol::{ClientIntent, ServerEvent};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread::{Builder, JoinHandle};

struct Simulation {
    current_world: World,
    players: Vec<u32>,
    tick_count: u64,
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

fn loop_server(intents: Receiver<ClientIntent>, event_sender: Sender<ServerEvent>, simulation: &mut Simulation) {
    'server: loop {
        loop {
            match intents.try_recv() {
                Ok(ClientIntent::Shutdown) => break 'server,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break 'server,
                _ => (),
            }
        }
        simulation.tick_count += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("{:?}", simulation.tick_count)
}
