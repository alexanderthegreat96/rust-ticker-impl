use std::sync::Arc;
use std::time::Duration;

use crate::sdk::{LocalPlayer, Vec3};
use crate::ticker::{Tickable, Tickrate};

pub struct BroadcastPlayerPos {
    enabled: bool,
    cooldown_timer: Duration,
    lp: Arc<LocalPlayer>,
}

impl BroadcastPlayerPos {
    pub fn new(enabled: bool, lp: Arc<LocalPlayer>) -> Self {
        Self {
            enabled,
            cooldown_timer: Duration::ZERO,
            lp: lp,
        }
    }

    pub fn broadcast(&self) {
        let current_pos: Vec3 = self.lp.get_position();

        println!(
            "Broadcasting Player Pos at: x:{:?} | y: {:?} | z: {:?}",
            current_pos.x, current_pos.y, current_pos.z
        )
    }
}

// trait implementation for the Tickable
impl Tickable for BroadcastPlayerPos {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn name(&self) -> &str {
        return "BroadcastPlayerPos";
    }

    fn tick(&mut self, dt: std::time::Duration) {
        // intentionally not using dt
        // could use that to further pause the execution
        // or for potential computations
        self.cooldown_timer = self.cooldown_timer.saturating_sub(dt);

        if self.cooldown_timer > Duration::ZERO {
            return;
        }

        self.broadcast();
        self.cooldown_timer = Duration::from_millis(250);
    }

    fn tick_rate(&self) -> Tickrate {
        Tickrate::Hz8
    }
}
