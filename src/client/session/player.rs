//! Local player state.
use crate::shared::player_collision::{Body, Orientation};
use crate::shared::CAMERA_FORWARD_OFFSET;
use crate::shared::WORLD_UP_VECTOR;
use glam::Vec3;

pub struct LocalPlayer {
    pub body: Body,
    pub orientation: Orientation,
}

impl LocalPlayer {
    pub fn new() -> Self {
        let mut player = Self {
            body: Body {
                x: 0.0,
                y: 90.0,
                z: 0.0,
                height: 1.7,
                half_width: 0.3,
                forward: Vec3::ZERO,
                right: Vec3::ZERO,
                velocity: Vec3::ZERO,
                on_ground: true,
                fly_mode: false,
            },
            orientation: Orientation { yaw: 0.0, pitch: 0.0 },
        };
        player.orientation.update_look_vectors(&mut player.body);
        player
    }

    /// Cartesian eye position.
    pub fn camera_position(&self) -> Vec3 {
        const EYE_HEIGHT: f32 = 1.6; // Make this depend on height in future
        self.body.world_position() + Vec3::new(0.0, EYE_HEIGHT, 0.0) + self.body.forward * CAMERA_FORWARD_OFFSET
    }

    /// Cartesian right direction relative to player.
    pub fn right_vector(&self) -> Vec3 {
        self.body.right
    }

    /// Cartesian unit vector in the up direction.
    pub fn up(&self) -> Vec3 {
        WORLD_UP_VECTOR
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
