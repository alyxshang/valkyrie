/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the macro
/// for deserializing JSON
/// into a Rust data structure.
use serde::Deserialize;

/// A data structure to
/// encapsulate data for
/// a JSON payload to 
/// create a new Valkyrie
/// user.
#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub description: String,
}

/// A data structure to
/// encapsulate data for
/// a JSON payload to 
/// login a Valkyrie
/// user.
#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String
}

/// A data structure to
/// encapsulate data for
/// a JSON payload to 
/// logout a Valkyrie
/// user.
#[derive(Deserialize)]
pub struct LogoutPayload {
    pub token: String,
}

/// A data structure to
/// encapsulate data for
/// a JSON payload to 
/// make a trivial change
/// to a Valkyrie user's
/// account.
#[derive(Deserialize)]
pub struct ChangePayload {
    pub token: String,
    pub new_value: String,
}

/// A data structure to
/// encapsulate data for
/// a JSON payload to 
/// make a non-trivial change
/// to a Valkyrie user's
/// account securely.
#[derive(Deserialize)]
pub struct SecureChangePayload {
    pub token: String,
    pub password: String,
    pub new_value: String,
}

/// A data structure to
/// encapsulate data for
/// a JSON payload to 
/// create a new post 
/// for a user.
#[derive(Deserialize)]
pub struct PostCreatePayload {
    pub token: String,
    pub content: String
}

/// A data structure to
/// encapsulate data for
/// a JSON payload to 
/// update a post for
/// a user.
#[derive(Deserialize)]
pub struct PostUpdatePayload {
    pub token: String,
    pub content: String,
    pub post_id: String
}

/// A data structure to
/// encapsulate data for
/// a JSON payload to delete
/// a post for a user.
#[derive(Deserialize)]
pub struct PostDeletePayload {
    pub token: String,
    pub post_id: String
}
