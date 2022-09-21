#![allow(unused)]

use crate::{
    models::{
        ActivityZone, Comment, DetailedActivity, Lap, SummaryActivity, SummaryAthlete,
        UpdatableActivity,
    },
    Pagination, Result,
};

/// Creates a manual activity for an athlete, requires activity:write scope.
pub fn create_activity(
    name: String,
    activity_type: Option<String>,
    sport_type: String,
    start_date_local: String,
    elapsed_time: u32,
    description: Option<String>,
    distance: Option<f32>,
    trainer: Option<bool>,
    commute: Option<bool>,
) -> Result<DetailedActivity> {
    todo!()
}

/// Returns the given activity that is owned by the authenticated athlete. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.
pub fn get_activity(id: u64, include_all_efforts: Option<bool>) -> Result<DetailedActivity> {
    todo!()
}

/// Returns the comments on the given activity. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.
pub fn get_comments_by_activity_id(
    id: u64,
    pagination: Option<Pagination>,
) -> Result<Vec<Comment>> {
    todo!()
}

/// Returns the athletes who kudoed an activity identified by an identifier. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.
pub fn get_kudoers_by_activity_id(
    id: u64,
    pagination: Option<Pagination>,
) -> Result<Vec<SummaryAthlete>> {
    todo!()
}

/// Returns the laps of an activity identified by an identifier. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.
pub fn get_laps_by_activity_id(id: u64) -> Result<Vec<Lap>> {
    todo!()
}

/// Returns the activities of an athlete for a specific identifier. Requires activity:read. Only Me activities will be filtered out unless requested by a token with activity:read_all.
pub fn get_logged_in_athlete_activities(
    before: Option<u32>,
    after: Option<u32>,
    pagination: Option<Pagination>,
) -> Result<Vec<SummaryActivity>> {
    todo!()
}

/// Summit Feature. Returns the zones of a given activity. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.
pub fn get_zones_by_activity_id(id: u64) -> Result<Vec<ActivityZone>> {
    todo!()
}

/// Updates the given activity that is owned by the authenticated athlete. Requires activity:write. Also requires activity:read_all in order to update Only Me activities
pub fn update_activity_by_id(
    id: u64,
    updated_activity: UpdatableActivity,
) -> Result<DetailedActivity> {
    todo!()
}
