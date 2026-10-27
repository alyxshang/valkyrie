/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the macro
/// from the "sqlx" crate
/// to deserialize data
/// from a database row
/// into a Rust data
/// structure.
use sqlx::FromRow;

/// A structure to
/// model information
/// about the current
/// Valkyrie instance
/// in the database.
#[derive(FromRow, Clone)]
pub struct Instance{ 
    pub host: String,
    pub name: String,
    pub admin: String,
    pub use_https: bool,
    pub description: String
}

/// A structure to model
/// information about a 
/// user in the database.
#[derive(FromRow, Clone)]
pub struct ValkyrieUser{
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub description: String,
    pub is_admin: bool
}

/// A structure to model
/// information about a post
/// a user has made in the
/// database.
#[derive(FromRow, Clone)]
pub struct ValkyriePost {
    pub post_id: String,
    pub created_at: String, 
    pub created_by: String,
    pub created_by_host: String,
    pub post_content: String
}

/// A structure to model
/// information about a 
/// user's API token
/// in the database.
#[derive(FromRow, Clone)]
pub struct APIToken {
    pub owner: String,
    pub token: String,
}

/// A table to model
/// information about
/// a following-followee
/// relationship in the
/// database.
#[derive(FromRow, Clone)]
pub struct Follow {
  pub id: String,
  pub sender_host: String,
  pub sender_handle: String,
  pub receiver_host: String,
  pub receiver_handle: String
}
