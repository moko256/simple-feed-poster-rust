use std::str::FromStr;

use chrono::{DateTime, Utc};
use cron::Schedule;

pub struct Schedules {
    schedules: Vec<Schedule>,
}

impl Schedules {
    pub fn parse(crons: &[String]) -> Result<Schedules, <Schedule as FromStr>::Err> {
        let mut schedules = Vec::with_capacity(crons.len());

        for c in crons {
            schedules.push(Schedule::from_str(c)?);
        }

        Ok(Schedules { schedules })
    }

    pub fn upcoming_next(&self) -> Option<DateTime<Utc>> {
        Some(
            self.schedules
                .iter()
                .map(|s| s.upcoming(Utc).next().unwrap())
                .min()
                .unwrap(),
        )
    }
}
