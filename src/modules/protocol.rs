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

/// Importing the
/// macro to accept
/// GET requests from
/// the Actix Web
/// crate.
use actix_web::get;

/// Importing the structure
/// to send more complex
/// HTTP requests.
use reqwest::Client;

/// Importing the
/// macro to accept
/// POST requests from
/// the Actix Web
/// crate.
use actix_web::post;

/// Importing the structure
/// that encapsulates 
/// response data from
/// an HTTP request.
use reqwest::Response;

/// Importing the structure
/// to accept JSON payloads
/// in POST requests.
use actix_web::web::Json;

/// Importing the structure
/// to accept URL path patterns.
use actix_web::web::Path;

/// Importing the structure
/// to store persistent app
/// data for the app's database
/// pool.
use actix_web::web::Data;

/// Importing the function
/// to create a new record
/// for a post in the 
/// database.
use super::db::create_post;

/// Importing the function
/// to update an existing
/// record for a post in the 
/// database.
use super::db::update_post;

/// Importing the function
/// to delete an existing
/// record for a post in the 
/// database.
use super::db::delete_post;

/// Importing the data structure
/// to catch and handle errors.
use super::err::ValkyrieErr;

/// Importing the structure
/// to return HTTP responses
/// from a service function
/// for the REST API.
use actix_web::HttpResponse;

/// Importing the function to 
/// retrieve information on 
/// the current Valkyrie instance.
use super::db::get_instance;

/// Importing the enumeration
/// describing which type of
/// entity a federated action
/// is being carried out.
use super::payloads::Entity;

/// Importing the structure 
/// encapsulating data about 
/// a cross-server 
/// posting action.
use super::payloads::Object;

/// Importing the enumeration
/// describing which type of
/// federated action is being
/// carried out.
use super::payloads::Action;

/// Importing the data structure
/// that models information about
/// the current Valkyrie instance
/// in the database.
use super::models::Instance;

/// Importing the structure to
/// signal to the `Pool` structure
/// that PostgreSQL database is
/// being connected to.
use sqlx::postgres::Postgres;
 
/// Importing the funciton
/// to write a new record
/// for a following-followee
/// relationship into the
/// database.
use super::db::create_follow;

/// Importing the function to 
/// retrieve all the posts a
/// user has created to test it.
use super::db::get_user_posts;

/// Importing the function to
/// retrieve the record of a 
/// Valkyrie user given their
/// username.
use super::db::get_user_by_name;

/// Importing the data structure
/// that models information about
/// a user in the database.
use super::models::ValkyrieUser;

/// Importing the structure 
/// encapsulating data about 
/// a cross-server 
/// posting action.
use super::payloads::PostObject;

/// Importing the data structure
/// that models information about
/// a post a user has made in the 
/// database.
use super::models::ValkyriePost;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for carrying
/// out a federated action.
use super::payloads::InboxPayload;

/// Importing the data structure to 
/// encapsulate data about the success 
/// or failure of an operation. This 
/// data structure is sent 
/// as a JSON string.
use super::responses::StatusResponse;

/// Importing the structure 
/// encapsulating data about 
/// a cross-server 
/// following-followee
/// relationship.
use super::payloads::SubscriberObject;

/// Importing the data structure 
/// to encapsulate data for the 
/// Valkyrie protocol about a user 
/// and their posts. This data 
/// structure is sent as  a JSON 
/// string.
use super::responses::UserInfoResponse;

/// Importing the data structure 
/// to encapsulate data about a 
/// created post. This data structure 
/// is sent as a JSON string.
use super::responses::PostCreatedResponse;

/// Importing the function
/// to delete an existing record
/// for a following-followee relationship 
/// from the database.
use super::db::delete_follow_by_participants;

/// A service function for a Valkyrie
/// server that accepts federated actions.
/// Actions have an `Action` field, and an
/// `object` field. The former describes actions
/// like creating, updating, and deleting objects.
/// The latter carries the object to be deleted
/// or created. These can be posts or following-
/// followee relationships. If the action is
/// successful, a status response is returned
/// containing a success or failure code in
/// the form of a boolean. The response is
/// in JSON format and the string is built
/// from an instance of the `StatusResponse`
/// data structure. If the operation fails, an
/// error response is returned.
#[post("/protocol/{user}/inbox")]
pub async fn user_inbox_service(
    payload: Json<InboxPayload>,
    user: Path<String>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        &user.into_inner(),
        &data
    ).await?;
    let status: bool;
    match payload.entity {
        Entity::Post => {
            let object: PostObject = match payload.object.clone() {
                Object::Post(p) => p,
                _ => return Err::<HttpResponse, ValkyrieErr>(
                    ValkyrieErr::new("Unrecognized object.")
                )
            };
            if payload.action == Action::Create {
                status = match create_post(
                    &Some(&object.post_id),
                    &Some(&object.created_at),
                    &object.created_by,
                    &Some(&object.created_by_host),
                    &object.post_content,
                    &data
                ).await {
                    Ok(_f) => true,
                    Err(_e) => false
                };
            }
            else if payload.action == Action::Update {
                status = match update_post(
                    &object.post_id,
                    &object.created_by_host,
                    &object.post_content,
                    &data
                ).await {
                    Ok(_f) => true,
                    Err(_e) => false
                };
            }
            else if payload.action == Action::Delete {
                status = match delete_post(
                    &object.post_id,
                    &object.created_by_host,
                    &data
                ).await {
                    Ok(_f) => true,
                    Err(_e) => false
                };
            }
            else {
                status = false;
            }
        }
        Entity::Subscription => {
            let s: SubscriberObject = match payload.object.clone() {
                Object::Subscriber(s) => s,
                _ => return Err::<HttpResponse, ValkyrieErr>(
                    ValkyrieErr::new("Unrecognized object.")
                )
            };
            let instance: Instance = get_instance(&data).await?;
            if payload.action == Action::Create {
                status = match create_follow(
                    &Some(&s.id),
                    &s.sender_host,
                    &s.sender_handle,
                    &instance.host,
                    &user.username,
                    &data
                ).await {
                    Ok(_f) => true,
                    Err(_e) => false
                };
            }
            else if payload.action == Action::Delete {
                status = match delete_follow_by_participants(
                    &s.sender_host,
                    &s.sender_handle,
                    &instance.host,
                    &user.username,
                    &data
                ).await {
                    Ok(_f) => true,
                    Err(_e) => false
                };
            }
            else {
                status = false;
            }
        }, 
    }
    let status_response: StatusResponse = StatusResponse{
        is_ok: status
    };
    Ok(HttpResponse::Ok().json(status_response))
}

/// A service function for a Valkyrie
/// server that sends back information
/// about a user and the posts they have
/// made. Other servers can then fetch and
/// store this information. The response is
/// in JSON format and the string is built
/// from an instance of the `UserInfoResponse`
/// data structure. If the operation fails, an
/// error response is returned.
#[get("/protocol/{user}")]
pub async fn user_info_service(
    path: Path<String>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        &path.into_inner(),
        &data
    ).await?;
    let instance: Instance = get_instance(&data)
        .await?;
    let inbox_url: String = match instance.use_https{
        true => format!(
            "https://{}/protocol/{}/inbox",
            instance.host,
            user.username
        ),
        false => format!(
            "http://{}/protocol/{}/inbox",
            instance.host,
            user.username
        )
    };
    let posts: Vec<ValkyriePost> = get_user_posts(
        &user.username,
        &instance.host,
        &data
    ).await?;
    let mut res_posts: Vec<PostCreatedResponse> = Vec::new();
    for post in &posts {
        let res_post: PostCreatedResponse = PostCreatedResponse{
            post_id: post.post_id.clone(),
            created_at: post.created_at.clone(),
            created_by: post.created_by.clone(),
            post_content: post.post_content.clone()
        };
        res_posts.push(res_post);
    }
    let response: UserInfoResponse = UserInfoResponse {
        username: user.username.clone(),
        display_name: user.display_name.clone(),
        description: user.description.clone(),
        inbox_url: inbox_url,
        post_count: posts.clone().len(),
        posts: res_posts
    };
    Ok(HttpResponse::Ok().json(response))
}

/// A function to query a user's
/// public federated information.
/// If the operation succeeds,
/// an instance of the `UserInforesponse`
/// is returned. If the operation fails,
/// an error is returned.
pub async fn fetch_user_info(
    host: &str,
    username: &str,
) -> Result<UserInfoResponse, ValkyrieErr>{
    let client: Client = Client::new();
    let url: String = format!(
        "{}/protocol/{}",
        host,
        username
    );
    let resp: Response = client.get(url)
        .send()
        .await?;
    let result: UserInfoResponse = resp.json().await?;
    Ok(result)
}

/// A function to query a user's
/// public federated information
/// and obtain the public inbox
/// URL. If the operation succeeds,
/// a string containing the URL is
/// returned. If the operation fails,
/// an error is returned.
pub async fn fetch_user_inbox_url(
    host: &str,
    username: &str,
) -> Result<String, ValkyrieErr>{
    let client: Client = Client::new();
    let url: String = format!(
        "{}/protocol/{}",
        host,
        username
    );
    let resp: Response = client.get(
        url
    ).send().await?;
    let result: UserInfoResponse = resp.json().await?;
    Ok(result.inbox_url)
}

/// A function to send a message
/// to a user's inbox. The message
/// sent is an instance of the 
/// `InboxPayload` structure. If
/// the operation succeeds, a boolean
/// is returned to signal the success
/// or failure of the sending of the
/// message.
pub async fn send_message_to_inbox(
    url: &str,
    message: &InboxPayload
) -> Result<bool, ValkyrieErr>{
    let client: Client = Client::new();
    let resp: Response = client.post(url)
        .json(&message)
        .send()
        .await?;
    let result: StatusResponse = resp.json().await?;
    Ok(result.is_ok)
}
