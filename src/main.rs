mod client;
mod server;
mod shared;
mod utils;
use crate::client::app::event_handling::{handle_wait_event, initialize_event_info};
use crate::client::app::window::{handle_device_event, handle_window_event, release_cursor};
use anyhow::Result;
use winit::{event::Event, event_loop::EventLoop};

fn main() -> Result<()> {
    initialize_error_handler();
    let event_handler = EventLoop::new()?;
    let mut event_info = initialize_event_info(&event_handler)?;
    release_cursor(&event_info.user_window);

    event_handler
        .run(move |event, current_window| match event {
            Event::WindowEvent { event, .. } => {
                let _ = handle_window_event(event, &mut event_info, current_window);
            }
            Event::DeviceEvent { event, .. } => {
                handle_device_event(event, &mut event_info);
            }
            Event::AboutToWait => {
                handle_wait_event(&mut event_info);
            }
            _ => (),
        })
        .expect("Main function crashed!");
    Ok(())
}

fn initialize_error_handler() {
    pretty_env_logger::init();
}
