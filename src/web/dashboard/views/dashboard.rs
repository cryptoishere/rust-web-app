use jelly::prelude::*;
use jelly::Result;

use log::{debug, error, log_enabled, info, Level};

/// Returns an overview of everything in the system.
pub async fn dashboard(request: HttpRequest) -> Result<HttpResponse> {
    let user = request.user()?;

    // LOG SYSTEM
    info!(target: "yak_events", "log request from dashboard {:?}", "testing logger ....");

    request.render(200, "dashboard/index.html", {
        let mut context = Context::new();

        let id = user.id;
        let name = user.name;
        let is_admin = user.is_admin;
        let is_anonymous = user.is_anonymous;

        context.insert("integer_example", &42);
        // context.insert("logger", format!("test logger {:?}",));
        context.insert("user_instance", &format!("DASHBOARD.rs id: {:?} name: {:?} is admin: {:?} is anonymous: {:?}", id, name, is_admin, is_anonymous));
        context
    })
}
