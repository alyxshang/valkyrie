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

/// Exporting the module
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

/// Exporting the module
/// containing Valkyrie's
/// database models.
pub use modules::models::*;
