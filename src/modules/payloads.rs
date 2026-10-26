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

/// An enumeration
/// describing which 
/// federated action
/// is being carried
/// out.
#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub enum Action{
    Create,
    Update,
    Delete
}

/// An enumeration
/// describing which 
/// entity a federated 
/// action is being 
/// carried out on.
#[derive(Deserialize, Serialize, Clone)]
pub enum Entity {
    Post,
    Subscription
}

/// In case a post is being
/// sent from one server to
/// another, this data structure
/// is there to receive that data
/// and be transmitted as a JSON
/// string to the receiver.
#[derive(Deserialize, Serialize, Clone)]
pub struct PostObject {
    pub post_id: String,
    pub created_at: String, 
    pub created_by: String,
    pub created_by_host: String,
    pub post_content: String
}

/// In case a following-followee
/// relationship is being established,
/// this data structure is there to
/// receive that data and be transmitted
/// as a JSON string to the receiver.
#[derive(Deserialize, Serialize, Clone)]
pub struct SubscriberObject {
    pub id: String,
    pub sender_host: String,
    pub sender_handle: String,
    pub receiver_host: String,
    pub receiver_handle: String
}

/// An enumeration that
/// lists all possible
/// objects that can be
/// sent to another Valkyrie
/// server as a JSON object.
#[derive(Deserialize, Serialize, Clone)]
pub enum Object{
    Post(PostObject),
    Subscriber(SubscriberObject)
}

/// A data structure describing
/// a federated action being
/// carried out. This data 
/// structure is deserialized
/// from a JSON string.
#[derive(Deserialize, Serialize, Clone)]
pub struct InboxPayload {
    pub action: Action,
    pub entity: Entity,
    pub object: Object
}
