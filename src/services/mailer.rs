use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, SmtpTransport, Transport};
use lettre_email::{Email, EmailBuilder};
use std::env;

pub fn create_mailer() -> Result<SmtpTransport, &'static str> {
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

pub fn create_text_email(to: String, subject: String, text: String) -> Result<Email, &'static str> {
    EmailBuilder::new()
        .to(to)
        .from("notifications.malatynski@gmail.com")
        .subject(subject)
        .text(text)
        .build()
        .map_err(|_| "Cannot create mail")
}
