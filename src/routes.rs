#![allow(unused)]

use crate::{models::Route, Pagination, Result};

/// Returns a GPX file of the route. Requires read_all scope for private routes.
pub fn get_route_as_gpx(id: u64) -> Result<()> {
    todo!()
}

/// Returns a TCX file of the route. Requires read_all scope for private routes.
pub fn get_route_as_tcx(id: u64) -> Result<()> {
    todo!()
}

/// Returns a route using its identifier. Requires read_all scope for private routes.
pub fn get_route_by_id(id: u64) -> Result<Route> {
    todo!()
}

/// Returns a list of the routes created by the authenticated athlete. Private routes are filtered out unless requested by a token with read_all scope.
pub fn get_routes_by_athlete_id(pagination: Option<Pagination>) -> Result<Vec<Route>> {
    todo!()
}
