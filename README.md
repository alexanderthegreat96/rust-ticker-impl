# Rust Ticker Implementation

This small project showcases how to implement dynamically dispatchable ticker implementations for various applications. Can be used with games, high frequency loops and so on. It's a direct implementation of the `accumulator pattern` in Rust.

## Features

- global frame time
- global sleep
- typed tick rates via `TickRate` enum
- easy to use API

## Usage

To use this implementation, all you gotta do is clone this repo, then copy the `ticker` folder in your project.

### Implementation Example

```rust
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

```

### Main function example

```rust
mod features;
mod sdk;
mod ticker;

use std::sync::Arc;

use crate::features::{BroadcastPlayerPos, KillDispatch};
use crate::sdk::{GameWorld, LocalPlayer};
use crate::ticker::{GlobalTicker, Tickable};

fn main() {
    let lp: Arc<LocalPlayer> = Arc::new(LocalPlayer::new(123.0, 33.2, 983.1));
    let game: GameWorld = GameWorld::new(Arc::clone(&lp));

    // feature container
    let mut tick_features: Vec<Box<dyn Tickable>> = Vec::new();

    // all features must be added here
    tick_features.push(Box::new(BroadcastPlayerPos::new(
        true,
        Arc::clone(&game.get_local_player()),
    )));

    tick_features.push(Box::new(KillDispatch::new(
        true,
        Arc::clone(&game.get_local_player()),
    )));

    // exec the ticker
    let mut ticker = GlobalTicker::new(tick_features);
    ticker.set_base_sleep(5);
    ticker.set_max_frame_time(250);
    ticker.run();
}

```

### Why?

Cause why not? I think rust has an expressive syntax that is loved by many and exploring design patterns is just awesome. Not just in rust, but in just about any language. Rust's API is just crazy though. But anyways. Do whatever you want with it.
