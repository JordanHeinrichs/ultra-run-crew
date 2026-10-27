use geo_types::Point;
use gpx::read;
use gpx::{Gpx, Track, Waypoint};

use crate::errors::AppError::{self, BadRequest};

pub const SEGMENT_LENGTH: f64 = 100.0;

#[derive(Debug)]
pub struct GpxSegment {
    pub km: f64,
    pub dist_m: f64,
    pub elev_diff_m: f64,
    pub gap_factor: f64,
}

impl GpxSegment {
    fn new(km: f64, dist_m: f64, elev_diff_m: f64) -> GpxSegment {
        return GpxSegment {
            km,
            dist_m,
            elev_diff_m,
            gap_factor: gap_factor(dist_m, elev_diff_m),
        };
    }
}

#[derive(Debug)]
pub struct CourseStats {
    pub total_distance_km: f64,
    pub elev_gain_m: f64,
    pub elev_loss_m: f64,
}

impl CourseStats {
    fn empty() -> CourseStats {
        CourseStats {
            total_distance_km: 0.0,
            elev_gain_m: 0.0,
            elev_loss_m: 0.0,
        }
    }
}

pub fn generate_segments_from_gpx(
    file: Vec<u8>,
) -> Result<(Vec<GpxSegment>, CourseStats), AppError> {
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
        return Ok((Vec::new(), CourseStats::empty()));
    }
    let segments = calculate_segments(&points);
    let course_stats = calculate_course_stats(&points, &segments);
    Ok((segments, course_stats))
}

fn calculate_course_stats(points: &Vec<&Waypoint>, segments: &Vec<GpxSegment>) -> CourseStats {
    if segments.len() == 0 {
        return CourseStats::empty();
    }

    let total_distance = match segments.last() {
        Some(last) => last.km + last.dist_m / 1000.0,
        _ => 0.0,
    };

    let mut elev_gain_m = 0.0;
    let mut elev_loss_m = 0.0;

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        let ele_diff = match (p1.elevation, p2.elevation) {
            (Some(e1), Some(e2)) => e2 - e1,
            _ => 0.0,
        };
        match ele_diff > 0.0 {
            true => elev_gain_m += ele_diff,
            false => elev_loss_m += ele_diff.abs(),
        };
    }

    CourseStats {
        total_distance_km: total_distance,
        elev_gain_m: elev_gain_m,
        elev_loss_m: elev_loss_m,
    }
}

fn calculate_segments(points: &Vec<&Waypoint>) -> Vec<GpxSegment> {
    let mut segments = Vec::new();

    let mut current_dist_m = 0.0;
    let mut current_diff_m = 0.0;

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

            current_diff_m += ele_diff * prorate;

            segments.push(GpxSegment::new(
                segments.len() as f64 * SEGMENT_LENGTH / 1000.0,
                SEGMENT_LENGTH,
                current_diff_m,
            ));
            remaining_step_m -= needed_for_segment_m;
            current_dist_m = 0.0;
            current_diff_m = 0.0;
        }

        if remaining_step_m > 0.0 {
            let prorate = remaining_step_m / step_distance;
            current_dist_m += remaining_step_m;
            current_diff_m += ele_diff * prorate;
        }
    }
    segments.push(GpxSegment::new(
        (segments.len() as f64 * SEGMENT_LENGTH + current_dist_m) / 1000.0,
        current_dist_m,
        current_diff_m,
    ));
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

// Computes the grade adjusted pace (GAP) factor
fn gap_factor(dist_m: f64, elev_diff_m: f64) -> f64 {
    if dist_m <= 0.0 {
        return 1.0;
    }

    let grade_percent = ((elev_diff_m / dist_m) * 100.0).clamp(-50.0, 50.0);

    // Minetti (2002) normalized polynomial coefficients for grade in %
    // const C5: f64 =  4.31666667e-09; // ( 155.4 / 3.6) * 1e-10
    // const C4: f64 = -8.44444444e-08; // ( -30.4 / 3.6) * 1e-8
    // const C3: f64 = -1.20277778e-05; // ( -43.3 / 3.6) * 1e-6
    // const C2: f64 =  1.28611111e-03; // (  46.3 / 3.6) * 1e-4
    // const C1: f64 =  5.41666667e-02; // (  19.5 / 3.6) * 1e-2
    // const C0: f64 =  1.000000000;    // (   3.6 / 3.6)

    // Adjusted (inspired from Strava) for higher downhill steep downhill penalty
    const C5: f64 = -5.96892723e-10;
    const C4: f64 = -3.66663630e-07;
    const C3: f64 = -1.66779606e-06;
    const C2: f64 = 1.82471254e-03;
    const C1: f64 = 3.01350192e-02;
    const C0: f64 = 0.997584372;

    C5.mul_add(grade_percent, C4)
        .mul_add(grade_percent, C3)
        .mul_add(grade_percent, C2)
        .mul_add(grade_percent, C1)
        .mul_add(grade_percent, C0)
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

        assert_eq!(segments.len(), 9);

        assert_abs_diff_eq!(segments[0].km, 0.0, epsilon = 0.001);
        assert_abs_diff_eq!(segments[1].km, 0.1, epsilon = 0.001);
        assert_abs_diff_eq!(segments[1].dist_m, SEGMENT_LENGTH, epsilon = 0.001);
        assert_abs_diff_eq!(segments[8].dist_m, 85.878, epsilon = 0.1);

        assert_abs_diff_eq!(
            segments.iter().map(|s| s.elev_diff_m).sum::<f64>(),
            -90.0,
            epsilon = 1.0
        );
    }
}
