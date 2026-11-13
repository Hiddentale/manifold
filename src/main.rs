mod client;
mod server;
mod shared;
mod utils;
use crate::client::app::event_handling::{handle_wait_event, ClientApplication};
use crate::client::app::window::{handle_device_event, handle_window_event, release_cursor};
use anyhow::Result;
use winit::{event::Event, event_loop::EventLoop};

const APPLICATION_NAME: &str = "Manifold";

fn main() -> Result<()> {
    initialize_error_handler();
    let event_handler = EventLoop::new()?;
    let mut client_application = ClientApplication::new(&event_handler, APPLICATION_NAME)?;
    release_cursor(&client_application.user_window);

    event_handler
        .run(move |event, current_window| match event {
            Event::WindowEvent { event, .. } => {
                let _ = handle_window_event(event, &mut client_application, current_window);
            }
            Event::DeviceEvent { event, .. } => {
                handle_device_event(event, &mut client_application);
            }
            Event::AboutToWait => {
                handle_wait_event(&mut client_application);
            }
            _ => (),
        })
        .expect("Main function crashed!");
    Ok(())
}

fn initialize_error_handler() {
    pretty_env_logger::init();
}
