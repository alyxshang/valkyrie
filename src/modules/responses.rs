/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the macro
/// for serializing a 
/// Rust data structure
/// into a JSON string.
use serde::Serialize;

/// A data structure
/// to encapsulate data
/// about a newly created
/// user to be sent back
/// to the client as a JSON
/// string.
#[derive(Serialize)]
pub struct CreateUserResponse {
    pub username: String,
    pub display_name: String,
    pub description: String
}

/// A data structure to encapsulate
/// data about a response containing
/// a user's API token. This data
/// structure is sent as a JSON
/// string.
#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String
}

/// A data structure to encapsulate
/// data about the success or failure
/// of an operation that has been carried
/// out. This data structure is sent as 
/// a JSON string.
#[derive(Serialize)]
pub struct StatusResponse {
    pub is_ok: bool
}

/// A data structure to encapsulate
/// data about the current Valkyrie
/// instance. This data structure 
/// is sent as a JSON string.
#[derive(Serialize)]
pub struct InstanceResponse {
    pub name: String,
    pub admin: String,
    pub description: String
}

/// A data structure to encapsulate
/// data about a created post. This 
/// data structure is sent as 
/// a JSON string.
#[derive(Serialize)]
pub struct PostCreatedResponse {
    pub post_id: String,
    pub created_at: String, 
    pub created_by: String, 
    pub post_content: String
}

/// A data structure to encapsulate
/// data about a Valkyrie user on
/// a certain Valkyrie instance. 
/// This data structure is sent as 
/// a JSON string.
#[derive(Serialize)]
pub struct UserInfoResponse {
    pub user: CreateUserResponse,
    pub posts: Vec<PostCreatedResponse>
}

/// A data structure to encapsulate
/// data about a Valkyrie user's
/// API tokens. This data structure 
/// is sent as  a JSON string.
#[derive(Serialize)]
pub struct UserTokensResponse {
    pub tokens: Vec<String>
}
