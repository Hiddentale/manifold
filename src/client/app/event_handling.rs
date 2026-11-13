use super::game_state::GameState;
use super::menu::MenuState;
use crate::{
    client::render::graphical_core::{camera::Camera, input::InputState, vulkan_object::VulkanApplication},
    client::session::hud::FpsCounter,
};
use std::time::Instant;
use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

pub struct ClientApplication {
    pub game_state: GameState,
    pub input: InputState,
    pub fps_counter: FpsCounter,
    pub camera: Camera,
    pub last_frame: Instant,
    pub application: VulkanApplication,
    pub user_window: Window,
    pub cursor_position: [f32; 2],
    pub minimized: bool,
    pub menu_click: bool,
    pub destroy_application: bool,
}

impl ClientApplication {
    pub fn new(event_handler: &EventLoopWindowTarget<()>, game_title: &str) -> anyhow::Result<ClientApplication> {
        let user_window = WindowBuilder::new()
            .with_title(game_title)
            .with_inner_size(event_handler.primary_monitor().unwrap().size())
            .build(event_handler)?;

        let vulkan_application = unsafe { VulkanApplication::create_vulkan_application(&user_window) }?;

        let event_info = ClientApplication {
            game_state: GameState::Menu(MenuState::TitleScreen),
            input: InputState::new(),
            fps_counter: FpsCounter::new(),
            camera: Camera::default(),
            last_frame: Instant::now(),
            application: vulkan_application,
            user_window,
            cursor_position: [0.0; 2],
            minimized: false,
            menu_click: false,
            destroy_application: false,
        };
        Ok(event_info)
    }
}

pub fn handle_wait_event(event_info: &mut ClientApplication) {
    const MAX_PHYSICS_CATCHUP: f32 = 0.25;
    const PHYSICS_TICK: f32 = 1.0 / 60.0;
    let now = Instant::now();
    let delta_time = (now - event_info.last_frame).as_secs_f32();
    event_info.last_frame = now;
    event_info.fps_counter.tick(delta_time);

    match &mut event_info.game_state {
        GameState::Menu(menu_state) => {}
        GameState::Playing(session) => {} //     GameState::Playing => {
                                          //         let chunk_changes = event_info
                                          //             .world
                                          //             .as_mut()
                                          //             .map(|world| world.update(event_info.player.body.world_position().as_dvec3(), WORLD_DISTANCE));
                                          //         //
                                          //         //
                                          //         //
                                          //         //
                                          //         event_info.input.apply_camera_rotation(&mut event_info.player);
                                          //         event_info.physics_accumulator = (event_info.physics_accumulator + delta_time).min(MAX_PHYSICS_CATCHUP);

                                          //         let Some(world) = &event_info.world else { return };
                                          //         while event_info.physics_accumulator >= PHYSICS_TICK {
                                          //             let local_p = world.metric.sample_metric_at_pos(event_info.player.body.world_position()).minkowski_exponent;
                                          //             event_info.input.tick_movement(&mut event_info.player, &world, PHYSICS_TICK, local_p);
                                          //             event_info.physics_accumulator -= PHYSICS_TICK;
                                          //         }
                                          //         event_info.camera.sync_from_player(&event_info.player);
                                          //         event_info.input.take_left_click();
                                          //         event_info.input.take_right_click();
                                          //     }
                                          //     GameState::PreGenerating { .. } => {}
                                          //     GameState::EnteringWorld { .. } => {
                                          //         if let GameState::EnteringWorld { world_dir, seed } = std::mem::replace(&mut event_info.game_state, GameState::Playing) {
                                          //             if let Err(e) = unsafe { event_info.application.enter_world(&world_dir, seed) } {
                                          //                 eprintln!("Failed to enter world: {e}");
                                          //                 event_info.game_state = GameState::TitleScreen;
                                          //             } else {
                                          //                 grab_cursor(&event_info.user_window);
                                          //                 event_info.camera = Camera::default();
                                          //                 event_info.player = LocalPlayer::new();
                                          //             }
                                          //         }
                                          //     }
                                          //     _ => {}
                                          // }
    }
    event_info.user_window.request_redraw();
}
