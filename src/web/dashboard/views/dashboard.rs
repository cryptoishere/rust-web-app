use jelly::prelude::*;
use jelly::Result;

// use log::{debug, error, log_enabled, info, Level};

// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{Message, SmtpTransport, Transport};



/// Returns an overview of everything in the system.
pub async fn dashboard(request: HttpRequest) -> Result<HttpResponse> {
    let user = request.user()?;

    // LOG SYSTEM
    // let testcopy = request.clone();
    // info!(target: "yak_events", "log request from dashboard {:?}", testcopy);

    // SEND EMAIL
    // let email = Message::builder()
    //     .from("NoBody <info@livedev.info>".parse().unwrap())
    //     .reply_to("Crypto <cryptoishere88@gmail.com>".parse().unwrap())
    //     .to("Hei <cryptoishere88@gmail.com>".parse().unwrap())
    //     .subject("Happy new year")
    //     .body(String::from("Be happy!"))
    //     .unwrap();
    // let creds = Credentials::new("mihhailk88@gmail.com".to_string(), "waafyyietqzqeuxr".to_string());

    // // Open a remote connection to gmail
    // let mailer = SmtpTransport::relay("smtp.gmail.com")
    //     .unwrap()
    //     .credentials(creds)
    //     .build();

    // // Send the email
    // match mailer.send(&email) {
    //     Ok(_) => info!("Email sent successfully!"),
    //     Err(e) => panic!("Could not send email: {:?}", e),
    // }
    /////////

    request.render(200, "dashboard/index.html", {
        let mut context = Context::new();

        let id = user.id;
        let name = user.name;
        let is_admin = user.is_admin;
        let is_anonymous = user.is_anonymous;
        
        // info!(target: "target1", "id  {:?}", id);
        // info!(target: "target2", "name {:?}", name);
        // info!(target: "target3", "is_admin {:?}", is_admin);
        // info!(target: "target4", "is_anonymous {:?}", is_anonymous);

        context.insert("integer_example", &42);
        // context.insert("logger", format!("test logger {:?}",));
        context.insert("user_instance", &format!("DASHBOARD.rs id: {:?} name: {:?} is admin: {:?} is anonymous: {:?}", id, name, is_admin, is_anonymous));
        context
    })
}
