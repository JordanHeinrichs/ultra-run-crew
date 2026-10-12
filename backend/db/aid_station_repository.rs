use crate::db::models::AidStation;
use crate::db::sqlite_repository::SqliteRepository;
use crate::errors::AppError::{self, NotFound};

pub trait AidStationRepository {
    async fn find_aid_station(&self, id: i64) -> Result<AidStation, AppError>;
    async fn list_aid_stations(&self, race_id: i64) -> Result<Vec<AidStation>, AppError>;
}

impl AidStationRepository for SqliteRepository {
    async fn find_aid_station(&self, id: i64) -> Result<AidStation, AppError> {
        sqlx::query_as!(
          AidStation,
          r#"
          SELECT a.id, a.race_id, a.name, a.distance_km, a.is_crew_allowed, a.has_drop_bag, a.is_deleted, a.arrival_timestamp, a.created_at, a.updated_at
          FROM aid_stations a
          WHERE a.id = ? AND a.is_deleted = FALSE
          "#,
          id,
      )
        .fetch_one(&self.pool)
        .await.map_err(|_| NotFound("Aid station not found".into()))
    }

    async fn list_aid_stations(&self, race_id: i64) -> Result<Vec<AidStation>, AppError> {
        sqlx::query_as!(
          AidStation,
          r#"
          SELECT a.id, a.race_id, a.name, a.distance_km, a.is_crew_allowed, a.has_drop_bag, a.is_deleted, a.arrival_timestamp, a.created_at, a.updated_at
          FROM aid_stations a
          WHERE a.is_deleted = FALSE AND a.race_id = ?
          ORDER BY a.distance_km ASC
          "#,
          race_id,
      )
        .fetch_all(&self.pool)
        .await.map_err(|_| NotFound("Aid station not found".into()))
    }
}
