//! Database module
//!
//! This module implement Dragon Bot Z Portal Service repositories and models
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use dbzlib_rs::{
    database::PgDatabase,
    util::exception::{ExcResult, Exception},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct PortalData {
    id: Option<i64>,
    name: String,
    description: String,
    image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct PortalContent {
    portal: i64,
    characters: Vec<i64>,
}

pub struct PortalRepository<'a> {
    database: &'a PgDatabase,
}

impl<'a> PortalRepository<'a> {
    /// Creates an instance of PortalRepository
    ///
    /// # Arguments:
    /// * database - the Postgres database instance
    pub fn new(database: &'a PgDatabase) -> Self {
        Self { database }
    }

    /// Retrieves a portal and returns it
    ///
    /// # Arguments:
    /// * id - the portal's id
    pub async fn get(&self, id: i64) -> ExcResult<PortalData> {
        let result = sqlx::query_as::<_, PortalData>("SELECT * FROM portal WHERE id = $1")
            .bind(id)
            .fetch_one(self.database.pool())
            .await;

        if let Err(error) = result {
            return Err(Exception::RetrievePortal(error.to_string()));
        }

        Ok(result.unwrap())
    }
}

#[cfg(test)]
mod portal_test {
    use super::*;

    async fn init_database() -> PgDatabase {
        let database = PgDatabase::new("postgresql://postgres@127.0.0.1:5433/portaldb").await;

        if let Err(error) = database {
            panic!("{}", error.to_string())
        }

        database.unwrap()
    }

    fn init_repository(database: &PgDatabase) -> PortalRepository {
        PortalRepository { database: database }
    }

    #[tokio::test]
    async fn ok_if_return_retrieve_portal_exception_on_bad_id() {
        let database = init_database().await;
        let repository = init_repository(&database);

        assert!(repository.get(-255).await.is_ok())
    }
}
