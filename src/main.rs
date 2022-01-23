//! Your Service Description here, etc.

use jelly::actix_web;
use mainlib;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    mainlib::main().await
}
