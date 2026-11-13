use super::world::World;
use crate::shared::protocol::{ClientIntent, ServerEvent};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::{current, scope, Builder, JoinHandle};

struct Simulation {
    current_world: World,
    players: Vec<u32>,
    tick_counter: u64,
}

pub fn start_server(shutdown: Arc<AtomicBool>) -> JoinHandle<()> {
    let thread_builder = Builder::new().name("server".into());
    let server_thread = thread_builder
        .spawn(move || {
            let mut tick_count = 0;
            while !shutdown.as_ref().load(Ordering::Relaxed) {
                if tick_count % 100000000 == 0 {
                    println!("{:?}", current().name())
                }
                tick_count += 1;
            }
            println!("{:?}", tick_count)
        })
        .unwrap();
    server_thread
}

fn initialize_server_handler() -> (Sender<ClientIntent>, Receiver<ClientIntent>, Sender<ServerEvent>, Receiver<ServerEvent>) {
    let (intent_sender, intent_receiver) = channel::<ClientIntent>();
    let (event_sender, event_receiver) = channel::<ServerEvent>();
    (intent_sender, intent_receiver, event_sender, event_receiver)
}
