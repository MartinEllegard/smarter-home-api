use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::custom_error::ShError;

pub struct Db {
    pool: PgPool,
}
impl Db {
    pub async fn new(connection_string: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(connection_string)
            .await
            .unwrap();

        Db { pool }
    }
    pub async fn migrate(&mut self) -> Result<(), ShError> {
        let mut tx = self.pool.begin().await.map_err(custom_error)?;

        sqlx::migrate!()
            .run(&mut tx)
            .await
            .map_err(|err| ShError::Sql(err.to_string()))?;

        tx.commit().await.map_err(custom_error)
    }
}

pub fn custom_error(err: sqlx::Error) -> ShError {
    ShError::Sql(err.to_string())
}
