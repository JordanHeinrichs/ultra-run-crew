use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use ts_rs::TS;

#[derive(sqlx::FromRow, Debug, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub provider: String,
    pub is_active: bool,
    pub password_hash: String,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub id: i64,
    pub runner_id: i64,
    pub name: String,
    #[ts(type = "string")]
    pub event_date: NaiveDate,
    pub distance_km: f64,
    pub goal_duration: Option<f64>,
    pub pacing_strategy: Option<f64>, // [-1, 1] for negative to positive pacing
    pub default_aid_station_duration: Option<f64>,
    pub is_deleted: bool,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AidStation {
    pub id: i64,
    pub race_id: i64,
    pub name: String,
    pub distance_km: f64,
    pub is_crew_allowed: bool,
    pub has_drop_bag: bool,
    pub is_deleted: bool,
    pub arrival_timestamp: Option<i64>,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AidStationChecklistItem {
    pub id: i64,
    pub aid_station_id: i64,
    pub item: String,
    pub checked: bool,
    pub is_deleted: bool,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub id: i64,
    pub race_id: i64,
    pub total_distance_km: f64,
    pub total_elevation_gain_m: f64,
    pub total_elevation_loss_m: f64,
    pub is_deleted: bool,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct RouteSegment {
    pub id: i64,
    pub route_id: i64,
    pub km: f64,
    pub gain_m: f64,
    pub loss_m: f64,
}
