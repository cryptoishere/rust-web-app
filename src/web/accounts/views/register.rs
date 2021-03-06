use jelly::prelude::*;
use jelly::actix_web::{HttpRequest, web::{Form}};
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;

use crate::web::accounts::Account;
use crate::web::accounts::jobs::{SendVerifyAccountEmail, SendAccountOddRegisterAttemptEmail};
use crate::web::accounts::forms::{NewAccountForm};

pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    request.render(200, "accounts/register.html", {
        let mut ctx = Context::new();
        ctx.insert("form", &NewAccountForm::default());
        ctx
    })
}

pub async fn create_account(
    request: HttpRequest,
    form: Form<NewAccountForm>
) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "accounts/register.html", {
            let mut ctx = Context::new();
            ctx.insert("form", &form);
            ctx
        });
    }

    // Catch this error
    // if duplicate:
    //  - send email to existing user asking if they were trying to sign in
    //  - pass requesting user through normal "fake" flow to avoid leaking if
    //      an account exists?
    let db = request.db_pool()?;
    match Account::register(&form, db).await {
        Ok(uid) => {
            request.queue(SendVerifyAccountEmail {
                to: uid
            })?;
        },

        Err(e) => {
            error!("Error with registering: {:?}", e);
            request.queue(SendAccountOddRegisterAttemptEmail {
                to: form.email.value.clone()
            })?;
        }
    }

    // No matter what, just appear as if it worked.
    request.redirect("/accounts/verify/")
}
