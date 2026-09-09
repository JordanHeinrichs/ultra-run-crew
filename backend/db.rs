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
    pub runner: i64,
    pub name: String,
    #[ts(type = "string")]
    pub event_date: NaiveDate,
    pub is_deleted: bool,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}
