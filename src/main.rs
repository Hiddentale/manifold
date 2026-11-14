mod client;
mod server;
mod shared;
mod utils;
use crate::client::app::event_handling::{update_game_frame, ClientApplication};
use crate::client::app::window::{handle_device_event, handle_window_event, release_cursor};
use crate::server::simulation::start_server;
use crate::shared::protocol::ClientIntent;
use anyhow::Result;
use winit::{event::Event, event_loop::EventLoop};

const APPLICATION_NAME: &str = "Manifold";

fn main() -> Result<()> {
    initialize_error_handler();
    let event_handler = EventLoop::new()?;
    let mut client_application = ClientApplication::new(&event_handler, APPLICATION_NAME)?;
    release_cursor(&client_application.user_window);
    let server_connection = start_server(12345);

    event_handler
        .run(move |event, current_window| match event {
            Event::WindowEvent { event, .. } => {
                let shutdown_application = handle_window_event(event, &mut client_application, current_window);
                if shutdown_application {
                    server_connection.intent_sender.send(ClientIntent::Shutdown).unwrap();
                }
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
    server_connection.join_handle.join().unwrap();
    Ok(())
}

fn initialize_error_handler() {
    pretty_env_logger::init();
}
