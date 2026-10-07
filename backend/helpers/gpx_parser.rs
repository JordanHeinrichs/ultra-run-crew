use geo_types::Point;
use gpx::read;
use gpx::{Gpx, Track, TrackSegment, Waypoint};

use crate::errors::AppError::{self, BadRequest};

pub const SEGMENT_LENGTH: f64 = 500.0;

pub struct GpxSegment {
    pub km: f64,
    pub gain_m: f64,
    pub loss_m: f64,
}

pub fn generate_segments_from_gpx(file: Vec<u8>) -> Result<Vec<GpxSegment>, AppError> {
    let gpx: Gpx = read(&file[..]).map_err(|_| BadRequest("Invalid GPX file".into()))?;
    if gpx.tracks.is_empty() {
        return Err(BadRequest("Invalid GPX file".into()));
    }
    let track: &Track = &gpx.tracks[0];

    let points: Vec<&Waypoint> = track
        .segments
        .iter()
        .flat_map(|seg| seg.points.iter())
        .collect();

    if points.len() < 2 {
        return Ok(Vec::new());
    }

    let mut segments = Vec::new();

    let mut current_dist_m = 0.0;
    let mut current_gain_m = 0.0;
    let mut current_loss_m = 0.0;

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        let step_distance = haversine_m(&p1.point(), &p2.point());
        if step_distance == 0.0 {
            continue;
        }

        let ele_diff = match (p1.elevation, p2.elevation) {
            (Some(e1), Some(e2)) => e2 - e1,
            _ => 0.0,
        };

        if current_dist_m + step_distance <= SEGMENT_LENGTH {
            current_dist_m = current_dist_m + step_distance;
            if ele_diff > 0.0 {
                current_gain_m = current_gain_m + ele_diff;
            } else {
                current_loss_m = current_loss_m + ele_diff.abs();
            }
        }

        // let pos1
    }

    Ok(segments)
}

fn haversine_m(p1: &Point<f64>, p2: &Point<f64>) -> f64 {
    let earth_radius_m = 6_371_000.0;
    println!("{:?}, {:?}", p1, p2);

    let p1_rad = p1.to_radians();
    let p2_rad = p2.to_radians();
    let d = p2_rad - p1_rad;
    println!("{:?}, {:?}, {:?}", p1_rad, p2_rad, d);

    let a = ((d.y() / 2.0).sin().powi(2)
        + p1_rad.y().cos() * p2_rad.y().cos() * (d.x() / 2.0).sin().powi(2))
    .clamp(0.0, 1.0);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    earth_radius_m * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_same_point_returns_zero() {
        let p = Point::new(-73.5673, 45.5017); // Montreal
        let dist = haversine_m(&p, &p);
        assert_abs_diff_eq!(dist, 0.0, epsilon = 0.001);
    }

    #[test]
    fn test_known_long_distance_london_to_paris() {
        let london = Point::new(-0.1278, 51.5074);
        let paris = Point::new(2.3522, 48.8566);

        let dist = haversine_m(&london, &paris);

        // True geodesic distance ~343,556 meters
        assert_abs_diff_eq!(dist, 343_556.0, epsilon = 100.0);
    }

    #[test]
    fn test_one_degree_latitude_at_equator() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(1.0, 0.0);

        let dist = haversine_m(&p1, &p2);

        // 1 degree of latitude is approximately 111.19 km (111,195 m)
        assert_abs_diff_eq!(dist, 111_195.0, epsilon = 50.0);
    }

    #[test]
    fn test_short_500m_trail_segment() {
        // Move ~500 meters north at 45° latitude (approx 0.0045 degrees lat)
        let p1 = Point::new(-110.0000, 45.0000);
        let p2 = Point::new(-110.0000, 45.0045);

        let dist = haversine_m(&p1, &p2);

        // Expected ~500.37 meters
        assert_abs_diff_eq!(dist, 500.37, epsilon = 1.0);
    }

    #[test]
    fn test_prime_meridian_crossing() {
        let west = Point::new(-0.0010, 51.4778); // West of Greenwich
        let east = Point::new(0.0010, 51.4778); // East of Greenwich

        let dist = haversine_m(&west, &east);

        // Should correctly handle negative-to-positive longitude transition (~139 m)
        assert!(dist > 130.0 && dist < 150.0);
    }

    #[test]
    fn test_international_date_line_crossing() {
        let west_of_antimeridian = Point::new(0.0, 179.99);
        let east_of_antimeridian = Point::new(0.0, -179.99);

        let dist = haversine_m(&west_of_antimeridian, &east_of_antimeridian);

        // Moving 0.02 degrees across the 180° meridian at the equator is ~2,224 meters
        assert_abs_diff_eq!(dist, 2_224.0, epsilon = 10.0);
    }
}
