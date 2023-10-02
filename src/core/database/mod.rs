//! Database module
//!
//! This module implement Dragon Bot Z Portal Service repositories and models
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use dbzlib_rs::{
    database::PgDatabase,
    util::exception::{ExcResult, Exception},
    model::portal::{PortalContent, PortalData},
};
use sqlx::Row;
use log::error;

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

    /// Retrieves a portal data and returns it
    ///
    /// # Arguments:
    /// * id - the portal's id
    pub async fn get_data(&self, id: i64) -> ExcResult<PortalData> {
        let result = sqlx::query_as::<_, PortalData>("SELECT * FROM portal WHERE id = $1")
            .bind(id)
            .fetch_one(self.database.pool())
            .await;

        if let Err(error) = result {
            return Err(Exception::RetrievePortal(error.to_string()));
        }

        Ok(result.unwrap())
    }

    /// Retrieves a raw portal content and returns it
    ///
    /// # Arguments:
    /// * id - the portal's id to fetch content from
    pub async fn get_content(&self, id: i64) -> ExcResult<PortalContent> {
        let characters = sqlx::query("SELECT character FROM portal_content WHERE portal = $1")
            .bind(id)
            .fetch_all(self.database.pool())
            .await;

        if let Err(error) = characters {
            error!("{}", error);
            return Err(Exception::RetrievePortalContent(error.to_string()));
        }
        let characters = characters.unwrap();

        let mut character_vec = Vec::<i64>::new();
        for row in characters {
            character_vec.push(row.get(0))
        }

        let portal_content = PortalContent::new(id, character_vec);
        Ok(portal_content)
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
        PortalRepository { database }
    }

    #[tokio::test]
    async fn ok_if_return_retrieve_portal_exception_on_bad_id() {
        let database = init_database().await;
        let repository = init_repository(&database);

        assert!(repository.get_data(-255).await.is_ok())
    }
}
