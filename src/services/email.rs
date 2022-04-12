use lettre::smtp::authentication::Credentials;
use lettre::smtp::response::Response;
use lettre::{SmtpClient, SmtpTransport, Transport};
use lettre_email::{Email, EmailBuilder};
use std::env;

fn create_mailer() -> Result<SmtpTransport, &'static str> {
    SmtpClient::new_simple("smtp.gmail.com")
        .map(|res| {
            res.credentials(Credentials::new(
                env::var("EMAIL_ADDRESS").unwrap(),
                env::var("EMAIL_PASSWORD").unwrap(),
            ))
        })
        .map(|res| res.transport())
        .map_err(|_| "Cannot create mailer")
}

pub fn send(
    mailer: &mut SmtpTransport,
    email: Email,
) -> Result<lettre::smtp::response::Response, &'static str> {
    mailer.send(email.into()).map_err(|_| "Cannot send email")
}

fn create_text_email(to: String, subject: String, text: String) -> Result<Email, &'static str> {
    EmailBuilder::new()
        .to(to)
        .from("notifications.malatynski@gmail.com")
        .subject(subject)
        .text(text)
        .build()
        .map_err(|_| "Cannot create mail")
}

pub fn email_verification(email: &String) -> Result<Response, &'static str> {
    let email = create_text_email(
        email.to_string(),
        "Email verification".into(),
        "Verify your email".into(),
    )?;
    let mut mailer = create_mailer()?;
    send(&mut mailer, email)
}
