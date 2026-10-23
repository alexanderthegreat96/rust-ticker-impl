// given a set of features
// run them, the moment they implement the Tickable trait

use crate::ticker::tickrate::Tickrate;

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

// default values
const FRAME_TIME: u64 = 250;
const BASE_SLEEP: u64 = 5;

// implement this for each of your features
// requiring ticking intervals
// ex: impl Tickable for PlayerPosBroadcast
pub trait Tickable: Send + Sync {
    // the function that needs to be ran
    // should contain the logic that's supposed
    // to run at the set interval
    fn tick(&mut self, dt: Duration);

    // is this enabled?
    // only execute tick func if enabled
    fn is_enabled(&self) -> bool;

    // different features demand different tick rates
    // in most cases, the one of the enum variants
    // of Tickrate is more than enough
    fn tick_rate(&self) -> Tickrate;

    // what's this called?
    // ex: player_pos_update
    fn name(&self) -> &str;
}

// tracking individual accumulators
// per feature
struct ScheduledFeature {
    feature: Box<dyn Tickable>,
    accumulator: Duration,
}

// actual impl
// which executes the code
pub struct GlobalTicker {
    feature_list: Vec<ScheduledFeature>,
    max_frame_time: Duration,
    base_sleep: Duration,
}

#[allow(unused)]
impl GlobalTicker {
    // auto-load features
    // you define them with
    // Box::new(YourFeatureName that implements Tickable)
    pub fn new(features: Vec<Box<dyn Tickable>>) -> Self {
        let scheduled_features = features
            .into_iter()
            .map(|f| ScheduledFeature {
                feature: f,
                accumulator: Duration::ZERO,
            })
            .collect();

        Self {
            feature_list: scheduled_features,
            max_frame_time: Duration::from_millis(FRAME_TIME),
            base_sleep: Duration::from_millis(BASE_SLEEP),
        }
    }

    // allows you to customize the frame time
    pub fn set_max_frame_time(&mut self, milliseconds: u64) {
        self.max_frame_time = Duration::from_millis(milliseconds);
    }

    // default is 5 ms, but you can customize it
    pub fn set_base_sleep(&mut self, milliseconds: u64) {
        self.base_sleep = Duration::from_millis(milliseconds);
    }

    // register a feature a bit layer
    pub fn register_feature(&mut self, feature: Box<dyn Tickable>) {
        self.feature_list.push(ScheduledFeature {
            feature,
            accumulator: Duration::ZERO,
        });
    }

    pub fn run(&mut self) {
        let mut last_time: Instant = Instant::now();
        loop {
            let current_time: Instant = Instant::now();
            let mut frame_time = current_time - last_time;

            last_time = current_time;

            // frame time clamp to prevent lagging the system
            if frame_time > self.max_frame_time {
                frame_time = self.max_frame_time;
            }

            for sf in &mut self.feature_list {
                // nothing to do if the feature
                // is NOT enabled
                if !sf.feature.is_enabled() {
                    continue;
                }

                sf.accumulator += frame_time;
                let interval = sf.feature.tick_rate().duration();

                while sf.accumulator >= interval {
                    let start: Instant = Instant::now();
                    sf.feature.tick(interval); // feature dispatch
                    let elapsed = start.elapsed();

                    // if a feature took longer
                    // to execute we can signal it here
                    // possibly send it to a queue or something
                    // for further investigation
                    if elapsed > interval {
                        println!(
                            "Warning: feature: {} took: {:?} -> exceeds it's {:?} tick rate",
                            sf.feature.name(),
                            elapsed,
                            interval
                        )
                    }

                    sf.accumulator -= interval;
                }
            }

            // pause for cpu spike
            let processing_duration = last_time.elapsed();
            if processing_duration < self.base_sleep {
                sleep(self.base_sleep - processing_duration);
            }
        }
    }
}
