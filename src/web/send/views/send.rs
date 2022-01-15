use jelly::prelude::*;
use jelly::Result;
// use log::{debug, error, log_enabled, info, Level};

/// Returns an overview of everything in the system.
pub async fn send(request: HttpRequest) -> Result<HttpResponse> {
    let user = request.user()?;

    request.render(200, "send/index.html", {
        let mut context = Context::new();

        let id = user.id;
        let _name = user.name;
        let _is_admin = user.is_admin;
        let _is_anonymous = user.is_anonymous;

        context.insert("user_id_string", &format!("SEND.rs User ID: {:?}", id));
        context
    })
}
