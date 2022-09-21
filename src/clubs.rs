#![allow(unused)]

use crate::{
    models::{DetailedClub, SummaryActivity, SummaryAthlete, SummaryClub},
    Pagination, Result,
};

/// Retrieve recent activities from members of a specific club. The authenticated athlete must belong to the requested club in order to hit this endpoint. Pagination is supported. Athlete profile visibility is respected for all activities.
pub fn get_club_activities_by_id(
    id: u64,
    pagination: Option<Pagination>,
) -> Result<Vec<SummaryActivity>> {
    todo!()
}

/// Returns a list of the administrators of a given club.
pub fn get_club_admins_by_id(
    id: u64,
    pagination: Option<Pagination>,
) -> Result<Vec<SummaryAthlete>> {
    todo!()
}

/// Returns a given club using its identifier.
pub fn get_club_by_id(id: u64) -> Result<DetailedClub> {
    todo!()
}

/// Returns a list of the athletes who are members of a given club.
pub fn get_club_members_by_id(
    id: u64,
    pagination: Option<Pagination>,
) -> Result<Vec<SummaryAthlete>> {
    todo!()
}

/// Returns a list of the clubs whose membership includes the authenticated athlete.
pub fn get_logged_in_athlete_clubs(pagination: Option<Pagination>) -> Result<Vec<SummaryClub>> {
    todo!()
}
