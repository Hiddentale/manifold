mod client;
mod server;
mod shared;
mod utils;
use crate::client::app::event_handling::{update_game_frame, ClientApplication};
use crate::client::app::window::{handle_device_event, handle_window_event, release_cursor};
use crate::server::simulation::start_server;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winit::{event::Event, event_loop::EventLoop};

const APPLICATION_NAME: &str = "Manifold";

fn main() -> Result<()> {
    initialize_error_handler();
    let event_handler = EventLoop::new()?;
    let mut client_application = ClientApplication::new(&event_handler, APPLICATION_NAME)?;
    release_cursor(&client_application.user_window);
    let shutdown_thread = Arc::new(AtomicBool::new(false));
    let shutdown = Arc::clone(&shutdown_thread);
    let thread_handle = start_server(shutdown);

    event_handler
        .run(move |event, current_window| match event {
            Event::WindowEvent { event, .. } => {
                handle_window_event(event, &mut client_application, current_window);
            }
            Event::DeviceEvent { event, .. } => {
                handle_device_event(event, &mut client_application);
            }
            Event::AboutToWait => {
                update_game_frame(&mut client_application);
            }
            _ => (),
        })
        .expect("Main function crashed!");
    shutdown_thread.store(true, Ordering::Relaxed);
    thread_handle.join().unwrap();
    Ok(())
}

fn initialize_error_handler() {
    pretty_env_logger::init();
}
