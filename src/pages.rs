use jelly::actix_web::web::{resource, ServiceConfig};
use jelly::prelude::*;
use jelly::Result;

pub async fn homepage(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "index.html", {
        let context = Context::new();
        context
    })
}

pub async fn node(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "node.html", {
        let context = Context::new();
        context
    })
}

pub async fn help(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "help.html", {
        let context = Context::new();
        context
    })
}

pub fn configure(config: &mut ServiceConfig) {    
    config.service(resource("/").to(homepage));
    config.service(resource("/node").to(node));
    config.service(resource("/help").to(help));
}
