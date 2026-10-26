/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// containing the functions
/// to read data from the
/// database and write data
/// to the database.
pub mod db;

/// Exporting the module
/// containing the structure
/// to catch and handle errors.
pub mod err;

/// Declaring this crate's
/// unit tests file.
#[cfg(test)]
pub mod tests;

/// Exporting the module
/// containing utility
/// functions.
pub mod utils;

/// Exporting the module
/// containing Valkyrie's
/// database models.
pub mod models;

/// Exporting the module
/// containing the function
/// and structure to gather
/// configuration values
/// for a Valkyrie instance.
pub mod config;

/// Exporting the module
/// containing the service
/// functions for the REST
/// API.
pub mod services;

/// Exporting the module
/// containing data structures
/// modelling data for 
/// accepting JSON payloads
/// from REST API requests.
pub mod payloads;

/// Exporting the module
/// containing the service
/// functions for the REST
/// API for the Valkyrie
/// protocol.
pub mod protocol;

/// Exporting the module
/// containing data structures
/// modelling data to be sent
/// to the client as JSON strings
/// by serialized into JSON.
pub mod responses;
