use std::time::{Duration, SystemTime};

use serde::Serialize;
use sysinfo::{LoadAvg, System, Users};

#[derive(Debug, Serialize)]
pub struct Uptime {
    time: SystemTime,
    #[allow(clippy::struct_field_names)]
    uptime: Duration,
    users: Users,
    load_avg: LoadAvg,
}

impl Default for Uptime {
    fn default() -> Self {
        Self {
            time: SystemTime::now(),
            uptime: Duration::from_millis(System::uptime()),
            users: Users::new(),
            load_avg: System::load_average(),
        }
    }
}