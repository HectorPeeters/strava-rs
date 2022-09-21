#![allow(unused)]

use crate::{models::DetailedSegmentEffort, Pagination, Result};

/// Returns a set of the authenticated athlete's segment efforts for a given segment. Requires subscription.
pub fn get_efforts_by_segment_id(
    segment_id: u32,
    start_date_local: Option<String>,
    end_date_local: Option<String>,
    pagination: Option<Pagination>,
) -> Result<Vec<DetailedSegmentEffort>> {
    todo!()
}

/// Returns a segment effort from an activity that is owned by the authenticated athlete. Requires subscription.
pub fn get_segment_effort_by_id(id: u64) -> Result<DetailedSegmentEffort> {
    todo!()
}
