use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(unused)]
impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[allow(unused)]
pub struct LocalPlayer {
    position: Vec3,
    velocity: Vec3,
    health: f32,
    is_alive: bool,
}

#[allow(unused)]
impl LocalPlayer {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x, y, z),
            velocity: Vec3::ZERO,
            health: 100.0,
            is_alive: true,
        }
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn get_coordinates(&self) -> (f32, f32, f32) {
        (self.position.x, self.position.y, self.position.z)
    }

    pub fn get_velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive && self.health > 0.0
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    pub fn set_velocity(&mut self, vel: Vec3) {
        self.velocity = vel;
    }
}

#[allow(unused)]
pub struct GameWorld {
    local_player: Arc<LocalPlayer>,
}

#[allow(unused)]
impl GameWorld {
    pub fn new(local_player: Arc<LocalPlayer>) -> Self {
        Self { local_player }
    }

    pub fn get_local_player(&self) -> Arc<LocalPlayer> {
        Arc::clone(&self.local_player)
    }

    pub fn get_local_player_position(&self) -> Vec3 {
        self.local_player.get_position()
    }
}
