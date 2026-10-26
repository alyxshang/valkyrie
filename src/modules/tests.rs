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

/// Importing the function to retrieve
/// the record for an API token, given
/// the token's string to test it.
use super::db::get_token_by_token;

/// Importing the function to create
/// a connection to the database to
/// test it.
use super::utils::create_connection;

/// Importing the function to test
/// whether a supplied username
/// only contains valid characters
/// to test it.
use super::utils::is_valid_username;

/// Importing the function to update
/// a user's password to test it.
use super::db::update_user_password;

/// Importing the function to update the
/// instance's name to test it.
use super::db::update_instance_name;

/// Importing the function to update
/// a user's description to test it.
use super::db::update_user_description;

/// Importing the function to update
/// a user's display name to test it.
use super::db::update_user_display_name;

/// Importing the function to update the
/// instance's description to test it.
use super::db::update_instance_description;

/// A function to test the functions
/// in the module containing utility
/// functions.
#[tokio::test]
async fn test_utils_functions(){
    let url: String = var("INSTANCE_DB_URL")
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
    let url: String = var("INSTANCE_DB_URL")
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
    assert_eq!(del_post_op.is_ok(), true);
    assert_eq!(del_token.is_ok(), true);
    assert_eq!(del_user.is_ok(), true);
}
