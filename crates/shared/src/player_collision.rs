use super::chunk::ChunkLookup;
use glam::Vec3;

const GRAVITY: f32 = 20.0;
const JUMP_VELOCITY: Vec3 = Vec3::new(0.0, 8.0, 0.0);

/// Keeps pitch just short of ±90° so forward/right never go parallel to WORLD_UP.
const PITCH_LIMIT: f32 = 1.5533; // ~89 degrees

#[derive(PartialEq)]
pub struct Body {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub height: f32,
    pub half_width: f32,
    pub forward: Vec3,
    pub right: Vec3,
    pub velocity: Vec3,
    pub on_ground: bool,
    pub fly_mode: bool,
}

impl Body {
    /// Cartesian body-center position.
    pub fn world_position(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn fly_move(&mut self, delta: Vec3) {
        self.x += delta.x;
        self.y += delta.y;
        self.z += delta.z;
    }

    pub fn apply_physics(&mut self, dt: f32, world: &impl ChunkLookup) {
        if !self.fly_mode {
            self.velocity.y -= GRAVITY * dt;
            if !self.can_move(0.0, self.velocity.y * dt, 0.0, world) {
                if self.velocity.y <= 0.0 {
                    self.on_ground = true;
                }
                self.velocity.y = 0.0;
            }
        }
    }

    pub fn can_move(&mut self, dx: f32, dy: f32, dz: f32, world: &impl ChunkLookup) -> bool {
        let old_x = self.x;
        let old_y = self.y;
        let old_z = self.z;

        self.x += dx;
        self.y += dy;
        self.z += dz;

        if self.capsule_collides(world) {
            self.x = old_x;
            self.y = old_y;
            self.z = old_z;
            return false;
        }
        true
    }

    /// Walk-mode horizontal move with wall sliding: if the combined (x, z)
    /// move is blocked, retry each axis independently.
    pub fn walk(&mut self, direction: Vec3, world: &impl ChunkLookup) {
        if self.can_move(direction.x, 0.0, direction.z, world) {
            return;
        }
        self.can_move(direction.x, 0.0, 0.0, world);
        self.can_move(0.0, 0.0, direction.z, world);
    }

    pub fn capsule_collides(&self, world: &impl ChunkLookup) -> bool {
        let offsets = [
            (-self.half_width, -self.half_width),
            (-self.half_width, self.half_width),
            (self.half_width, -self.half_width),
            (self.half_width, self.half_width),
        ];

        for y_offset in [0.0, 0.85, self.height] {
            for &(dx, dz) in &offsets {
                if world.block_solid_at(
                    (self.x + dx).floor() as i32,
                    (self.y + y_offset).floor() as i32,
                    (self.z + dz).floor() as i32,
                ) {
                    return true;
                }
            }
        }
        false
    }

    pub fn jump(&mut self) {
        if self.on_ground && !self.fly_mode {
            self.velocity += JUMP_VELOCITY;
            self.on_ground = false;
        }
    }

    pub fn toggle_fly_mode(&mut self) {
        self.fly_mode = !self.fly_mode;
        self.velocity = Vec3::ZERO;
    }
}

pub struct Orientation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Orientation {
    /// Turn left/right around world-up.
    pub fn rotate_yaw(&mut self, body: &mut Body, delta: f32) {
        self.yaw -= delta;
        self.update_look_vectors(body);
    }

    /// Look up/down, clamped to ±~89° so the camera can't flip over.
    pub fn rotate_pitch(&mut self, body: &mut Body, delta: f32) {
        self.pitch = (self.pitch + delta).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        self.update_look_vectors(body);
    }

    /// Compute the player's forward and right direction vectors from the current yaw
    /// and pitch angles.
    pub fn update_look_vectors(&self, body: &mut Body) {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        body.forward = Vec3::new(cos_yaw * cos_pitch, sin_pitch, sin_yaw * cos_pitch).normalize();
        body.right = body.forward.cross(super::WORLD_UP_VECTOR).normalize();
    }
}
