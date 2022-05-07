use jelly::prelude::*;
use jelly::Result;
use log::{debug, error, log_enabled, info, Level};

use jelly::actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use std::future::Future;
// use futures_core::stream::Stream;
use futures_util::StreamExt;

use std::io::Write;

use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use uuid::Uuid;

// use csv;

use std::str;


/// Returns an overview of everything in the system.
pub async fn upload(request: HttpRequest) -> Result<HttpResponse> {
    // let user = request.user()?;

    request.render(200, "upload/index.html", {
        let mut context = Context::new();

        // let id = user.id;
        // let _name = user.name;
        // let _is_admin = user.is_admin;
        // let _is_anonymous = user.is_anonymous;

        context.insert("is_logged_in", &true);
        // context.insert("form", &LoginForm::default());
        // context.insert("user_id_string", &format!("SEND.rs User ID: {:?}", id));
        context
    })
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyObj {
    name: String,
    number: String,
}

pub async fn upload_file(request: HttpRequest, mut body: web::Payload) -> Result<HttpResponse> {
    info!(target: "upload request", "{:#?}", request);
    // info!(target: "upload mp", "{:#?}", mp);
    // info!(target: "upload state", "{:#?}", state);
    // info!(target: "upload request", "{:#?}", body);

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    info!(target: "bytes request", "{:#?}", bytes);


    let data = bytes.to_vec();

    info!(target: "data", "{:#?}", data);

    let headers = request.headers();

    info!(target: "headers", "{:#?}", headers);


    // let test = bytes.len();

    // let str = str::from_utf8(&bytes);

    // info!(target: "bytes request", "{:#?}", str);

    // let mut rdr = csv::Reader::from_reader(bytes);

    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     info!(target: "bytes request", "{:#?}", record);
    // }



    request.render(200, "upload/file_uploaded_success.html", {
        let mut context = Context::new();

        // let id = user.id;
        // let _name = user.name;
        // let _is_admin = user.is_admin;
        // let _is_anonymous = user.is_anonymous;

        context.insert("upload_success", &true);
        // context.insert("user_id_string", &format!("SEND.rs User ID: {:?}", id));
        context
    })
}
