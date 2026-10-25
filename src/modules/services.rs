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
/// to accept JSON payloads
/// in POST requests.
use actix_web::web::Json;

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
/// that models infoormation about
/// an API token for a user in the 
/// database.
use super::models::APIToken;

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
/// get a list of all the tokens
/// a Valkyrie user has created.
use super::db::get_user_tokens;

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

/// Importing the function to test
/// whether a supplied username
/// only contains valid characters
/// to test it.
use super::utils::is_valid_username;

/// Importing the data structure to encapsulate
/// data about a response containing
/// a user's API token. This data
/// structure is sent as a JSON
/// string.
use super::responses::TokenResponse;

/// Importng the function to update
/// the name of current Valkyrie 
/// instance's record in the database.
use super::db::update_instance_name;

/// Importng the function to update
/// the password of a Valkyrie user's
/// record in the database.
use super::db::update_user_password;

/// Importing the data structure to encapsulate
/// data about the success or failure of an 
/// operation. This data structure is sent 
/// as a JSON string.
use super::responses::StatusResponse;

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
/// on the current Valkyrie instance.
use super::responses::InstanceResponse;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for creating
/// a new post.
use super::payloads::PostCreatePayload;

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

/// Importng the function to update
/// the description of current Valkyrie 
/// instance's record in the database.
use super::db::update_instance_description;

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
/// Valkyrie user. record in the database. 
/// The user is not an administrator by 
/// default. If the operation succeeds,
/// an HTTP response containing information
/// on the created user is returned. If the
/// operation fails, an error is returned.
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

#[post("/api/post/create")]
pub async fn post_create_service(
    payload: Json<PostCreatePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let new_post: ValkyriePost = create_post(
        &user.username,
        &payload.content,
        &data
    ).await?;
    let response: PostCreatedResponse = PostCreatedResponse{
        post_id: new_post.post_id,
        created_at: new_post.created_at,
        created_by: new_post.created_by,
        post_content: new_post.post_content

    };
    Ok(HttpResponse::Ok().json(response))
}

#[post("/api/post/delete")]
pub async fn post_delete_service(
    payload: Json<PostDeletePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let post: ValkyriePost = get_post_by_id(
        &payload.post_id,
        &data
    ).await?;
    if post.created_by == user.username {
        let d_o: bool = match delete_post(
            &post.post_id,
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

#[post("/api/post/update")]
pub async fn post_update_service(
    payload: Json<PostUpdatePayload>,
    data: Data<Pool<Postgres>>
) -> Result<HttpResponse, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_token(
        &payload.token,
        &data
    ).await?;
    let post: ValkyriePost = get_post_by_id(
        &payload.post_id,
        &data
    ).await?;
    if post.created_by == user.username {
        let uo: bool = match update_post(
            &post.post_id,
            &payload.content,
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
            ValkyrieErr::new("Post ownership could not be verified.")
        )
    }
}

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
    for token in tokens { res_strings.push(token.token.clone()) }
    let response: UserTokensResponse = UserTokensResponse{
        tokens: res_strings
    };
    Ok(HttpResponse::Ok().json(response))
}
