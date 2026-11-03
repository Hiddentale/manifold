//! Per-frame rendering: world frames, menu frames, and chunk streaming.

use super::super::camera::{update_uniform_buffer, Camera, EyeMatrices};
use super::super::{
    commands::{begin_render_pass, draw_sky, record_vertex_pull_command_buffer},
    compute_cull::CullPushConstants,
    frustum::Frustum,
    MAX_FRAMES_IN_FLIGHT,
};
use super::{VulkanApplication, TIMING_QUERY_COUNT};
use crate::client::render::chunk_source::ChunkSource;
use crate::client::session::client_world::ClientWorld;
use crate::shared::grid::ChunkPos;
use crate::voxel::world::{TERRAIN_MAX_CY, TERRAIN_MIN_CY};
use vulkan_rust::vk;
use winit::window::Window;

fn neighbors_all_completely_solid(world: &ClientWorld, chunk_pos: ChunkPos) -> bool {
    let neighbor_solid = |neighbor_pos: ChunkPos| -> bool {
        match world.get_chunk_at(neighbor_pos) {
            Some(chunk) => chunk.contains_no_air(),
            None => (TERRAIN_MIN_CY..=TERRAIN_MAX_CY).contains(&neighbor_pos.y),
        }
    };
    let neighbors = [
        chunk_pos.offset(1, 0, 0),
        chunk_pos.offset(-1, 0, 0),
        chunk_pos.offset(0, 1, 0),
        chunk_pos.offset(0, -1, 0),
        chunk_pos.offset(0, 0, 1),
        chunk_pos.offset(0, 0, -1),
    ];
    neighbors.iter().all(|&neighbor_pos| neighbor_solid(neighbor_pos))
}

impl VulkanApplication {
    /// Acquires a swapchain image, submits the command buffer, and presents the result.
    /// Also keeps check of performance of every specific step in the rendering.
    ///
    /// # Safety
    /// Calls unsafe Vulkan queue and synchronization APIs.
    pub unsafe fn render_frame(&mut self, window: &Window, camera: &Camera, eyes: &EyeMatrices) -> anyhow::Result<()> {
        let image_index = match self.acquire_next_image(window)? {
            Some(index) => index,
            None => return Ok(()),
        };
        update_uniform_buffer(&self.data, eyes)?;

        let world_resources = self.scene_resources.as_ref().unwrap();
        let frustum = if eyes.is_stereo() {
            Frustum::combined_stereo(&eyes.view_projection[0], &eyes.view_projection[1])
        } else {
            Frustum::from_view_projection(&eyes.primary_vp())
        };
        let cull_push = CullPushConstants {
            planes: [
                frustum.plane(0),
                frustum.plane(1),
                frustum.plane(2),
                frustum.plane(3),
                frustum.plane(4),
                frustum.plane(5),
            ],
            camera_pos: camera.position.to_array(),
            chunk_count: world_resources.voxel_pool.chunk_count(),
            screen_size: [self.data.swapchain_extent.width as f32, self.data.swapchain_extent.height as f32],
            phase: 1,
            stereo: if eyes.is_stereo() { 1 } else { 0 },
            _pad: [0.0; 2],
        };
        record_vertex_pull_command_buffer(
            &self.device,
            &self.data,
            image_index,
            &world_resources.face_gen_pipeline,
            &world_resources.vertex_pull_pipeline,
            &world_resources.cull_compact_pipeline,
            &world_resources.voxel_pool,
            &self.depth_pyramid_pipeline,
            &cull_push,
            self.depth_pyramid_needs_init,
            &self.ui,
            self.timing_query_pool,
        )?;
        self.depth_pyramid_needs_init = false;

        self.submit_command_buffer(image_index)?;

        self.present_frame(image_index, window)?;

        // Read back GPU timestamps from THIS frame (blocks until done; the
        // CPU is normally idle during this window because acquire blocks
        // anyway). Convert tick deltas → ms via timestamp_period.
        let mut ts = [0u64; TIMING_QUERY_COUNT as usize];
        let _ = self.device.get_query_pool_results(
            self.timing_query_pool,
            0,
            TIMING_QUERY_COUNT,
            std::mem::size_of_val(&ts),
            ts.as_mut_ptr() as *mut std::ffi::c_void,
            std::mem::size_of::<u64>() as u64,
            vk::QueryResultFlags::_64 | vk::QueryResultFlags::WAIT,
        );
        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;
        Ok(())
    }

    /// Render a menu frame (sky background + UI overlay).
    pub unsafe fn render_menu_frame(&mut self, window: &Window, eyes: &EyeMatrices) -> anyhow::Result<()> {
        let image_index = match self.acquire_next_image(window)? {
            Some(index) => index,
            None => return Ok(()),
        };
        update_uniform_buffer(&self.data, eyes)?;

        let cmd = self.data.command_buffers[image_index];
        self.device.reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())?;
        self.device.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::builder())?;

        begin_render_pass(&self.device, cmd, &self.data, image_index);
        draw_sky(&self.device, cmd, &self.data);
        let screen = [self.data.swapchain_extent.width as f32, self.data.swapchain_extent.height as f32];
        self.ui.record(&self.device, cmd, screen);
        self.device.cmd_end_render_pass(cmd);
        self.device.end_command_buffer(cmd)?;

        self.submit_command_buffer(image_index)?;
        self.present_frame(image_index, window)?;
        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;
        Ok(())
    }
}
