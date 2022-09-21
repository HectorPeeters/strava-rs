use models::Fault;

pub mod activities;
pub mod athletes;
pub mod clubs;
pub mod gears;
pub mod models;
pub mod routes;
pub mod segmentefforts;
pub mod segments;
pub mod streams;
pub mod uploads;

pub type Result<T> = std::result::Result<T, Fault>;
