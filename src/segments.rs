#![allow(unused)]

use crate::{
    models::{DetailedSegment, ExplorerResponse, LatLng, SummarySegment},
    Pagination, Result,
};

/// Returns the top 10 segments matching a specified query.
pub fn explore_segments(
    bounds: &(LatLng, LatLng),
    activity: Option<String>,
    min_cat: Option<u32>,
    max_cat: Option<u32>,
) -> Vec<ExplorerResponse> {
    todo!()
}

/// List of the authenticated athlete's starred segments. Private segments are filtered out unless requested by a token with read_all scope.
pub fn get_logged_in_athlete_starred_segments(
    pagination: Option<Pagination>,
) -> Result<Vec<SummarySegment>> {
    todo!()
}

/// Returns the specified segment. read_all scope required in order to retrieve athlete-specific segment information, or to retrieve private segments.
pub fn get_segment_by_id(id: u64) -> Result<DetailedSegment> {
    todo!()
}

/// Stars/Unstars the given segment for the authenticated athlete. Requires profile:write scope.
pub fn star_segment(id: u64, starred: bool) -> Result<DetailedSegment> {
    todo!()
}
