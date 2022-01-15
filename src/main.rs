//! Your Service Description here, etc.

use std::io;

#[macro_use] extern crate log;

pub mod web;
pub mod pages;

use jelly::{actix_web, Server};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();

    Server::new()
        .register_service(pages::configure)
        .register_service(web::accounts::configure)
        .register_jobs(web::accounts::jobs::configure)
        .register_service(web::dashboard::configure)
        .register_service(web::send::configure)
    .run().await?.await
}
