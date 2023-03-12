use crate::utils::template::Template;

pub struct JCTemplate {}

impl Template for JCTemplate {
    fn crates(&self) -> Vec<(&'static str, &'static str)> {
        vec![("thiserror", "")]
    }

    fn files(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "main.rs",
                "
// #![allow(unused)] // For beginning only.
        
use crate::prelude::*;
        
mod error;
mod prelude;
mod utils;
        
fn main() -> Result<()> {
    println!(\"Hello, world!\");
        
    Ok(())
}",
            ),
            (
                "prelude.rs",
                "
//! Crate prelude

// Re-export the crate Error.
pub use crate::error::Error;
                
// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;
                
// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);
",
            ),
            (
                "error.rs",
                "
//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// For starter, to remove as code matures.
    #[error(\"Generic error: {0}\")]
    Generic(String),
    /// For starter, to remove as code matures.
    #[error(\"Static error: {0}\")]
    Static(&'static str),
            
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
            ",
            ),
            ("utils/mod.rs", ""),
        ]
    }
}

pub struct ActixTemplate {}

impl Template for ActixTemplate {
    fn crates(&self) -> Vec<(&'static str, &'static str)> {
        vec![("actix-web", "")]
    }

    fn files(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "main.rs",
                "
use actix_web::{HttpServer, App};
use crate::api::hello::hello;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind((\"127.0.0.1\", 8080))?
    .run()
    .await
}",
            ),
            ("api/mod.rs", "ppub mod hello;"),
            (
                "api/hello.rs",
                "
use actix_web::{get, HttpResponse, Responder}

#[get(\"/\")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(\"Hello world!\")
}",
            ),
        ]
    }
}
