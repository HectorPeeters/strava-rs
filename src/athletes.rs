#![allow(unused)]

use crate::{
    models::{ActivityStats, DetailedAthelete, Zones},
    Result,
};

/// Returns the currently authenticated athlete. Tokens with profile:read_all scope will receive a detailed athlete representation; all others will receive a summary representation.
pub fn get_logged_in_athlete() -> Result<DetailedAthelete> {
    todo!()
}

/// Returns the the authenticated athlete's heart rate and power zones. Requires profile:read_all.
pub fn get_logged_in_athlete_zones() -> Result<Zones> {
    todo!()
}

/// Returns the activity stats of an athlete. Only includes data from activities set to Everyone visibilty.
pub fn get_stats(id: u64) -> Result<ActivityStats> {
    todo!()
}

/// Update the currently authenticated athlete. Requires profile:write scope.
pub fn update_logged_in_athlete(weight: f32) -> Result<DetailedAthelete> {
    todo!()
}
