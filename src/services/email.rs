use crate::model::user::User;
use crate::services::mailer as MailerService;
use crate::services::token as TokenService;
use lettre::smtp::response::Response;
use std::env;
use uuid::Uuid;

pub fn email_verification(user: &User) -> Result<Response, &'static str> {
    let be_url = env::var("BE_URL").unwrap();
    let token = TokenService::create_account_verification_token(&user.id)?;
    let link = be_url + "/user/verify_email/" + &token;
    let mail = MailerService::create_text_email(
        user.email.to_string(),
        "Email verification".into(),
        (String::from("Verify your email: ") + &link).into(),
    )?;
    let mut mailer = MailerService::create_mailer()?;
    MailerService::send(&mut mailer, mail)
}

pub fn session_verification(user_email: String, session_id: Uuid) -> Result<Response, &'static str> {
    let be_url = env::var("BE_URL").unwrap();
    let token = TokenService::create_session_verification_token(&session_id)?;
    let link = be_url + "/user/confirm_session/" + &token;
    let mail = MailerService::create_text_email(
        user_email.to_string(),
        "Alkierz login".into(),
        (String::from("Validate your session: ") + &link).into(),
    )?;
    let mut mailer = MailerService::create_mailer()?;
    MailerService::send(&mut mailer, mail)
}
