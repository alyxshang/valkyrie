/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the macro
/// for serializing a 
/// Rust data structure
/// into a JSON string.
use serde::Serialize;

/// Importing the macro
/// for deserializing JSON
/// into a Rust data structure.
use serde::Deserialize;

/// A data structure
/// to encapsulate data
/// about a newly created
/// user to be sent back
/// to the client as a JSON
/// string.
#[derive(Deserialize, Serialize)]
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
#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    pub token: String
}

/// A data structure to encapsulate
/// data about the success or failure
/// of an operation that has been carried
/// out. This data structure is sent as 
/// a JSON string.
#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub is_ok: bool
}

/// A data structure to encapsulate
/// data about the current Valkyrie
/// instance. This data structure 
/// is sent as a JSON string.
#[derive(Deserialize, Serialize)]
pub struct InstanceResponse {
    pub name: String,
    pub admin: String,
    pub description: String
}

/// A data structure to encapsulate
/// data about a created post. This 
/// data structure is sent as 
/// a JSON string but can also
/// be received as a JSON string.
#[derive(Serialize, Deserialize)]
pub struct PostCreatedResponse {
    pub post_id: String,
    pub created_at: String, 
    pub created_by: String, 
    pub post_content: String
}

/// A data structure to encapsulate
/// data for the Valkyrie protocol
/// about a user and their posts. 
/// This data structure is sent as 
/// a JSON string but can also
/// be received as a JSON string.
#[derive(Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub username: String,
    pub display_name: String,
    pub description: String,
    pub inbox_url: String,
    pub post_count: usize,
    pub posts: Vec<PostCreatedResponse>
}

/// A data structure to encapsulate
/// data about a Valkyrie user's
/// API tokens. This data structure 
/// is sent as  a JSON string.
#[derive(Deserialize, Serialize)]
pub struct UserTokensResponse {
    pub tokens: Vec<String>
}

/// A data structure to encapsulate
/// data about a Valkyrie user's timeline.
/// This data structure 
/// is sent as  a JSON string.
#[derive(Deserialize, Serialize)]
pub struct TimeLinePost {
    pub post_id: String,
    pub created_at: String, 
    pub created_by: String, 
    pub post_content: String,
    pub author_instance: String,
    pub author_display_name: String
}

/// A data structure to encapsulate
/// data about a Valkyrie user's timeline.
/// This data structure 
/// is sent as  a JSON string.
#[derive(Deserialize, Serialize)]
pub struct TimeLineResponse {
    pub posts: Vec<TimeLinePost>
}
