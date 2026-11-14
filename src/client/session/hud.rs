use crate::client::{
    render::graphical_core::{
        camera::{Camera, EyeMatrices},
        vulkan_object::VulkanApplication,
    },
    session::client_session::Session,
};
use winit::window::Window;
pub struct FpsCounter {
    frames: u32,
    elapsed: f32,
    display_fps: u32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frames: 0,
            elapsed: 0.0,
            display_fps: 0,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.frames += 1;
        self.elapsed += dt;
        if self.elapsed >= 0.5 {
            self.display_fps = (self.frames as f32 / self.elapsed) as u32;
            self.frames = 0;
            self.elapsed = 0.0;
        }
    }

    fn display(&self) -> String {
        format!("{} fps", self.display_fps)
    }
}

pub fn draw_ui(
    session: &mut Session,
    camera: &Camera,
    fps_counter: &FpsCounter,
    application: &mut VulkanApplication,
    user_window: &Window,
    eyes: EyeMatrices,
) -> anyhow::Result<()> {
    application.ui.begin_frame();
    application.ui.draw_text(&fps_counter.display(), 4.0, 4.0, 16.0, [1.0, 1.0, 1.0, 0.8]);
    let pos_text = format!(
        "pos=({:.1},{:.1},{:.1})",
        session.local_player.body.x, session.local_player.body.y, session.local_player.body.z
    );
    application.ui.draw_text(&pos_text, 4.0, 22.0, 14.0, [1.0, 1.0, 1.0, 0.8]);
    unsafe { application.render_frame(&user_window, &camera, &eyes) }
}
