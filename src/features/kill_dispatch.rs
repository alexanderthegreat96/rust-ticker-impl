use std::{sync::Arc, time::Duration};

use rand::RngExt;

use crate::{
    sdk::LocalPlayer,
    ticker::{Tickable, Tickrate},
};

#[allow(unused)]
pub struct KillDispatch {
    enabled: bool,
    cooldown_timer: Duration,
    lp: Arc<LocalPlayer>,
}

impl KillDispatch {
    pub fn new(enabled: bool, lp: Arc<LocalPlayer>) -> Self {
        Self {
            enabled,
            cooldown_timer: Duration::ZERO,
            lp,
        }
    }

    pub fn dispatch_kill(&self) {
        let some_names: [&str; 5] = [
            "dudestan123",
            "shamastan332",
            "hungayrian_bear",
            "transformer_love990",
            "gokusan_223",
        ];

        let mut rng = rand::rng();
        let rand_key = rng.random_range(0..=4);
        let chosen_username = some_names[rand_key];

        println!("Local Player killed: {}", chosen_username);
    }
}

impl Tickable for KillDispatch {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn name(&self) -> &str {
        return "KillDispatch";
    }

    fn tick_rate(&self) -> crate::ticker::Tickrate {
        Tickrate::Hz8
    }

    fn tick(&mut self, dt: Duration) {
        self.cooldown_timer = self.cooldown_timer.saturating_sub(dt);
        if self.cooldown_timer > Duration::ZERO {
            return;
        }

        self.dispatch_kill();
        self.cooldown_timer = Duration::from_millis(10);
    }
}
