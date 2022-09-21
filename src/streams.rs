#![allow(unused)]

use crate::{models::StreamSet, Result};

/// Returns the given activity's streams. Requires activity:read scope. Requires activity:read_all scope for Only Me activities.
pub fn get_activity_streams(id: u64, keys: &[String], key_by_type: bool) -> Result<StreamSet> {
    todo!()
}

/// Returns the given route's streams. Requires read_all scope for private routes.
pub fn get_route_streams(id: u64) -> Result<StreamSet> {
    todo!()
}

/// Returns a set of streams for a segment effort completed by the authenticated athlete. Requires read_all scope.
pub fn get_segment_effort_streams(
    id: u64,
    keys: &[String],
    key_by_type: bool,
) -> Result<StreamSet> {
    todo!()
}

/// Returns the given segment's streams. Requires read_all scope for private segments.
pub fn get_segment_streams(id: u64, keys: &[String], key_by_type: bool) -> Result<StreamSet> {
    todo!()
}
