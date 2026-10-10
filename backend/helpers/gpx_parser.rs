use geo_types::Point;
use gpx::read;
use gpx::{Gpx, Track, Waypoint};

use crate::errors::AppError::{self, BadRequest};

pub const SEGMENT_LENGTH: f64 = 500.0;

#[derive(Debug)]
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
    Ok(calculate_segments(&points))
}

fn calculate_segments(points: &Vec<&Waypoint>) -> Vec<GpxSegment> {
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

        let mut remaining_step_m = step_distance;
        while current_dist_m + remaining_step_m >= SEGMENT_LENGTH {
            let needed_for_segment_m = SEGMENT_LENGTH - current_dist_m;
            let prorate = needed_for_segment_m / step_distance;

            if ele_diff > 0.0 {
                current_gain_m += ele_diff * prorate;
            } else {
                current_loss_m += (ele_diff * prorate).abs();
            }

            segments.push(GpxSegment {
                km: (segments.len() + 1) as f64 * SEGMENT_LENGTH / 1000.0,
                gain_m: current_gain_m,
                loss_m: current_loss_m,
            });
            remaining_step_m -= needed_for_segment_m;
            current_dist_m = 0.0;
            current_gain_m = 0.0;
            current_loss_m = 0.0;
        }

        if remaining_step_m > 0.0 {
            let prorate = remaining_step_m / step_distance;
            current_dist_m += remaining_step_m;
            if ele_diff > 0.0 {
                current_gain_m += ele_diff * prorate;
            } else {
                current_loss_m += (ele_diff * prorate).abs();
            }
        }
    }
    segments.push(GpxSegment {
        km: (segments.len() as f64 * SEGMENT_LENGTH + current_dist_m) / 1000.0,
        gain_m: current_gain_m,
        loss_m: current_loss_m,
    });
    segments
}

fn haversine_m(p1: &Point<f64>, p2: &Point<f64>) -> f64 {
    let earth_radius_m = 6_371_000.0;

    let p1_rad = p1.to_radians();
    let p2_rad = p2.to_radians();
    let d = p2_rad - p1_rad;

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
    use geo_types::Point;
    use gpx::Waypoint;

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

    fn make_waypoint(lon: f64, lat: f64, elevation_m: f64) -> Waypoint {
        let mut wp = Waypoint::new(Point::new(lon, lat));
        wp.elevation = Some(elevation_m);
        wp
    }

    #[test]
    fn test_calculate_segments_single_step() {
        let wp1 = make_waypoint(-0.1278, 51.5074, 100.0);
        let wp2 = make_waypoint(-0.1200, 51.5074, 150.0);
        let wp3 = make_waypoint(-0.1150, 51.5074, 10.0);

        let waypoints = vec![&wp1, &wp2, &wp3];
        let segments = calculate_segments(&waypoints);

        assert_eq!(segments.len(), 2);

        assert_abs_diff_eq!(segments[0].km, 0.5, epsilon = 0.001);
        assert_abs_diff_eq!(segments[1].km, 0.8859, epsilon = 0.001);

        assert_abs_diff_eq!(
            segments.iter().map(|s| s.gain_m).sum::<f64>(),
            50.0,
            epsilon = 1.0
        );
        assert_abs_diff_eq!(
            segments.iter().map(|s| s.loss_m).sum::<f64>(),
            140.0,
            epsilon = 1.0
        );
    }
}
