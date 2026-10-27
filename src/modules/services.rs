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

/// Importing the
/// macro to accept
/// POST requests from
/// the Actix Web
/// crate.
use actix_web::post;

/// Importing the structure
/// to sort items 
/// chronologically.
use chrono::DateTime;

/// Importing the structure
/// to accept JSON payloads
/// in POST requests.
use actix_web::web::Json;

/// Importing the structure
/// that models a following-followee
/// relationship in the database.
use super::models::Follow;

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

/// Importing the function from
/// the database to delete the
/// account of a Valkyrie user.
use super::db::delete_user;

/// Importing the function
/// to delete an existing
/// record for a post in the 
/// database.
use super::db::delete_post;

/// Importing the function to
/// check if a record for a user
/// with the specified username
/// already exists.
use super::db::user_exists;

/// Importing the function to 
/// create a record for a user. 
use super::db::create_user;

/// Importing the data structure
/// to catch and handle errors.
use super::err::ValkyrieErr;

/// Importing the enumeration
/// describing which type of
/// entity a federated action
/// is being carried out on.
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

/// Importing the function to 
/// retrieve information on 
/// the current Valkyrie instance.
use super::db::get_instance;

/// Importing the data structure
/// that models information about
/// the current Valkyrie instance
/// in the database.
use super::models::Instance;

/// Importing the structure
/// to return HTTP responses
/// from a service function
/// for the REST API.
use actix_web::HttpResponse;

/// Importing the data structure
/// that models information about
/// an API token for a user in the 
/// database.
use super::models::APIToken;

/// Importing the function
/// to create a new record
/// in the database for a
/// following-followee
/// relationship.
use super::db::create_follow;

/// Importing the structure to
/// signal to the `Pool` structure
/// that PostgreSQL database is
/// being connected to.
use sqlx::postgres::Postgres;

/// Importing the function to
/// retrieve the record for a
/// post, given the post's ID.
use super::db::get_post_by_id;

/// Importing the function to 
/// retrieve all the posts a
/// user has created to test it.
use super::db::get_user_posts;

/// Importing the function to
/// get a list of all the tokens
/// a Valkyrie user has created.
use super::db::get_user_tokens;

/// Importing the function to 
/// fetch the record for a user.
use super::db::get_user_by_name;

/// Importing the function to
/// create a new record for an
/// API token for a Valkyrie 
/// user.
use super::db::create_api_token;

/// Importing the function to
/// delete a record for an
/// API token created for a 
/// Valkyrie user.
use super::db::delete_api_token;

/// Importing the data structure
/// that models information about
/// a user in the database.
use super::models::ValkyrieUser;

/// Importing the data structure
/// that models information about
/// a post a user has made in the 
/// database.
use super::models::ValkyriePost;

/// Importing the structure 
/// encapsulating data about 
/// a cross-server 
/// posting action.
use super::payloads::PostObject;

/// Importing the function to
/// retrieve the record for a 
/// Valkyrie user given one of
/// their tokens.
use super::db::get_user_by_token;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for logging in
/// a Valkyrie user.
use super::payloads::LoginPayload;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for carrying
/// out a federated action.
use super::payloads::InboxPayload;

/// Importing the function to retrieve 
/// records for all the users a user 
/// is followed by.
use super::db::get_user_followers;

/// Importing the function to retrieve 
/// records for all the users a user 
/// is following.
use super::db::get_user_following;

/// Importing the function to get
/// the record for an API token
/// from the database.
use super::db::get_token_by_token;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for making trivial
/// changes to a Valkyrie user's account.
use super::payloads::ChangePayload;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for logging out
/// a Valkyrie user.
use super::payloads::LogoutPayload;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on a post contained in a user's timeline.
use super::responses::TimeLinePost;

/// Importing the function to test
/// whether a supplied username
/// only contains valid characters
/// to test it.
use super::utils::is_valid_username;

/// Importing the data structure to 
/// encapsulate data about a response 
/// containing a user's API token. 
/// This data structure is sent as 
/// a JSON string.
use super::responses::TokenResponse;

/// Importng the function to update
/// the name of current the Valkyrie 
/// instance's record in the database.
use super::db::update_instance_name;

/// Importng the function to update
/// the password of a Valkyrie user's
/// record in the database.
use super::db::update_user_password;

/// Importing the function to fetch
/// information on a user's federated
/// profile.
use super::protocol::fetch_user_info;

/// Importing the data structure to encapsulate
/// data about the success or failure of an 
/// operation. This data structure is sent 
/// as a JSON string.
use super::responses::StatusResponse;

/// Importing the structure 
/// encapsulating data about 
/// a cross-server 
/// following-followee
/// relationship.
use super::payloads::SubscriberObject;

/// Importing the data structure
/// that models data to be deserialized
/// from a JSON payload for creating
/// a Valkyrie user.
use super::payloads::CreateUserPayload;

/// Importng the function to update
/// the description of a Valkyrie user's
/// record in the database.
use super::db::update_user_description;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on a user's timeline.
use super::responses::TimeLineResponse;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on the current Valkyrie instance.
use super::responses::InstanceResponse;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for creating
/// a new post.
use super::payloads::PostCreatePayload;

/// Importing the data structure 
/// to encapsulate data for the 
/// Valkyrie protocol about a user 
/// and their posts. This data 
/// structure is sent as  a JSON 
/// string.
use super::responses::UserInfoResponse;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for deleting
/// an existing post.
use super::payloads::PostDeletePayload;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for updating
/// an existing post.
use super::payloads::PostUpdatePayload;

/// Importng the function to update
/// the display name of a Valkyrie user's
/// record in the database.
use super::db::update_user_display_name;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for making non-trivial
/// changes to a Valkyrie user's account securely.
use super::payloads::SecureChangePayload;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on a newly-created Valkyrie user.
use super::responses::CreateUserResponse;

/// Importing the data structure
/// that models data to be deserialized
/// from a JSON payload for following or
/// unfollowing a Valkyrie user.
use super::payloads::FollowActionPayload;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on all of a Valkyrie user's API tokens.
use super::responses::UserTokensResponse;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on a post a Valkyrie user has created.
use super::responses::PostCreatedResponse;

/// Importing the function to fetch the
/// public inbox url of a user.
use super::protocol::fetch_user_inbox_url;

/// Importing the function
/// to retrieve a record
/// in the database for a
/// following-followee
/// relationship.
use super::db::get_follow_by_participants;

/// Importing the function to send messages
/// to a user's inbox.
use super::protocol::send_message_to_inbox;

/// Importng the function to update
/// the description of the current Valkyrie 
/// instance's record in the database.
use super::db::update_instance_description;

/// Importing the function
/// to delete a record
/// in the database for a
/// following-followee
/// relationship.
use super::db::delete_follow_by_participants;

/// The service function to retrieve information
/// on the current Valkyrie instance.
/// If the operation succeeds, an HTTP response 
/// containing this information is sent back
/// to the client. If the operation fails, 
/// an error is returned.
/// HTTP VERB: GET
#[get("/api/instance")]
pub async fn instance_info_service(
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let instance: Instance = get_instance(&data).await?;
    let response: InstanceResponse = InstanceResponse {
        name: instance.name,
        admin: instance.admin,
        description: instance.description
    };
    Ok(HttpResponse::Ok().json(response))
}

/// The service function to edit the name
/// of a Valkyrie instance record in the database.
/// If the operation succeeds, an HTTP response 
/// containing a status code in the form of a 
/// boolean is sent back to the client. If the 
/// operation fails, an error is returned.
/// HTTP VERB: POST
#[post("/api/instance/edit/name")]
pub async fn edit_instance_name_service(
    payload: Json<ChangePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    if user.is_admin {
        let uo: bool = match update_instance_name(
            &payload.new_value,
            &data
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
        let response: StatusResponse = StatusResponse{
            is_ok: uo
        };
        Ok(HttpResponse::Ok().json(response))
    }
    else {
        Err::<HttpResponse, ValkyrieErr>(
            ValkyrieErr::new("User does not have the neccessary privileges.")
        )
    }
}

/// The service function to edit the description
/// of a Valkyrie instance record in the database.
/// If the operation succeeds, an HTTP response 
/// containing a status code in the form of a 
/// boolean is sent back to the client. If the 
/// operation fails, an error is returned.
/// HTTP VERB: POST
#[post("/api/instance/edit/description")]
pub async fn edit_instance_description_service(
    payload: Json<ChangePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    if user.is_admin {
        let uo: bool = match update_instance_description(
            &payload.new_value,
            &data
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
        let response: StatusResponse = StatusResponse{
            is_ok: uo
        };
        Ok(HttpResponse::Ok().json(response))
    }
    else {
        Err::<HttpResponse, ValkyrieErr>(
            ValkyrieErr::new("User does not have the neccessary privileges.")
        )
    }
}

/// The service function to create a new
/// Valkyrie user record in the database. 
/// The user is not an administrator by 
/// default. If the operation succeeds,
/// an HTTP response containing information
/// on the created user is returned. If the
/// operation fails, an error is returned.
/// HTTP VERB: POST
#[post("/api/user/create")]
pub async fn create_user_service(
    payload: Json<CreateUserPayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    if is_valid_username(&payload.username) &&
       payload.username.len() <= 16 &&
       payload.password.len() <= 32 &&
       !user_exists(&payload.username, &data).await
    {
        let new_user: ValkyrieUser = create_user(
            &payload.username,
            &payload.password,
            &payload.display_name,
            &payload.description,
            false,
            &data
        ).await?;
        let response: CreateUserResponse = CreateUserResponse{
            username: new_user.username,
            display_name: new_user.display_name,
            description: new_user.description
        };
        Ok(HttpResponse::Ok().json(response))
    }
    else {
        Err::<HttpResponse, ValkyrieErr>(
            ValkyrieErr::new(
                "Either username or password were of invalid formats or the user already exists."
            )
        )
    }
}

/// The service function to login a
/// Valkyrie user. If the operation succeeds,
/// an HTTP response containing an API token
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/login")]
pub async fn login_user_service(
    payload: Json<LoginPayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let token: APIToken = create_api_token(
        &payload.username,
        &payload.password,
        &data
    ).await?;
    let response: TokenResponse = TokenResponse{
        token: token.token
    };
    Ok(HttpResponse::Ok().json(response))
}

/// The service function to logout a
/// Valkyrie user. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/logout")]
pub async fn logout_user_service(
    payload: Json<LogoutPayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let fetched: APIToken = get_token_by_token(
        &payload.token,
        &data
    ).await?;
    let status: bool = match delete_api_token(
        &fetched.token,
        &data
    ).await {
        Ok(_f) => true,
        Err(_e) => false
    };
    let response: StatusResponse = StatusResponse{
        is_ok: status
    };
    Ok(HttpResponse::Ok().json(response))
}


/// The service function to edit a user's
/// display name. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/edit/name")]
pub async fn edit_user_display_name_service(
    payload: Json<ChangePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let uo: bool = match update_user_display_name(
        &user.username,
        &payload.new_value,
        &data
    ).await {
        Ok(_f) => true,
        Err(_e) => false
    };
    let response: StatusResponse = StatusResponse{
        is_ok: uo
    };
    Ok(HttpResponse::Ok().json(response))
}

/// The service function to edit a user's
/// description. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/edit/description")]
pub async fn edit_user_description_service(
    payload: Json<ChangePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let uo: bool = match update_user_description(
        &user.username,
        &payload.new_value,
        &data
    ).await {
        Ok(_f) => true,
        Err(_e) => false
    };
    let response: StatusResponse = StatusResponse{
        is_ok: uo
    };
    Ok(HttpResponse::Ok().json(response))
}

/// The service function to edit a user's
/// password. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/edit/password")]
pub async fn edit_user_password_service(
    payload: Json<SecureChangePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let uo: bool = match update_user_password(
        &user.username,
        &payload.password,
        &payload.new_value,
        &data
    ).await {
        Ok(_f) => true,
        Err(_e) => false
    };
    let response: StatusResponse = StatusResponse{
        is_ok: uo
    };
    Ok(HttpResponse::Ok().json(response))
}

/// The service function to create a new
/// post for a Valkyrie user. If the operation 
/// succeeds, an HTTP response containing in the
/// information on the newly-created post is
/// returned. If the operation fails, an 
/// error is returned.
/// HTTP VERB: POST
#[post("/api/post/create")]
pub async fn post_create_service(
    payload: Json<PostCreatePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let user_followers: Vec<Follow> = get_user_followers(
        &user.username,
        &data
    ).await?;
    let instance: Instance = get_instance(&data)
        .await?; 
    let new_post: ValkyriePost = create_post(
        &None,
        &None,
        &user.username,
        &None,
        &payload.content,
        &data
    ).await?;
    for follower in user_followers {
        if follower.sender_host != instance.host {
            let follower_inbox_url: String = fetch_user_inbox_url(
                &follower.sender_host,
                &follower.sender_handle
            ).await?;
            let _msg = send_message_to_inbox(
                &follower_inbox_url,
                &InboxPayload{
                action: Action::Create,
                entity: Entity::Post,
                object: Object::Post(
                    PostObject {
                        post_id: new_post.post_id.clone(),
                        created_at: new_post.created_at.clone(),
                        created_by: new_post.created_by.clone(),
                        created_by_host: new_post.created_by_host.clone(),
                        post_content: new_post.post_content.clone()
                    }
                )
            }
            ).await?;
        }
    }
    let response: PostCreatedResponse = PostCreatedResponse{
        post_id: new_post.post_id,
        created_at: new_post.created_at,
        created_by: new_post.created_by,
        post_content: new_post.post_content

    };
    Ok(HttpResponse::Ok().json(response))
}

/// The service function to delete a post
/// a user has made. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/post/delete")]
pub async fn post_delete_service(
    payload: Json<PostDeletePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let instance: Instance = get_instance(&data)
        .await?;
    let post: ValkyriePost = get_post_by_id(
        &payload.post_id,
        &instance.host,
        &data
    ).await?; 
    if post.created_by == user.username {
        let user_followers: Vec<Follow> = get_user_followers(
            &user.username,
            &data
        ).await?;
        for follower in user_followers {
            if follower.sender_host != instance.host {
                let follower_inbox_url: String = fetch_user_inbox_url(
                    &follower.sender_host,
                    &follower.sender_handle
                ).await?;
                let _msg = send_message_to_inbox(
                    &follower_inbox_url,
                    &InboxPayload{
                        action: Action::Delete,
                        entity: Entity::Post,
                        object: Object::Post(
                            PostObject {
                                post_id: post.post_id.clone(),
                                created_at: post.created_at.clone(),
                                created_by: post.created_by.clone(),
                                created_by_host: post.created_by_host.clone(),
                                post_content: post.post_content.clone()
                            }
                        )
                    }
                ).await?;
            }
        }
        let d_o: bool = match delete_post(
            &post.post_id,
            &instance.host,
            &data
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
        let response: StatusResponse = StatusResponse{
            is_ok: d_o
        };
        Ok(HttpResponse::Ok().json(response))
    }
    else {
        Err::<HttpResponse, ValkyrieErr>(
            ValkyrieErr::new("Post ownership could not be verified.")
        )
    }
}

/// The service function to update a post
/// a user has made. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/post/update")]
pub async fn post_update_service(
    payload: Json<PostUpdatePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let instance: Instance = get_instance(&data)
        .await?;
    let post: ValkyriePost = get_post_by_id(
        &payload.post_id,
        &instance.host,
        &data
    ).await?;
    if post.created_by == user.username {
        let uo: bool = match update_post(
            &post.post_id,
            &instance.host,
            &payload.content,
            &data
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
        let user_followers: Vec<Follow> = get_user_followers(
            &user.username,
            &data
        ).await?;
        for follower in user_followers {
            if follower.sender_host != instance.host {
                let follower_inbox_url: String = fetch_user_inbox_url(
                    &follower.sender_host,
                    &follower.sender_handle
                ).await?;
                let _msg = send_message_to_inbox(
                    &follower_inbox_url,
                    &InboxPayload{
                        action: Action::Update,
                        entity: Entity::Post,
                        object: Object::Post(
                            PostObject {
                                post_id: post.post_id.clone(),
                                created_at: post.created_at.clone(),
                                created_by: post.created_by.clone(),
                                created_by_host: post.created_by_host.clone(),
                                post_content: post.post_content.clone()
                            }
                        )
                    }
                ).await?;
            }
        }
        let response: StatusResponse = StatusResponse{
            is_ok: uo
        };
        Ok(HttpResponse::Ok().json(response))
    }
    else {
        Err::<HttpResponse, ValkyrieErr>(
            ValkyrieErr::new("Post ownership could not be verified.")
        )
    }
}

/// The service function to query all API
/// tokens created and active by and for a
/// Valkyrie user. If the operation succeeds,
/// an HTTP response containing a dictionary
/// containing a list of these tokens is
/// returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/tokens/list")]
pub async fn list_user_tokens_service(
    payload: Json<LogoutPayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let tokens: Vec<APIToken> = get_user_tokens(
        &user.username,
        &data
    ).await?;
    let mut res_strings: Vec<String> = Vec::new();
    for token in tokens { 
        res_strings.push(token.token.clone());
    }
    let response: UserTokensResponse = UserTokensResponse{
        tokens: res_strings
    };
    Ok(HttpResponse::Ok().json(response))
}

/// The service function to follow another
/// Valkyrie user. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/follow")]
pub async fn follow_user_service(
    payload: Json<FollowActionPayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let instance: Instance = get_instance(&data)
        .await?;
    let inbox_url: String = fetch_user_inbox_url(
        &payload.receiver_host,
        &payload.receiver_handle
    ).await?;
    let follow_rel: Follow = create_follow(
        &None,
        &instance.host,
        &user.username,
        &payload.receiver_host,
        &payload.receiver_handle,
        &data
    ).await?;
    if payload.receiver_host != instance.host {
        let _msg: bool = match send_message_to_inbox(
            &inbox_url,
            &InboxPayload{
                action: Action::Create,
                entity: Entity::Subscription,
                object: Object::Subscriber(
                    SubscriberObject {
                        id: follow_rel.id,
                        sender_host: follow_rel.sender_host,
                        sender_handle: follow_rel.sender_handle,
                        receiver_host: follow_rel.receiver_host,
                        receiver_handle: follow_rel.receiver_handle
                    }
                )
            }
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
    } 
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: true}))
}

/// The service function to unfollow another
/// Valkyrie user. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/unfollow")]
pub async fn unfollow_user_service(
    payload: Json<FollowActionPayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let instance: Instance = get_instance(&data)
        .await?;
    let follow_rel: Follow = get_follow_by_participants(
        &instance.host,
        &user.username,
        &payload.receiver_host,
        &payload.receiver_handle,
        &data
    ).await?;
    let del_op: bool = match delete_follow_by_participants(
        &instance.host,
        &user.username,
        &payload.receiver_host,
        &payload.receiver_handle,
        &data
    ).await {
        Ok(_f) => true,
        Err(_e) => false
    };
    if payload.receiver_host != instance.host {
        let inbox_url: String = fetch_user_inbox_url(
            &payload.receiver_host,
            &payload.receiver_handle
        ).await?;
        let _msg: bool = match send_message_to_inbox(
            &inbox_url,
            &InboxPayload{
                action: Action::Delete,
                entity: Entity::Subscription,
                object: Object::Subscriber(
                    SubscriberObject {
                        id: follow_rel.id,
                        sender_host: follow_rel.sender_host,
                        sender_handle: follow_rel.sender_handle,
                        receiver_host: follow_rel.receiver_host,
                        receiver_handle: follow_rel.receiver_handle
                    }
                )
            }
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
    }
    let del_response: StatusResponse = StatusResponse{
        is_ok: del_op
    };
    Ok(HttpResponse::Ok().json(del_response))
}

/// The service function to query all posts
/// of all users a Valkyrie user is following.
/// If the operation succeeds, an HTTP response 
/// containing a dictionary containing a list
/// containing each post with salient details
/// about the user who posted them is returned. 
/// If the operation fails, an error is returned.
/// HTTP VERB: POST
#[post("/api/user/timeline")]
pub async fn timeline_service(
    payload: Json<LogoutPayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let instance: Instance = get_instance(&data)
        .await?;
    let user_following: Vec<Follow> = get_user_following(
        &user.username,
        &data
    ).await?;
    let mut result_posts: Vec<TimeLinePost> = Vec::new();
    for following in user_following {
        if following.receiver_host != instance.host {
            let info: UserInfoResponse = fetch_user_info(
                &following.receiver_host,
                &following.receiver_handle
            ).await?;
            let posts: Vec<ValkyriePost> = get_user_posts(
                &following.receiver_handle,
                &following.receiver_host,
                &data
            ).await?;
            for post in posts {
                let item: TimeLinePost = TimeLinePost{
                    post_id: post.post_id,
                    created_at: post.created_at,
                    created_by: post.created_by,
                    post_content: post.post_content,
                    author_instance: following.receiver_host.clone(),
                    author_display_name: info.display_name.clone()
                };
                result_posts.push(item);
            }
        }
        else {
            let posts: Vec<ValkyriePost> = get_user_posts(
                &following.receiver_handle,
                &instance.host,
                &data
            ).await?;
            let info: ValkyrieUser = get_user_by_name(
                &following.receiver_handle,
                &data
            ).await?;
            for post in posts {
                let item: TimeLinePost = TimeLinePost{
                    post_id: post.post_id,
                    created_at: post.created_at,
                    created_by: post.created_by,
                    post_content: post.post_content,
                    author_instance: following.receiver_host.clone(),
                    author_display_name: info.display_name.clone()
                };
                result_posts.push(item);
            }
        }
    }
    result_posts.sort_by_key(
        |post| {
            DateTime::parse_from_rfc2822(&post.created_at)
                .map(|dt| dt.timestamp())
                .unwrap_or(0)
        }
    );
    let response: TimeLineResponse = TimeLineResponse {
        posts: result_posts
    };
    Ok(HttpResponse::Ok().json(response))
}

/// A service function to delete the
/// account of a Valkyrie user in a federated
/// manner. If the operation succeeds,
/// an HTTP response containing a success or
/// failure code in the form of a boolean
/// is returned. If the operation fails, 
/// an error is returned.
/// HTTP VERB: POST
#[post("/api/user/delete")]
pub async fn delete_account_service(
    payload: Json<LogoutPayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let instance: Instance = get_instance(&data)
        .await?;
    let user_following: Vec<Follow> = get_user_following(
        &user.username,
        &data
    ).await?;
    let user_followers: Vec<Follow> = get_user_followers(
        &user.username,
        &data
    ).await?;
    for following in user_following {
        let follow_rel: Follow = get_follow_by_participants(
            &instance.host,
            &user.username,
            &following.receiver_host,
            &following.receiver_handle,
            &data
        ).await?;
        if following.receiver_host != instance.host {
            let inbox_url: String = fetch_user_inbox_url(
                &following.receiver_host,
                &following.receiver_handle
            ).await?;
            send_message_to_inbox(
                &inbox_url,
                &InboxPayload{
                    action: Action::Delete,
                    entity: Entity::Subscription,
                    object: Object::Subscriber(
                        SubscriberObject {
                            id: follow_rel.id,
                            sender_host: follow_rel.sender_host,
                            sender_handle: follow_rel.sender_handle,
                            receiver_host: follow_rel.receiver_host,
                            receiver_handle: follow_rel.receiver_handle
                        }
                    )
                }
            ).await?;
        }
        delete_follow_by_participants(
            &instance.host,
            &user.username,
            &following.receiver_host,
            &following.receiver_handle,
            &data
        ).await?;
    }
    let user_posts: Vec<ValkyriePost> = get_user_posts(
        &user.username,
        &instance.host,
        &data
    ).await?;
    for post in user_posts {
        delete_post(
            &post.post_id,
            &post.created_by_host,
            &data
        ).await?;
        for follower in user_followers.clone() {
            if follower.sender_host != instance.host {
                let follower_inbox_url: String = fetch_user_inbox_url(
                    &follower.sender_host,
                    &follower.sender_handle
                ).await?;
                send_message_to_inbox(
                    &follower_inbox_url,
                    &InboxPayload{
                        action: Action::Delete,
                        entity: Entity::Post,
                        object: Object::Post(
                            PostObject {
                                post_id: post.post_id.clone(),
                                created_at: post.created_at.clone(),
                                created_by: post.created_by.clone(),
                                created_by_host: post.created_by_host.clone(),
                                post_content: post.post_content.clone()
                            }
                        )
                    }
                ).await?;
            }
        }
    }
    let user_tokens: Vec<APIToken> = get_user_tokens(
        &user.username,
        &data
    ).await?;
    for token in user_tokens {
        delete_api_token(
            &token.token,
            &data
        ).await?;
    }
    let d_o: bool = match delete_user(
        &user.username,
        &data
    ).await {
        Ok(_f) => true,
        Err(_e) => false
    };
    let response: StatusResponse = StatusResponse{
        is_ok: d_o
    };
    Ok(HttpResponse::Ok().json(response))
}
