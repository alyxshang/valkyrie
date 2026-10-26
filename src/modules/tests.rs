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
/// namespace for
/// Acix Web to
/// run automated
/// unit tests.
use actix_web;

/// Importing the function
/// to read environment 
/// variables.
use std::env::var;

/// Importing the function
/// to get the current time
/// as RFC 2822 as a string.
use super::utils::rfc2822;

/// Importing the function to 
/// create a record for a user 
/// to test it.
use super::db::create_user;

/// Importing the function to
/// check whether a user exists
/// to test it.
use super::db::user_exists;

/// Importing the function
/// to delete a record for
/// a user to test it.
use super::db::delete_user;

/// Importing the function
/// to create a new record
/// for a user's post in the
/// database to test it.
use super::db::create_post;

/// Importing the function
/// to delete a record
/// for a user's post in the
/// database to test it.
use super::db::delete_post;

/// Importing the function
/// to update a record
/// for a user's post in the
/// database to test it.
use super::db::update_post;

/// Importing the function to
/// retrieve the current instance
/// to test it.
use super::db::get_instance;

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
/// retrieve all the posts a
/// user has created to test it.
use super::db::get_user_posts;

/// Importing the function to
/// fetch a post's record from
/// the database to test it.
use super::db::get_post_by_id;


/// Importing the function to create
/// a new instance to test it.
use super::db::create_instance;

/// Importing the function to 
/// retrieve all the API tokens a
/// user has created to test it.
use super::db::get_user_tokens;

/// Importing the function to
/// read configuration values
/// for a Valkyrie instance to
/// test it.
use super::config::read_config;

/// Importing the function to 
/// fetch the record for a user
/// to test it.
use super::db::get_user_by_name;

/// Importing the function to
/// create a new record for an
/// API token in the database to
/// test it.
use super::db::create_api_token;

/// Importing the function to
/// delete a record for an
/// API token in the database to
/// test it.
use super::db::delete_api_token;

/// Importing the function to
/// retrieve the record for a user
/// given one of their API tokens
/// to test it.
use super::db::get_user_by_token;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for logging in
/// a Valkyrie user.
use super::payloads::LoginPayload;

/// Importing the function to retrieve
/// the record for an API token, given
/// the token's string to test it.
use super::db::get_token_by_token;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for logging out
/// a Valkyrie user.
use super::payloads::LogoutPayload;

/// Importing the function to retrieve 
/// records for all the users a user 
/// is followed by.
use super::db::get_user_followers;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for making trivial
/// changes to a Valkyrie user's account.
use super::payloads::ChangePayload;

/// Importing the function to retrieve 
/// records for all the users a user 
/// is following.
use super::db::get_user_following;

/// Importing the function to create
/// a connection to the database to
/// test it.
use super::utils::create_connection;

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

/// Importing the function to update
/// a user's password to test it.
use super::db::update_user_password;

/// Importing the data structure to encapsulate
/// data about the success or failure of an 
/// operation. This data structure is sent 
/// as a JSON string.
use super::responses::StatusResponse;

/// Importing the function to update the
/// instance's name to test it.
use super::db::update_instance_name;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on the current Valkyrie instance.
use super::responses::InstanceResponse;

/// Importing the function to update
/// a user's description to test it.
use super::db::update_user_description;

/// Importing the service function for obtaining
/// federated info on a user to test it.
use super::protocol::user_info_service;

/// Importing the data structure 
/// to encapsulate data for the 
/// Valkyrie protocol about a user 
/// and their posts. This data 
/// structure is sent as  a JSON 
/// string.
use super::responses::UserInfoResponse;

/// Importing the data structure
/// that models data to be deserialized
/// from a JSON payload for creating
/// a Valkyrie user.
use super::payloads::CreateUserPayload;

/// Importing the function to update
/// a user's display name to test it.
use super::db::update_user_display_name;

/// Importing the service function to
/// login a user to test it.
use super::services::login_user_service;

/// Importing the service function to
/// create a new user to test it.
use super::services::create_user_service;

/// Importing the service function to
/// logout a user to test it.
use super::services::logout_user_service;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on all of a Valkyrie user's API tokens.
use super::responses::UserTokensResponse;

/// Importing the data structure that
/// models data to be deserialized from
/// a JSON string payload for making 
/// non-trivial changes to a Valkyrie 
/// user's account securely.
use super::payloads::SecureChangePayload;

/// Importing the data structure
/// that models data to be serialized
/// from a Rust data structure into 
/// a JSON payload containing information
/// on a newly-created Valkyrie user.
use super::responses::CreateUserResponse;

/// Importing the function to retrieve the
/// record for a following-followee relation-
/// ship from the database given the
/// participants.
use super::db::get_follow_by_participants;

/// Importing the function to update the
/// instance's description to test it.
use super::db::update_instance_description;

/// Importing the service function to get
/// infor on the current Valkyrie instance.
use super::services::instance_info_service;

/// Importing the function to delete a
/// record for a following-followee relation-
/// ship from the database given the
/// participants.
use super::db::delete_follow_by_participants;

/// Importing the service function to list
/// all active API tokens a user has created
/// to test it.
use super::services::list_user_tokens_service;

/// Importing the service function to edit
/// the instance's name.
use super::services::edit_instance_name_service;

/// Importing the service function for changing
/// a password to test it.
use super::services::edit_user_password_service;

/// Importing the service function for changing
/// a description of a user to test it.
use super::services::edit_user_description_service;

/// Importing the service function for changing
/// a display name to test it.
use super::services::edit_user_display_name_service;

/// Importing the service function to edit
/// the instance's description.
use super::services::edit_instance_description_service;

/// A function to test the functions
/// in the module containing utility
/// functions.
#[tokio::test]
async fn test_utils_functions(){
    let url: String = var("VALKYRIE_INSTANCE_POSTGRES_URL")
        .expect("Could not get DB url.");
    let connection = create_connection(&url).await;
    let now: String = rfc2822();
    let invalid_username: bool = is_valid_username("AlYx");
    let valid_username: bool = is_valid_username("alyxshang");
    assert_eq!(now.is_empty(), false);
    assert_eq!(connection.is_ok(), true);
    assert_eq!(invalid_username, false);
    assert_eq!(valid_username, true);
}

/// A function to test the functions in
/// the module to read and write data
/// from and to the database.
#[tokio::test]
async fn test_db_functions() {
    let url: String = var("VALKYRIE_INSTANCE_POSTGRES_URL")
        .expect("Could not get DB url.");
    let connection: Pool<Postgres> = create_connection(&url)
        .await.expect("Could not connect to database.");
    let instance = create_instance(
        "example.com", 
        "Example", 
        "alyxshang",
        &false,
        "An example instance",
        &connection
    ).await.expect("Could not create instance.");
    let _updated_name = update_instance_name(
        "New Example", 
        &connection
    ).await.expect("Could not update instance name.");
    let _updated_description = update_instance_description(
        "A new example instance", 
        &connection
    ).await.expect("Could not update instance name.");
    let fetched = get_instance(&connection)
        .await
        .expect("Could not fetch instance.");
    let user = create_user(
        "alyxshang",
        "12345678",
        "Alyx Shang",
        "A crabby floofer",
        false,
        &connection
    ).await.expect("User could not be created.");
    let _test_user = create_user(
        "alexshang",
        "12345678",
        "Alyx Shang",
        "A crabby floofer",
        false,
        &connection
    ).await.expect("User could not be created.");
    let _uud = update_user_description(
        "alyxshang",
        "A ziggy floofer",
        &connection
    ).await.expect("Could not update user description.");
    let _uudd = update_user_display_name(
        "alyxshang",
        "ALYX SHANG",
        &connection
    ).await.expect("Could not update user display name.");
    let up = update_user_password(
        "alyxshang",
        "12345678",
        "123456780",
        &connection
    ).await;
    let fetched_user = get_user_by_name(
        "alyxshang",
        &connection
    ).await.expect("Could not fetch user record.");
    let token = create_api_token(
        "alyxshang",
        "123456780",
        &connection
    ).await.expect("Could not create API token.");
    let fetched_token = get_token_by_token(
        &token.token,
        &connection
    ).await.expect("Could not fetch created API token.");
    let post = create_post(
        &None,
        &None,
        "alyxshang",
        &None,
        "Hello World",
        &connection
    ).await.expect("Could not create post.");
    let _upp = update_post(
        &post.post_id,
        &instance.host,
        "HELLO WORLD",
        &connection
    ).await.expect("Could not update post.");
    let fetched_post = get_post_by_id(
        &post.post_id,
        &instance.host,
        &connection
    ).await.expect("Could not fetch post.");
    let fetched_token_user = get_user_by_token(
        &fetched_token.token,
        &connection
    ).await.expect("Could not get user by token.");
    let user_posts = get_user_posts("alyxshang", &instance.host, &connection)
        .await.expect("Could not fetch user posts.");
    let user_tokens = get_user_tokens("alyxshang", &connection)
        .await.expect("Could not fetch user tokens.");
    let user_e: bool = user_exists("alyxshang", &connection).await;
    let follow_rel_one = create_follow(
        &None,
        &instance.host,
        "alyxshang",
        &instance.host,
        "alexshang",
        &connection
    ).await.expect("Alyx could not follow Alex.");
    let follow_rel_two = create_follow(
        &None,
        &instance.host,
        "alexshang",
        &instance.host,
        "alyxshang",
        &connection
    ).await.expect("Alyx could not follow Alex.");
    let fetched_rel_one = get_follow_by_participants(
        &instance.host,
        "alyxshang",
        &instance.host,
        "alexshang",
        &connection
    ).await.expect("Could not fetch first follow rel.");
    let fetched_rel_two = get_follow_by_participants(
        &instance.host,
        "alexshang",
        &instance.host,
        "alyxshang",
        &connection
    ).await.expect("Could not fetch first follow rel.");
    let alyx_following_count = get_user_following(
        "alyxshang",
        &connection
    ).await.expect("Could not get Alyx's following count.");
    let alyx_follower_count = get_user_followers(
        "alyxshang",
        &connection
    ).await.expect("Could not get Alex's following count.");
    assert_eq!(follow_rel_one.id, fetched_rel_one.id);
    assert_eq!(follow_rel_two.id, fetched_rel_two.id);
    assert_eq!(alyx_following_count.len(), 1);
    assert_eq!(alyx_follower_count.len(), 1);
    assert_eq!(fetched.name, "New Example");
    assert_eq!(fetched.description, "A new example instance");
    assert_eq!(fetched.host, instance.host);
    assert_eq!(user.username, fetched_user.username);
    assert_eq!(fetched_user.display_name, "ALYX SHANG");
    assert_eq!(fetched_user.description, "A ziggy floofer");
    assert_eq!(up.is_ok(), true);
    assert_eq!(token.token, fetched_token.token);
    assert_eq!(post.created_by, "alyxshang");
    assert_eq!(fetched_post.post_content, "HELLO WORLD");
    assert_eq!(fetched_token_user.username, "alyxshang");
    assert_eq!(user_posts.len(), 1);
    assert_eq!(user_tokens.len(), 1);
    assert_eq!(user_e, true);
    let del_post_op = delete_post(
        &fetched_post.post_id,
        &instance.host,
        &connection
    ).await;
    let del_token = delete_api_token(
        &fetched_token.token,
        &connection
    ).await;
    let del_user = delete_user(
        "alyxshang",
        &connection
    ).await;
    let del_f_rel_o = delete_follow_by_participants(
        &instance.host,
        "alyxshang",
        &instance.host,
        "alexshang",
        &connection
    ).await;
    let del_f_rel_t = delete_follow_by_participants(
        &instance.host,
        "alexshang",
        &instance.host,
        "alyxshang",
        &connection
    ).await;
    assert_eq!(del_post_op.is_ok(), true);
    assert_eq!(del_token.is_ok(), true);
    assert_eq!(del_user.is_ok(), true);
    assert_eq!(del_f_rel_o.is_ok(), del_f_rel_t.is_ok());
}

/// A function to test the functions
/// in the module containing functions 
/// to read configuration values.
#[test]
pub fn test_config(){
    let values = read_config()
        .expect("Could not read configuration values.");
    assert_eq!(values.instance_admin, "alyxshang");
}

/// A function to test the service function
/// for retrieving information on a Valkyrie
/// instance.
#[actix_web::test]
pub async fn test_instance_info_service() {
    let url: String = var("VALKYRIE_INSTANCE_POSTGRES_URL")
        .expect("Could not get DB url.");
    let connection: Pool<Postgres> = create_connection(&url)
        .await.expect("Could not connect to database.");
    let app = actix_web::test::init_service(
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(connection))
            .service(instance_info_service),
    ).await;
    let req = actix_web::test::TestRequest::get().uri("/api/instance").to_request();
    let resp: InstanceResponse= actix_web::test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.name, "Example");
}

/// A function to test the service functions
/// for managing users on a Valkyrie instance.
#[actix_web::test]
pub async fn test_user_management_services() {
    let url: String = var("VALKYRIE_INSTANCE_POSTGRES_URL")
        .expect("Could not get DB url.");
    let connection: Pool<Postgres> = create_connection(&url)
        .await.expect("Could not connect to database.");
    let app = actix_web::test::init_service(
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(connection))
            .service(create_user_service)
            .service(login_user_service)
            .service(logout_user_service)
    ).await;
    let user_create_payload: CreateUserPayload = CreateUserPayload{
        username: "bear".to_string(),
        password: "123456789".to_string(),
        display_name: "Silly Floofer".to_string(),
        description: "Monarch of all floofers".to_string()
    };
    let login_payload: LoginPayload = LoginPayload{
        username: "bear".to_string(),
        password: "123456789".to_string(),
    };
    let user_create_req = actix_web::test::TestRequest::post()
        .uri("/api/user/create")
        .set_json(&user_create_payload)
        .to_request();
    let user_create_resp: CreateUserResponse = actix_web::test::call_and_read_body_json(
        &app, 
        user_create_req
    ).await;
    let login_req = actix_web::test::TestRequest::post()
        .uri("/api/user/login")
        .set_json(&login_payload)
        .to_request();
    let login_resp: TokenResponse = actix_web::test::call_and_read_body_json(
        &app, 
        login_req
    ).await;
    let logout_payload: LogoutPayload = LogoutPayload{
        token: login_resp.token.clone()
    };
    let logout_req = actix_web::test::TestRequest::post()
        .uri("/api/user/logout")
        .set_json(&logout_payload)
        .to_request();
    let logout_resp: StatusResponse = actix_web::test::call_and_read_body_json(
        &app, 
        logout_req
    ).await;
    assert_eq!(user_create_resp.username, "bear");
    assert_eq!(login_resp.token.is_empty(), false);
    assert_eq!(logout_resp.is_ok, true);
}

/// A function to test the service functions
/// for editing the instance.
#[actix_web::test]
pub async fn test_instance_edit_services() {
    let url: String = var("VALKYRIE_INSTANCE_POSTGRES_URL")
        .expect("Could not get DB url.");
    let connection: Pool<Postgres> = create_connection(&url)
        .await.expect("Could not connect to database.");
    let app = actix_web::test::init_service(
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(connection.clone()))
            .service(edit_instance_name_service)
            .service(edit_instance_description_service)
    ).await;
    let _ = create_user(
        "megahugebear",
        "123456789",
        "Silly Floofer",
        "Monarch of all floofers",
        true,
        &connection
    ).await;
    let token = create_api_token(
        "megahugebear",
        "123456789",
        &connection
    ).await.expect("Could not create token for \"megahugebear\".");
    let edit_name_payload: ChangePayload = ChangePayload {
        token: token.token.clone(),
        new_value: "My Example".to_string()
    };
    let edit_description_payload: ChangePayload = ChangePayload {
        token: token.token.clone(),
        new_value: "My New Example Instance".to_string()
    };
    let edit_name_req = actix_web::test::TestRequest::post()
        .uri("/api/instance/edit/name")
        .set_json(&edit_name_payload)
        .to_request();
    let edit_name_resp: StatusResponse = actix_web::test::call_and_read_body_json(
        &app, 
        edit_name_req
    ).await; 
    let edit_description_req = actix_web::test::TestRequest::post()
        .uri("/api/instance/edit/description")
        .set_json(&edit_description_payload)
        .to_request();
    let edit_description_resp: StatusResponse = actix_web::test::call_and_read_body_json(
        &app, 
        edit_description_req
    ).await;
    assert_eq!(edit_name_resp.is_ok, true);
    assert_eq!(edit_description_resp.is_ok, true);
    
}

/// A function to test the service functions
/// for editing a user.
#[actix_web::test]
pub async fn test_user_edit_services() {
    let url: String = var("VALKYRIE_INSTANCE_POSTGRES_URL")
        .expect("Could not get DB url.");
    let connection: Pool<Postgres> = create_connection(&url)
        .await.expect("Could not connect to database.");
    let app = actix_web::test::init_service(
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(connection.clone()))
            .service(edit_user_display_name_service)
            .service(edit_user_description_service)
            .service(edit_user_password_service)
            .service(list_user_tokens_service)
    ).await;
    let _ = create_user(
        "superhugebear",
        "123456789",
        "Silly Floofer",
        "Monarch of all floofers",
        true,
        &connection
    ).await;
    let token = create_api_token(
        "superhugebear",
        "123456789",
        &connection
    ).await.expect("Could not create token for \"superhugebear\".");
    let list_payload: LogoutPayload = LogoutPayload {
        token: token.token.clone()
    };
    let edit_name_payload: ChangePayload = ChangePayload {
        token: token.token.clone(),
        new_value: "Monarch Floofer".to_string()
    };
    let edit_description_payload: ChangePayload = ChangePayload {
        token: token.token.clone(),
        new_value: "Ancient Evil of Floofers".to_string()
    };
    let edit_password_payload: SecureChangePayload = SecureChangePayload {
        token: token.token.clone(),
        new_value: "1234567890".to_string(),
        password: "123456789".to_string()
    };
    let edit_name_req = actix_web::test::TestRequest::post()
        .uri("/api/user/edit/name")
        .set_json(&edit_name_payload)
        .to_request();
    let edit_name_resp: StatusResponse = actix_web::test::call_and_read_body_json(
        &app, 
        edit_name_req
    ).await; 
    let edit_description_req = actix_web::test::TestRequest::post()
        .uri("/api/user/edit/description")
        .set_json(&edit_description_payload)
        .to_request();
    let edit_description_resp: StatusResponse = actix_web::test::call_and_read_body_json(
        &app, 
        edit_description_req
    ).await;
    let edit_password_req = actix_web::test::TestRequest::post()
        .uri("/api/user/edit/password")
        .set_json(&edit_password_payload)
        .to_request();
    let edit_password_resp: StatusResponse = actix_web::test::call_and_read_body_json(
        &app, 
        edit_password_req
    ).await;
    let list_tokens_req = actix_web::test::TestRequest::post()
        .uri("/api/user/tokens/list")
        .set_json(&list_payload)
        .to_request();
    let list_tokens_resp: UserTokensResponse = actix_web::test::call_and_read_body_json(
        &app, 
        list_tokens_req
    ).await;
    assert_eq!(edit_name_resp.is_ok, true);
    assert_eq!(edit_password_resp.is_ok, true);
    assert_eq!(edit_description_resp.is_ok, true);
    assert_eq!(list_tokens_resp.tokens.is_empty(), false);
    
}

/// A function to test one of the the Valkyrie
/// protocol service functions.
#[actix_web::test]
pub async fn test_protocol_services() {
    let url: String = var("VALKYRIE_INSTANCE_POSTGRES_URL")
        .expect("Could not get DB url.");
    let connection: Pool<Postgres> = create_connection(&url)
        .await.expect("Could not connect to database.");
    let app = actix_web::test::init_service(
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(connection.clone()))
            .service(user_info_service)
    ).await;
    let _ = create_user(
        "hugebear",
        "123456789",
        "Silly Floofer",
        "Monarch of all floofers",
        false,
        &connection
    ).await;
    let info_req = actix_web::test::TestRequest::get()
        .uri("/protocol/hugebear")
        .to_request();
    let info_resp: UserInfoResponse = actix_web::test::call_and_read_body_json(
        &app, 
        info_req
    ).await;
    assert_eq!(info_resp.username, "hugebear");
}
