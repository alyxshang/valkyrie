/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the main
/// function to run a
/// Valkyrie instance.
use valkyrie::run_app;

/// A function to signal
/// to the operating system
/// which exit code a process
/// should exit with.
use std::process::exit;

/// The main point of
/// entry for the Rust
/// compiler.
#[actix_web::main]
async fn main(){
    match run_app().await {
        Ok(_f) => {},
        Err(e) => {
            eprintln!("Error running Valkyrie: {:?}", e);
            exit(1);
        }
    }
}
