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
use sqlx::{FromRow, Row};

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
    pub async fn get_content(&self, id: i64) -> ExcResult<Vec<i64>> {
        let characters = sqlx::query("SELECT character FROM portal_content WHERE portal = $1")
            .bind(id)
            .fetch_all(self.database.pool())
            .await;

        if let Err(error) = characters {
            return Err(Exception::RetrievePortalContent(error.to_string()));
        }
        let characters = characters.unwrap();

        if characters.len() == 0 {
            return Err(Exception::RetrievePortalContent(format!(
                "No character found for Portal #{id}"
            )));
        }

        let mut character_vec = Vec::<i64>::new();
        for row in characters {
            character_vec.push(row.get(0))
        }

        Ok(character_vec)
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
