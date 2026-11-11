pub fn handle_window_event(event: WindowEvent, event_info: &mut EventInfo, current_window: &EventLoopWindowTarget<()>) -> anyhow::Result<()> {
    match event {
        WindowEvent::CloseRequested => {
            exit_program(&mut event_info.destroy_application, current_window, &mut event_info.application);
        }
        WindowEvent::Resized(size) => {
            if size.width == 0 || size.height == 0 {
                event_info.minimized = true;
            } else {
                event_info.minimized = false;
                event_info.application.resized = true;
            }
        }
        WindowEvent::CursorMoved { position, .. } => {
            event_info.cursor_position = [position.x as f32, position.y as f32];
        }
        WindowEvent::KeyboardInput { event: key_event, .. } => handle_keyboard_input(key_event, current_window, event_info),
        WindowEvent::MouseInput {
            state: ElementState::Pressed,
            button: MouseButton::Left,
            ..
        } => {
            if event_info.game_state.is_menu() {
                event_info.menu_click = true;
            } else {
                event_info.input.mouse_pressed(MouseButton::Left);
            }
        }
        WindowEvent::MouseInput {
            state: ElementState::Pressed,
            button,
            ..
        } if !event_info.game_state.is_menu() => {
            event_info.input.mouse_pressed(button);
        }
        WindowEvent::RedrawRequested => {
            if event_info.destroy_application || event_info.minimized {
                return Ok(());
            }
            let eyes = EyeMatrices::from_camera(&event_info.camera, event_info.application.swapchain_extent());
            let result = match &event_info.game_state {
                GameState::Playing => draw_ui(event_info, eyes),
                _ => {
                    let extent = event_info.application.swapchain_extent();
                    let screen_width = extent.width as f32;
                    let screen_height = extent.height as f32;
                    let clicked = event_info.menu_click;
                    event_info.menu_click = false;

                    event_info.application.ui.begin_frame();
                    draw_menu(
                        &mut event_info.application.ui,
                        &mut event_info.game_state,
                        screen_width,
                        screen_height,
                        event_info.cursor_position,
                        clicked,
                    );
                    unsafe { event_info.application.render_menu_frame(&event_info.user_window, &eyes) }
                }
            };
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
        if matches!(event_info.game_state, GameState::Playing) {
            event_info.input.accumulate_mouse_delta(dx, dy);
        }
    }
}

fn handle_keyboard_input(key_event: KeyEvent, current_window: &EventLoopWindowTarget<()>, event_info: &mut EventInfo) {
    if let PhysicalKey::Code(key_code) = key_event.physical_key {
        match &mut event_info.game_state {
            GameState::Playing => match key_event.state {
                ElementState::Pressed => {
                    if key_code == KeyCode::Escape {
                        unsafe { event_info.application.destroy_gpu_resources() };
                        event_info.game_state = GameState::TitleScreen;
                        release_cursor(&event_info.user_window);
                        return;
                    }
                    if key_code == KeyCode::KeyF && !key_event.repeat {
                        event_info.player.body.toggle_fly_mode();
                    }
                    if key_code == KeyCode::Space {
                        event_info.player.body.jump();
                    }
                    event_info.input.key_pressed(key_code);
                }
                ElementState::Released => event_info.input.key_released(key_code),
            },
            GameState::CreateWorld { name, seed_text } => {
                if key_event.state == ElementState::Pressed {
                    if key_code == KeyCode::Escape {
                        event_info.game_state = GameState::TitleScreen;
                        return;
                    }
                    if key_code == KeyCode::Backspace {
                        if !seed_text.is_empty() {
                            seed_text.pop();
                        } else {
                            name.pop();
                        }
                        return;
                    }
                    if key_code == KeyCode::Enter && !name.is_empty() {
                        let seed: u32 = seed_text.parse().unwrap_or_else(|_| rand_seed());
                        match unsafe {event_info.application.create_gpu_resources() }{
                            Ok(dir) => {
                                let stream_rd = WORLD_DISTANCE;
                                let side = (2 * stream_rd + 1) as usize;
                                let total = side * side;
                                event_info.game_state = GameState::PreGenerating {
                                    world_dir: ,
                                    loaded: 0,
                                    total,
                                };
                            }
                            Err(e) => eprintln!("Failed to create world: {e}"),
                        }
                        return;
                    }
                    if let Some(text) = &key_event.text {
                        for char in text.chars() {
                            if char.is_ascii_digit() {
                                if seed_text.len() < 10 {
                                    seed_text.push(char);
                                }
                            } else if (char.is_ascii_alphanumeric() || char == ' ' || char == '-' || char == '_') && name.len() < 24 {
                                name.push(char);
                            }
                        }
                    }
                }
            }
            _ => {
                if key_event.state == ElementState::Pressed && key_code == KeyCode::Escape {
                    match &event_info.game_state {
                        GameState::WorldSelect { .. } => event_info.game_state = GameState::TitleScreen,
                        GameState::TitleScreen => {
                            exit_program(&mut event_info.destroy_application, current_window, &mut event_info.application);
                        }
                        _ => {}
                    }
                }
            }
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
