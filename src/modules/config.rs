/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// to read environment
/// variables.
use std::env::var;

/// Importing the data structure
/// to catch and handle errors.
use super::err::ValkyrieErr;

/// A structure to encapsulate
/// data about configuration
/// values for the current
/// Valkyrie instance.
pub struct Config {
    pub use_ssl: bool,
    pub instance_ip: String,
    pub instance_port: i64,
    pub instance_host: String,
    pub instance_name: String,
    pub instance_admin: String,
    pub instance_description: String,
    pub instance_postgres_url: String,
    pub instance_admin_username: String,
    pub instance_admin_password: String,
    pub instance_admin_display_name: String,
    pub instance_admin_description: String, 
}

/// A function to read different environment
/// variables and gather configuration values
/// for setting up and running a Valkyrie instance.
/// If the operation succeeds, an instance of the
/// `Config` structure is returned. If the operation
/// fails, an error is returned.
pub fn read_config(
) -> Result<Config, ValkyrieErr>{
    let use_ssl: String = var("VALKYRIE_USE_SSL")?;
    let instance_ip: String = var("VALKYRIE_INSTANCE_IP")?;
    let instance_port: String = var("VALKYRIE_INSTANCE_PORT")?;
    let instance_host: String = var("VALYKRIE_INSTANCE_DOMAIN")?;
    let instance_name: String = var("VALKYRIE_INSTANCE_NAME")?;
    let instance_description: String = var("VALKYRIE_INSTANCE_DESCRIPTION")?;
    let instance_postgres_url: String = var("VALKYRIE_INSTANCE_POSTGRES_URL")?;
    let instance_admin_username: String = var("VALYKRIE_INSTANCE_ADMIN_USERNAME")?;
    let instance_admin_password: String = var("VALKYRIE_INSTANCE_ADMIN_PASSWORD")?;
    let instance_admin_display_name: String = var("VALKYRIE_INSTANCE_ADMIN_DISPLAY_NAME")?;
    let instance_admin_description: String = var("VALKYRIE_INSTANCE_ADMIN_DESCRIPTION")?;
    let mut use_https: bool = false;
    if use_ssl == "YES" {
        use_https = true;
    }
    let parsed: i64 = instance_port.parse::<i64>()?;
    let config: Config = Config {
        use_ssl: use_https,
        instance_ip: instance_ip,
        instance_port: parsed,
        instance_host: instance_host,
        instance_name: instance_name,
        instance_admin: instance_admin_username.clone(),
        instance_description: instance_description,
        instance_postgres_url: instance_postgres_url,
        instance_admin_username:  instance_admin_username,
        instance_admin_password: instance_admin_password,
        instance_admin_display_name: instance_admin_display_name,
        instance_admin_description: instance_admin_description
    };
    Ok(config)
}
