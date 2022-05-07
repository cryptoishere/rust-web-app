//! Configure scope section.

use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};
use jelly::guards::Auth;

mod views;

pub fn configure(config: &mut ServiceConfig) {
    let guard = Auth {
        redirect_to: "/accounts/login/"
    };
    

    config.service(scope("/upload/").wrap(guard)
        .service(resource("")
                            .route(get().to(views::upload))
                            .route(post().to(views::upload_file))
        )
    );
}
