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

/// Importing the structure
/// to create a new Actix Web
/// app.
use actix_web::App;

/// Importing the structure
/// for environment logging.
use env_logger::Env;

/// Importing the structure
/// to allow CORS.
use actix_cors::Cors;

/// Importing the structure
/// to add data to an Actix
/// Web app.
use actix_web::web::Data;

/// Importing the structure
/// to create a new Actix Web
/// app.
use actix_web::HttpServer;

/// Importing the structure
/// to encapsulate data about
/// configuration values.
use super::config::Config;

/// Importing the function to 
/// create a record for a user 
/// to test it.
use super::db::create_user;

/// Importing the function to
/// check whether a user exists
/// to test it.
use super::db::user_exists;

/// Importing the data structure
/// that models information about
/// the current Valkyrie instance
/// in the database.
use super::models::Instance;

/// Importing the data structure
/// to catch and handle errors.
use super::err::ValkyrieErr;

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
/// read configuration values
/// for a Valkyrie instance to
/// test it.
use super::config::read_config;

/// Importing the function to create
/// a new instance to test it.
use super::db::create_instance;

/// Importing the data structure
/// that models information about
/// a user in the database.
use super::models::ValkyrieUser;

/// Importing Actix Web's default
/// logger.
use actix_web::middleware::Logger;

/// Importing the function to create
/// a connection to the database to
/// test it.
use super::utils::create_connection;

/// Importing the service function to
/// get posts for a user's timeline.
use super::services::timeline_service;

/// Importing the service function to
/// allow users to retrieve informtion
/// about each other.
use super::protocol::user_info_service;

/// Importing the service function to
/// login a user.
use super::services::login_user_service;

/// Importing the service function to
/// create a new user.
use super::services::create_user_service;

/// Importing the service function to
/// logout a user.
use super::services::logout_user_service;

/// Importing the service function to create
/// a new post.
use super::services::post_create_service;

/// Importing the service function to
/// allow users to send federated messages
/// to each other.
use super::protocol::user_inbox_service;

/// Importing the service function to update
/// a post.
use super::services::post_update_service;

/// Importing the service function to delete
/// a post.
use super::services::post_delete_service;

/// Importing the service function to
/// follow a user.
use super::services::follow_user_service;

/// Importing the service function to get
/// infor on the current Valkyrie instance.
use super::services::instance_info_service;

/// Importing the service function to
/// unfollow a user.
use super::services::unfollow_user_service;

/// Importing the service function to list
/// all active API tokens a user has created.
use super::services::list_user_tokens_service;

/// Importing the service function to edit
/// the instance's name.
use super::services::edit_instance_name_service;

/// Importing the service function for changing
/// a password.
use super::services::edit_user_password_service;

/// Importing the service function for changing
/// a description of a user.
use super::services::edit_user_description_service;

/// Importing the service function for changing
/// a display name.
use super::services::edit_user_display_name_service;

/// Importing the service function to edit
/// the instance's description.
use super::services::edit_instance_description_service;

/// The function to run a Valkyrie
/// instance. If the operation is
/// successful, nothing is returned.
/// If the operation fails, an error
/// is returned.
pub async fn run_app(
) -> Result<(), ValkyrieErr>{
    let config: Config = read_config()?;
    let conn: Pool<Postgres> = create_connection(
        &config.instance_postgres_url
    ).await?;
    let admin_exists: bool = user_exists(
        &config.instance_admin_username,
        &conn
    ).await;
    if !admin_exists {
        let _admin: ValkyrieUser = create_user(
            &config.instance_admin_username,
            &config.instance_admin_password,
            &config.instance_admin_display_name,
            &config.instance_admin_description,
            true,
            &conn
        ).await?;
    }
    if get_instance(&conn).await.is_err() {
        let _instance: Instance = create_instance(
            &config.instance_host,
            &config.instance_name,
            &config.instance_admin_username,
            &config.use_ssl,
            &config.instance_description,
            &conn
        ).await?;
    } 
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(
        move || {
            let cors = Cors::default()
                .allow_any_origin()
                .send_wildcard()
                .allow_any_method()
                .allow_any_header();
            App::new()
                .wrap(cors)
                .app_data(Data::new(conn.clone()))
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(timeline_service)
                .service(user_info_service)
                .service(user_inbox_service)
                .service(login_user_service)
                .service(create_user_service)
                .service(logout_user_service)
                .service(post_create_service)
                .service(post_update_service)
                .service(post_delete_service)
                .service(follow_user_service)
                .service(instance_info_service)
                .service(instance_info_service)
                .service(unfollow_user_service)
                .service(list_user_tokens_service)
                .service(edit_instance_name_service)
                .service(edit_user_password_service)
                .service(edit_user_description_service)
                .service(edit_user_display_name_service)
                .service(edit_instance_description_service)
        }
    ).bind(format!("{}:{}", config.instance_ip, config.instance_port))?
     .run()
     .await
     .map_err(|e| ValkyrieErr::new(&e.to_string()))?;
    Ok(())
}
