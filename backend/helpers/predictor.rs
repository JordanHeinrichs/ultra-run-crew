use serde::{Deserialize, Serialize};

use crate::{
    db::models::{AidStation, Course, Race},
    errors::AppError,
    helpers::gpx_parser::GpxSegment,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CoursePredictions {
    pub aid_station_id: i64,
    pub duration_s: i64,
}

pub fn calculate_predictions(
    race: &Race,
    course: &Course,
    aid_stations: &Vec<AidStation>,
) -> Result<Vec<CoursePredictions>, AppError> {
    let distance_km = course.total_distance_km;

    // Calculate elevation gain/distance per aid station sections (timing sections), assign difficulties
    // Assign difficulties by multiplying section distances by the gap_factor.

    // Apply time strategy (positive/negative) to get planned sections

    // If race is started, apply rolling average against weightings (favouring later info)

    return Ok(vec![]);
}
