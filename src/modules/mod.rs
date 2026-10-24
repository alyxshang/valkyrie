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
