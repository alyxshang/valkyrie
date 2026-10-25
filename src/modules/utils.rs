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

/// A function to check whether
/// a supplied username contains
/// valid characters. A boolean
/// is returned to reflect this.
pub fn is_valid_username(
    username: &str
) -> bool {
    let mut result: bool = true;
    let characters: Vec<char> = username
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz_"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    for character in characters {
        if !alphabet.contains(&character){
            result = false;
        }
    }
    result
}
