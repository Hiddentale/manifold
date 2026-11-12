use super::{event_handling::EventInfo, game_state::GameState};
use crate::client::{
    app::{
        game_state::GameState::{Menu, Playing},
        menu::{draw_menu, MenuState},
    },
    render::graphical_core::{camera::EyeMatrices, vulkan_object::VulkanApplication},
    session::hud::draw_ui,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{DeviceEvent, ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
    window::CursorGrabMode,
};

const FLY_MODE_KEY: KeyCode = KeyCode::KeyF;
const JUMP_KEY: KeyCode = KeyCode::Space;

pub fn handle_window_event(event: WindowEvent, event_info: &mut EventInfo, current_window: &EventLoopWindowTarget<()>) -> anyhow::Result<()> {
    match event {
        WindowEvent::CloseRequested => {
            exit_program(&mut event_info.destroy_application, current_window, &mut event_info.application);
        }
        WindowEvent::Resized(size) => resize_or_minimize_screen(size, event_info),
        WindowEvent::CursorMoved { position, .. } => adjust_cursor_position(position, event_info),
        WindowEvent::KeyboardInput { event: key_event, .. } => handle_keyboard_input(key_event, event_info, current_window),
        WindowEvent::MouseInput {
            state: ElementState::Pressed,
            button,
            ..
        } => handle_mouse_input(button, event_info),
        WindowEvent::RedrawRequested => {
            if event_info.destroy_application || event_info.minimized {
                return Ok(());
            }
            let result = handle_redraw_request(event_info);
            if let Err(e) = result {
                eprintln!("Render error: {e}");
                exit_program(&mut event_info.destroy_application, current_window, &mut event_info.application);
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_device_event(event: DeviceEvent, event_info: &mut EventInfo) {
    if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
        if matches!(event_info.game_state, GameState::Playing(_)) {
            event_info.input.accumulate_mouse_delta(dx, dy);
        }
    }
}

fn handle_keyboard_input(key_event: KeyEvent, event_info: &mut EventInfo, current_window: &EventLoopWindowTarget<()>) {
    if let PhysicalKey::Code(key_code) = key_event.physical_key {
        match &mut event_info.game_state {
            GameState::Playing(session) => match key_event.state {
                ElementState::Pressed => {
                    if key_code == KeyCode::Escape {
                        unsafe { event_info.application.destroy_gpu_resources() };
                        event_info.game_state = GameState::Menu(MenuState::TitleScreen);
                        release_cursor(&event_info.user_window);
                        return;
                    }
                    if key_code == FLY_MODE_KEY && !key_event.repeat {
                        session.local_player.body.toggle_fly_mode();
                    }
                    if key_code == JUMP_KEY {
                        session.local_player.body.jump();
                    }
                    event_info.input.key_pressed(key_code);
                }
                ElementState::Released => event_info.input.key_released(key_code),
            },
            GameState::Menu(menu_state) => match key_event.state {
                ElementState::Pressed => {
                    if key_code == KeyCode::Escape && matches!(menu_state, MenuState::TitleScreen) {
                        exit_program(&mut event_info.destroy_application, current_window, &mut event_info.application);
                        return;
                    }
                    if key_code == KeyCode::Escape && (matches!(menu_state, MenuState::Singleplayer) || matches!(menu_state, MenuState::Multiplayer))
                    {
                        *menu_state = MenuState::TitleScreen;
                    }
                }
                ElementState::Released => event_info.input.key_released(key_code),
            },
        }
    }
}

fn grab_cursor(window: &winit::window::Window) {
    if window.set_cursor_grab(CursorGrabMode::Confined).is_err() {
        let _ = window.set_cursor_grab(CursorGrabMode::Locked);
    }
    window.set_cursor_visible(false);
}

pub fn release_cursor(window: &winit::window::Window) {
    let _ = window.set_cursor_grab(CursorGrabMode::None);
    window.set_cursor_visible(true);
}

fn exit_program(destroy_application: &mut bool, current_window: &EventLoopWindowTarget<()>, application: &mut VulkanApplication) {
    *destroy_application = true;
    current_window.exit();
    unsafe { application.destroy_vulkan_application() }
}

fn resize_or_minimize_screen(size: PhysicalSize<u32>, event_info: &mut EventInfo) {
    if size.width == 0 || size.height == 0 {
        event_info.minimized = true;
    } else {
        event_info.minimized = false;
        event_info.application.resized = true;
    }
}

fn adjust_cursor_position(position: PhysicalPosition<f64>, event_info: &mut EventInfo) {
    event_info.cursor_position = [position.x as f32, position.y as f32];
}

fn handle_mouse_input(button: MouseButton, event_info: &mut EventInfo) {
    match event_info.game_state {
        Menu(_) => {
            if button == MouseButton::Left {
                event_info.menu_click = true;
            }
        }
        Playing(_) => {
            event_info.input.mouse_pressed(button);
        }
    }
}

fn handle_redraw_request(event_info: &mut EventInfo) -> anyhow::Result<()> {
    let eyes = EyeMatrices::from_camera(&event_info.camera, event_info.application.swapchain_extent());
    let result = match &mut event_info.game_state {
        Playing(session) => draw_ui(
            session,
            &event_info.camera,
            &event_info.fps_counter,
            &mut event_info.application,
            &event_info.user_window,
            eyes,
        ),
        Menu(menu_state) => {
            let extent = event_info.application.swapchain_extent();
            let screen_width = extent.width as f32;
            let screen_height = extent.height as f32;
            let clicked = event_info.menu_click;
            event_info.menu_click = false;

            event_info.application.ui.begin_frame();
            draw_menu(
                &mut event_info.application.ui,
                menu_state,
                screen_width,
                screen_height,
                event_info.cursor_position,
                clicked,
            );
            unsafe { event_info.application.render_menu_frame(&event_info.user_window, &eyes) }
        }
    };
    result
}
