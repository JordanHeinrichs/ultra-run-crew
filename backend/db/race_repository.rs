use crate::db::models::Race;
use crate::db::sqlite_repository::SqliteRepository;
use crate::errors::AppError::{self, NotFound};

pub trait RaceRepository {
    async fn find_race_id(&self, id: i64) -> Result<Race, AppError>;
}

impl RaceRepository for SqliteRepository {
    async fn find_race_id(&self, id: i64) -> Result<Race, AppError> {
        sqlx::query_as!(
            Race,
            r#"
            SELECT id, runner_id, name, event_date, is_deleted, distance_km, goal_duration, pacing_strategy, default_aid_station_duration, created_at, updated_at
            FROM races
            WHERE id = ? AND runner_id = ? AND is_deleted = FALSE
            "#,
            id,
            self.user.user_id
        )
        .fetch_one(&self.pool)
        .await.map_err(|_| NotFound("Race not found".into()))
    }
}
