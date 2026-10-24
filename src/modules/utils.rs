/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// structure to
/// utilize a number
/// of simultaneous
/// connections to 
/// a database.
use sqlx::Pool;

/// Importing the structure
/// to get the local current 
/// date and time.
use chrono::Local;

/// Importing the data structure
/// to catch and handle errors.
use super::err::ValkyrieErr;

/// Importing the structure to
/// signal to the `Pool` structure
/// that PostgreSQL database is
/// being connected to.
use sqlx::postgres::Postgres;

/// Importing the structure to have
/// create a connection to a PostgreSQL
/// database.
use sqlx::postgres::PgPoolOptions;

/// A function to create a
/// connection to a PostgreSQL
/// database and return this
/// connection as an instance of
/// the `Pool<Postgres>` structure.
/// If the operation fails, an error
/// is returned.
pub async fn create_connection(
    url: &str
) -> Result<Pool<Postgres>, ValkyrieErr>{
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .connect(url)
        .await?;
    Ok(pool)
}

/// A function to to get the current
/// time as RFC 2822.
pub fn rfc2822() -> String{
    Local::now().to_rfc2822()
}
