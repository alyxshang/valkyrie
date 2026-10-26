/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "sqlx"
/// namespace to implement
/// the `Error` trait from it
/// for the `ValkyrieErr` 
/// structure.
use sqlx;

/// Importing the
/// namespace for
/// Reqwest to 
/// implement the
/// `Error`
/// trait.
use reqwest;

/// Importing the
/// namespace for
/// Actix Web to 
/// implement the
/// `ResponseError`
/// trait.
use actix_web;

/// Importing the macro
/// for serializing a 
/// Rust data structure
/// into a JSON string.
use serde::Serialize;

/// Importing the "Result"
/// type because it is needed
/// by the "Display" trait.
use std::fmt::Result;

/// Importing the "Display"
/// trait to implement it.
use std::fmt::Display;

/// Importing the "Error"
/// trait to implement it.
use std::error::Error;

/// Importing the "Formatter"
/// structure because it is needed
/// by the "Display" trait.
use std::fmt::Formatter;

/// Importing the error
/// structure from the
/// "bcrypt" crate to
/// implement it for the
/// `ValkyrieErr` structure.
use bcrypt::BcryptError;

/// Importing the structure
/// to return HTTP responses
/// from a service function
/// for the REST API.
use actix_web::HttpResponse;

/// A structure to catch and handle
/// errors for Valkyrie.
#[derive(Clone,Eq,PartialEq,Debug,Serialize)]
pub struct ValkyrieErr {
    pub details: String
}

/// Implementing functions
/// for the `ValkyrieErr` structure.
impl ValkyrieErr {

    /// A function to create
    /// a new instance of the
    /// `ValkyrieErr` structure and
    /// return it.
    pub fn new(
        details: &str
    ) -> ValkyrieErr {
        ValkyrieErr {
            details: details.to_owned()
        }
    }

}

/// Implementing the
/// `Error` trait for
/// the `ValkyrieErr` structure.
impl Error for ValkyrieErr {

    /// The function that
    /// implements the
    /// `Error` trait for
    /// the `ValkyrieErr` structure.
    fn description(
        &self
    ) -> &str {
        &self.details
    }
}

/// Implementing the
/// `Display` trait for
/// the `ValkyrieErr` structure.
impl Display for ValkyrieErr {

    /// The function that
    /// implements the
    /// `Display` trait for
    /// the `ValkyrieErr` structure.
    fn fmt(
        &self, 
        f: &mut Formatter
    ) -> Result {
        write!(f,"{}",self.details)
    }
}

/// Implementing the `From`
/// trait to "convert" the 
/// `sqlx::Error` structure 
/// to the `ValkyrieErr`
/// structure.
impl From<sqlx::Error> for ValkyrieErr{
    fn from(error: sqlx::Error) -> Self {
        ValkyrieErr::new(&format!("SQLx error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the 
/// `BcryptError` structure 
/// to the `ValkyrieErr`
/// structure.
impl From<BcryptError> for ValkyrieErr{
    fn from(error: BcryptError) -> Self {
        ValkyrieErr::new(&format!("BCrypt error: {:?}", error.to_string()))
    }
}

/// Implementing the `ResponseError`
/// trait to "convert" the 
/// `ResponseError` trait
/// to the `ValkyrieErr`
/// structure.
impl actix_web::error::ResponseError for ValkyrieErr{
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(self)
    }
}

/// Implementing the `reqwest::Error`
/// trait to "convert" the 
/// `reqwest::Error` trait
/// to the `ValkyrieErr`
/// structure.
impl From<reqwest::Error> for ValkyrieErr{
    fn from(error: reqwest::Error) -> Self {
        ValkyrieErr::new(&format!("Reqwest error: {:?}", error.to_string()))
    }
}

/// Implementing the 
/// `std::num::ParseIntError`
/// trait to "convert" the 
/// `ParseIntError` structure
/// to the `ValkyrieErr`
/// structure.
impl From<std::num::ParseIntError> for ValkyrieErr{
    fn from(error: std::num::ParseIntError) -> Self {
        ValkyrieErr::new(&format!("Integer-parsing error: {:?}", error.to_string()))
    }
}

/// Implementing the 
/// `std::env::VarError`
/// trait to "convert" the 
/// `VarError` structure
/// to the `ValkyrieErr`
/// structure.
impl From<std::env::VarError> for ValkyrieErr{
    fn from(error: std::env::VarError) -> Self {
        ValkyrieErr::new(&format!("Environment error: {:?}", error.to_string()))
    }
}
