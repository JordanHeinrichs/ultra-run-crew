use crate::db::models::Course;
use crate::db::sqlite_repository::SqliteRepository;
use crate::errors::AppError::{self, NotFound};

pub trait CourseRepository {
    async fn find_course(&self, id: i64) -> Result<Course, AppError>;
}

impl CourseRepository for SqliteRepository {
    async fn find_course(&self, id: i64) -> Result<Course, AppError> {
        sqlx::query_as!(
          Course,
          r#"
          SELECT id, race_id, total_distance_km, total_elevation_gain_m, total_elevation_loss_m, is_deleted, created_at, updated_at
          FROM courses
          WHERE id = ?
          "#,
          id,
      )
      .fetch_one(&self.pool)
      .await.map_err(|_| NotFound("Course not found".into()))
    }
}
