pub type DateTime = ();

// A set of rolled-up statistics and totals for an athlete.
pub struct ActivityStats {
    pub biggest_ride_distance: f32,
    pub biggest_climb_elevation_gain: f32,
    pub recent_ride_totals: ActivityTotal,
    pub recent_run_totals: ActivityTotal,
    pub recent_swim_totals: ActivityTotal,
    pub ytd_ride_totals: ActivityTotal,
    pub ytd_run_totals: ActivityTotal,
    pub ytd_swim_totals: ActivityTotal,
    pub all_ride_totals: ActivityTotal,
    pub all_run_totals: ActivityTotal,
    pub all_swim_totals: ActivityTotal,
}

// A roll-up of metrics pertaining to a set of activities. Values are in seconds and meters.
pub struct ActivityTotal {
    pub count: u32,
    pub distance: f32,
    pub moving_time: u32,
    pub elapsed_time: u32,
    pub elevation_gain: f32,
    pub achievement_count: u32,
}

// An enumeration of the types an activity may have. Note that this enumeration does not include new sport types (e.g. MountainBikeRide, EMountainBikeRide), activities with these sport types will have the corresponding activity type (e.g. Ride for MountainBikeRide, EBikeRide for EMountainBikeRide).
pub enum ActivityType {
    AlpineSki,
    BackcountrySki,
    Canoeing,
    Crossfit,
    EBikeRide,
    Elliptical,
    Golf,
    Handcycle,
    Hike,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    NordicSki,
    Ride,
    RockClimbing,
    RollerSki,
    Rowing,
    Run,
    Sail,
    Skateboard,
    Snowboard,
    Snowshoe,
    Soccer,
    StairStepper,
    StandUpPaddling,
    Surfing,
    Swim,
    Velomobile,
    VirtualRide,
    VirtualRun,
    Walk,
    WeightTraining,
    Wheelchair,
    Windsurf,
    Workout,
    Yoga,
}

pub struct ActivityZone {
    pub score: u32,
    pub distribution_buckets: TimedZoneDistribution,
    // original name: type, possible values: heartrate, power
    pub activity_type: String,
    pub sensor_based: bool,
    pub points: u32,
    pub custom_zones: bool,
    pub max: u32,
}

pub struct BaseStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
}

pub struct Comment {
    pub id: u64,
    pub activity_id: u64,
    pub text: String,
    pub athlete: SummaryAthlete,
    pub created_at: DateTime,
}

pub struct Error {
    pub code: String,
    pub field: String,
    pub resource: String,
}

pub struct ExplorerResponse {
    pub segments: Vec<ExplorerSegment>,
}

pub struct ExplorerSegment {
    pub id: u64,
    pub name: String,
    // category of the climb [0-5]
    pub climb_category: u32,
    // possible values: NC, 4, 3, 2, 1, HC
    pub climb_category_desc: String,
    pub avg_grade: f32,
    pub start_latlng: LatLng,
    pub end_latlng: LatLng,
    pub elev_difference: f32,
    pub distance: f32,
    pub points: String,
}

// Encapsulates the errors that may be returned from the API.
pub struct Fault {
    pub errors: Vec<Error>,
    pub message: String,
}

pub struct HeartRateZoneRanges {
    pub custom_zones: bool,
    pub zones: ZoneRanges,
}

pub struct Lap {
    pub id: u64,
    pub activity: MetaActivity,
    pub athlete: MetaAthlete,
    pub average_cadence: f32,
    pub average_speed: f32,
    pub distance: f32,
    pub elapsed_time: u32,
    pub start_index: u32,
    pub end_index: u32,
    pub lap_index: u32,
    pub max_speed: f32,
    pub moving_time: u32,
    pub name: String,
    pub pace_zone: u32,
    pub split: u32,
    pub start_date: DateTime,
    pub start_date_local: DateTime,
    pub total_elevation_gain: f32,
}

// A collection of float objects. A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers.
pub struct LatLng {
    pub lat: f32,
    pub lon: f32,
}

pub struct MetaActivity {
    pub id: u64,
}

pub struct MetaAthlete {
    pub id: u64,
}

pub struct MetaClub {
    pub id: u64,
    // possible values: 1: meta, 2: summary, 3: detail
    pub resource_state: u32,
    pub name: String,
}

pub struct PhotosSummary {
    pub count: u32,
    pub primary: PhotosSummaryPrimary,
}

pub struct PhotosSummaryPrimary {
    pub id: u64,
    pub source: u32,
    pub unique_id: u32,
    pub urls: String,
}

pub struct PolylineMap {
    pub id: String,
    pub polyline: String,
    pub summary_polyline: String,
}

pub struct PowerZoneRanges {
    pub zones: ZoneRanges,
}

pub struct Route {
    pub athlete: SummaryAthlete,
    pub description: String,
    pub distance: f32,
    pub elevation_gain: f32,
    pub id: u64,
    pub id_str: String,
    pub map: PolylineMap,
    pub name: String,
    pub private: bool,
    pub starred: bool,
    pub timestamp: u32,
    // original name: `type`, possible values: 1: ride, 2: run
    pub route_type: u32,
    // possible values: 1: road, 2: mountain bike, 3: cross, 4: trail, 5: mixed
    pub sub_type: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub estimated_moving_time: u32,
    pub segments: SummarySegment,
}

pub struct Split {
    pub average_speed: f32,
    pub distance: f32,
    pub elapsed_time: u32,
    pub elevation_difference: f32,
    pub pace_zone: u32,
    pub moving_time: u32,
    pub split: u32,
}

// An enumeration of the sport types an activity may have. Distinct from ActivityType in that it has new types (e.g. MountainBikeRide).
pub enum SportType {
    AlpineSki,
    BackcountrySki,
    Canoeing,
    Crossfit,
    EBikeRide,
    Elliptical,
    EMountainBikeRide,
    Golf,
    GravelRide,
    Handcycle,
    Hike,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    MountainBikeRide,
    NordicSki,
    Ride,
    RockClimbing,
    RollerSki,
    Rowing,
    Run,
    Sail,
    Skateboard,
    Snowboard,
    Snowshoe,
    Soccer,
    StairStepper,
    StandUpPaddling,
    Surfing,
    Swim,
    TrailRun,
    Velomobile,
    VirtualRide,
    VirtualRun,
    Walk,
    WeightTraining,
    Wheelchair,
    Windsurf,
    Workout,
    Yoga,
}

pub struct StreamSet {
    pub time: TimeStream,
    pub distance: DistanceStream,
    pub latlng: LatLngStream,
    pub altitude: AltitudeStream,
    pub velocity_smooth: SmoothVelocityStream,
    pub heartrate: HeartrateStream,
    pub cadence: CadenceStream,
    pub watts: PowerStream,
    pub temp: TemperatureStream,
    pub moving: MovingStream,
    pub grade_smooth: SmoothGradeStream,
}

pub struct SummaryGear {
    pub id: String,
    // possible values: 2: summary, 3: detail
    pub resource_state: u32,
    pub primary: bool,
    pub name: String,
    pub distance: f32,
}

pub struct SummaryPRSegmentEffort {
    pub pr_activity_id: u64,
    pub pr_elapsed_time: u32,
    pub pr_date: DateTime,
    pub effort_count: u32,
}

pub struct SummarySegment {
    pub id: u64,
    pub name: String,
    // possible values: Ride, Run
    pub activity_type: String,
    pub distance: f32,
    pub average_grade: f32,
    pub maximum_grade: f32,
    pub elevation_high: f32,
    pub elevation_low: f32,
    pub start_latlng: LatLng,
    pub end_latlng: LatLng,
    // possible values: 0-5
    pub climb_category: u32,
    pub city: String,
    pub state: String,
    pub country: String,
    pub private: bool,
    pub athlete_pr_effort: SummarySegmentEffort,
    pub athlete_segment_stats: SummaryPRSegmentEffort,
}

pub struct SummarySegmentEffort {
    pub id: u64,
    pub activity_id: u64,
    pub elapsed_time: u32,
    pub start_date: DateTime,
    pub start_date_local: DateTime,
    pub distance: f32,
    pub is_kom: bool,
}

pub type TimedZoneDistribution = Vec<TimedZoneRange>;

pub struct UpdatableActivity {
    pub commute: bool,
    pub trainer: bool,
    pub hide_from_home: bool,
    pub description: String,
    pub name: String,
    // original name: `type`
    #[deprecated]
    pub activity_type: ActivityType,
    pub sport_type: SportType,
    pub gear_id: String,
}

pub struct Upload {
    pub id: u64,
    pub id_str: String,
    pub external_id: String,
    pub error: String,
    pub status: String,
    pub activity_id: u64,
}

pub struct ZoneRange {
    pub min: u32,
    pub max: u32,
}

pub type ZoneRanges = Vec<ZoneRange>;

pub struct Zones {
    pub heart_rate: HeartRateZoneRanges,
    pub power: PowerZoneRanges,
}

pub struct AltitudeStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<f32>,
}

pub struct CadenceStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<u32>,
}

pub struct DetailedGear {
    pub id: String,
    // possible values: 2: summary, 3: detail
    pub resource_state: u32,
    pub primary: bool,
    pub name: String,
    pub distance: f32,
    pub brand_name: String,
    pub model_name: String,
    // bike only
    pub frame_type: Option<u32>,
    pub description: String,
}

pub struct DetailedSegment {
    pub id: u64,
    pub name: String,
    // possible values: Ride, Run
    pub activity_type: String,
    pub distance: f32,
    pub average_grade: f32,
    pub maximum_grade: f32,
    pub elevation_high: f32,
    pub elevation_low: f32,
    pub start_latlng: LatLng,
    pub end_latlng: LatLng,
    pub climb_category: u32,
    pub city: String,
    pub state: String,
    pub country: String,
    pub private: bool,
    pub athlete_pr_effort: SummarySegmentEffort,
    pub athlete_segment_stats: SummaryPRSegmentEffort,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub total_elevation_gain: f32,
    pub map: PolylineMap,
    pub effort_count: u32,
    pub athlete_count: u32,
    pub hazardous: bool,
    pub star_count: u32,
}

pub struct DetailedSegmentEffort {
    pub id: u64,
    pub activity_id: u64,
    pub elapsed_time: u32,
    pub start_date: DateTime,
    pub start_date_loca: DateTime,
    pub distance: f32,
    pub is_kom: bool,
    pub name: String,
    pub activity: MetaActivity,
    pub athlete: MetaAthlete,
    pub moving_time: u32,
    pub start_index: u32,
    pub end_index: u32,
    pub average_cadence: f32,
    pub average_watts: f32,
    pub device_watts: bool,
    pub average_heartrate: f32,
    pub max_heartrate: f32,
    pub segment: SummarySegment,
    pub kom_rank: u32,
    pub pr_rank: u32,
    pub hidden: bool,
}

pub struct DistanceStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<f32>,
}

pub struct HeartrateStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<u32>,
}

pub struct LatLngStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<LatLng>,
}

pub struct MovingStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<bool>,
}

pub struct PowerStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<u32>,
}

pub struct SmoothGradeStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<f32>,
}

pub struct SmoothVelocityStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<f32>,
}

pub struct SummaryActivity {
    pub id: u64,
    pub external_id: String,
    pub upload_id: u64,
    pub athlete: MetaAthlete,
    pub name: String,
    pub distance: f32,
    pub moving_time: u32,
    pub elapsed_time: u32,
    pub total_elevation_gain: f32,
    pub elev_high: f32,
    pub elev_low: f32,
    // original name: `type`
    #[deprecated]
    pub activity_type: ActivityType,
    pub sport_type: SportType,
    pub start_date: DateTime,
    pub start_date_local: DateTime,
    pub timezone: String,
    pub start_latlng: LatLng,
    pub end_latlng: LatLng,
    pub achievement_count: u32,
    pub kodus_count: u32,
    pub comment_count: u32,
    pub athlete_count: u32,
    pub photo_count: u32,
    pub total_photo_count: u32,
    pub map: PolylineMap,
    pub trainer: bool,
    pub commute: bool,
    pub manual: bool,
    pub private: bool,
    pub flagged: bool,
    pub workout_type: u32,
    pub upload_id_str: String,
    pub average_speed: f32,
    pub max_speed: f32,
    pub has_kudoed: bool,
    pub hide_from_home: bool,
    pub gear_id: String,
    pub kilojoules: f32,
    pub average_watts: f32,
    pub device_watts: bool,
    pub max_watts: u32,
    pub weighted_average_watts: u32,
}

pub struct SummaryAthlete {
    pub id: u64,
    // possible values: 1: meta, 2: summary, 3: detail
    pub resource_state: u32,
    pub firstname: String,
    pub lastname: String,
    pub profile_medium: String,
    pub profile: String,
    pub city: String,
    pub state: String,
    pub country: String,
    // possible values: M, F
    pub sex: String,
    #[deprecated]
    pub premium: bool,
    pub summit: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

pub struct SummaryClub {
    pub id: u64,
    // possible values: 1: meta, 2: summary, 3: detail
    pub resource_state: u32,
    pub name: String,
    pub profile_medium: String,
    pub cover_photo: String,
    pub cover_photo_small: String,
    // possible values: cycling, running, triathlon, other
    #[deprecated]
    pub sport_type: String,
    pub activity_type: ActivityType,
    pub city: String,
    pub state: String,
    pub country: String,
    pub private: bool,
    pub member_count: u32,
    pub featured: bool,
    pub verified: bool,
    pub url: String,
}

pub struct TemperatureStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<u32>,
}

pub struct TimeStream {
    pub original_size: u32,
    // possible values: low, medium, high
    pub resolution: String,
    // possible values: distance, time
    pub series_type: String,
    pub data: Vec<u32>,
}

pub struct TimedZoneRange {
    pub min: u32,
    pub max: u32,
    pub time: u32,
}

pub struct DetailedActivity {
    pub id: u64,
    pub external_id: String,
    pub upload_id: u64,
    pub athlete: MetaAthlete,
    pub name: String,
    pub distance: f32,
    pub moving_time: u32,
    pub elapsed_time: u32,
    pub total_elevation_gain: f32,
    pub elev_high: f32,
    pub elev_low: f32,
    // original name: `type`
    #[deprecated]
    pub activity_type: ActivityType,
    pub sport_type: SportType,
    pub start_date: DateTime,
    pub start_date_local: DateTime,
    pub timezone: String,
    pub start_latlng: LatLng,
    pub end_latlng: LatLng,
    pub achievement_count: u32,
    pub kodus_count: u32,
    pub comment_count: u32,
    pub athlete_count: u32,
    pub photo_count: u32,
    pub total_photo_count: u32,
    pub map: PolylineMap,
    pub trainer: bool,
    pub commute: bool,
    pub manual: bool,
    pub private: bool,
    pub flagged: bool,
    pub workout_type: u32,
    pub upload_id_str: String,
    pub average_speed: f32,
    pub max_speed: f32,
    pub has_kudoed: bool,
    pub hide_from_home: bool,
    pub gear_id: String,
    pub kilojoules: f32,
    pub average_watts: f32,
    pub device_watts: bool,
    pub max_watts: u32,
    pub weighted_average_watts: u32,
    pub description: String,
    pub photos: PhotosSummary,
    pub gear: SummaryGear,
    pub calories: f32,
    pub segment_efforts: Vec<DetailedSegmentEffort>,
    pub device_name: String,
    pub embed_token: String,
    pub splits_metric: Split,
    pub splits_standard: Split,
    pub laps: Vec<Lap>,
    pub best_efforts: Vec<DetailedSegmentEffort>,
}

pub struct DetailedAthelete {
    pub id: u64,
    // possible values: 1: meta, 2: summary, 3: detail
    pub resource_state: u32,
    pub firstname: String,
    pub lastname: String,
    pub profile_medium: String,
    pub profile: String,
    pub city: String,
    pub state: String,
    pub country: String,
    // possible values: M, F
    pub sex: String,
    #[deprecated]
    pub premium: bool,
    pub summit: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub follower_count: u32,
    pub friend_count: u32,
    // possible values: feet, meters
    pub measurement_preference: String,
    pub ftp: u32,
    pub weight: f32,
    pub clubs: SummaryClub,
    pub bikes: SummaryGear,
    pub shoes: SummaryGear,
}

pub struct DetailedClub {
    pub id: u64,
    // possible values: 1: meta, 2: summary, 3: detail
    pub resource_state: u32,
    pub name: String,
    pub profile_medium: String,
    pub cover_photo: String,
    pub cover_photo_small: String,
    // possible values: cycling, running, triathlon, other
    #[deprecated]
    pub sport_type: String,
    pub activity_type: ActivityType,
    pub city: String,
    pub state: String,
    pub country: String,
    pub private: bool,
    pub member_count: u32,
    pub features: bool,
    pub verified: bool,
    pub url: String,
    // possible values: member, pending
    pub membership: String,
    pub admin: bool,
    pub owner: bool,
    pub following_count: u32,
}
