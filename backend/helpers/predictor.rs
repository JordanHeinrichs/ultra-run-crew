use serde::{Deserialize, Serialize};

use crate::{
    db::models::{AidStation, Course, CourseSegment, Race},
    errors::AppError::{self, BadRequest},
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
    course_segments: &Vec<CourseSegment>,
    aid_stations: &Vec<AidStation>,
) -> Result<Vec<CoursePredictions>, AppError> {
    let distance_km = course.total_distance_km;
    let difficulty_distance_km: f64 = course_segments.iter().map(|x| x.km * x.gap_factor).sum();
    let expected_adj_pace_s_per_km = match race.goal_duration {
        Some(goal) => goal / difficulty_distance_km,
        _ => return Err(BadRequest("Need to have set goal time".into())),
    };

    let aid_station_iter = aid_stations.iter().peekable();
    let mut aid_station_difficulty_distances = Vec::with_capacity(aid_stations.len() + 1);
    let mut current_difficulty_distance = 0.0;
    let mut prev_km = 0.0;

    for seg in course_segments {
        let mut current_km = prev_km;
        let current_end_km = seg.km;

        while let Some(&aid_station) = aid_station_iter.peek() {
            if aid_station.km <= current_km {}
        }
    }

    // Calculate elevation gain/distance per aid station sections (timing sections), assign difficulties
    let aid_station_difficulty_distances: Vec<(i64, f64)> = aid_stations
        .iter()
        .map(|aid_station: &AidStation| {})
        .collect();

    // let mut current_km
    // for station in aid_stations.iter() {
    // }

    // Assign difficulties by multiplying section distances by the gap_factor.

    // Apply time strategy (positive/negative) to get planned sections

    // If race is started, apply rolling average against weightings (favouring later info)

    return Ok(vec![]);
}
