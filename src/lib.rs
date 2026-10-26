/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

#![deny(clippy::all)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::box_collection)]
#![allow(clippy::redundant_field_names)]

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing the functions
/// to read data from the
/// database and write data
/// to the database.
pub use modules::db::*;

/// Re-exporting the module
/// containing the structure
/// to catch and handle errors.
pub use modules::err::*;

/// Re-exporting the module
/// containing utility
/// functions.
pub use modules::utils::*;

/// Re-exporting the module
/// containing Valkyrie's
/// database models.
pub use modules::models::*;

/// Re-exporting the module
/// containing the service
/// functions for the REST
/// API.
pub use modules::services::*;

/// Re-exporting the module
/// containing data structures
/// modelling data for 
/// accepting JSON payloads
/// from REST API requests.
pub use modules::payloads::*;

/// Re-exporting the module
/// containing the service
/// functions for the REST
/// API for the Valkyrie
/// protocol.
pub use modules::protocol::*;

/// Re-exporting the module
/// containing data structures
/// modelling data to be sent
/// to the client as JSON strings
/// by serialized into JSON.
pub use modules::responses::*;
