use crate::auth_middleware::AuthenticatedUser;
use sqlx::SqlitePool;

pub struct SqliteRepository {
    pub pool: SqlitePool,
    pub user: AuthenticatedUser,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool, user: AuthenticatedUser) -> Self {
        Self { pool, user }
    }
}
