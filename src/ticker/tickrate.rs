use std::time::Duration;
#[allow(unused)]

// Generic helper for defining tick rates
pub enum Tickrate {
    Hz120,
    Hz60,
    Hz30,
    Hz15,
    Hz8,
    Hz1,
    Custom(Duration),
}

impl Tickrate {
    pub fn duration(&self) -> Duration {
        match self {
            Tickrate::Hz120 => Duration::from_secs_f64(1.0 / 120.0), // 8.83 ms
            Tickrate::Hz60 => Duration::from_secs_f64(1.0 / 60.0),   // 16.6 ms
            Tickrate::Hz30 => Duration::from_secs_f64(1.0 / 30.0),   // 33.3 ms
            Tickrate::Hz15 => Duration::from_secs_f64(1.0 / 15.0),   // 66.6 ms
            Tickrate::Hz8 => Duration::from_secs_f64(1.0 / 8.0),     // 125 ms
            Tickrate::Hz1 => Duration::from_secs(1),                 // 1000 ms
            Tickrate::Custom(d) => *d,
        }
    }
}
