#![allow(unused)]

use crate::{models::Upload, Result};
use std::path::Path;

pub enum UploadDataType {
    Fit,
    FitGz,
    Tcx,
    TcxGz,
    Gpx,
    GpxGz,
}

/// Uploads a new data file to create an activity from. Requires activity:write scope.
pub fn create_upload(
    file: &Path,
    name: String,
    description: String,
    trainer: bool,
    commute: bool,
    data_type: UploadDataType,
    external_id: String,
) -> Result<Upload> {
    todo!()
}

/// Returns an upload for a given identifier. Requires activity:write scope.
pub fn get_upload_by_id(upload_id: u64) -> Result<Upload> {
    todo!()
}
