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
