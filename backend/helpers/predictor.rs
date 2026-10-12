use serde::{Deserialize, Serialize};

use crate::{
    db::models::{AidStation, Course},
    errors::AppError,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CoursePredictions {
    pub aid_station_id: i64,
    pub duration_s: i64,
}

pub fn calculate_predictions(
    course: &Course,
    aid_stations: &Vec<AidStation>,
) -> Result<Vec<CoursePredictions>, AppError> {
    return Ok(vec![]);
}
