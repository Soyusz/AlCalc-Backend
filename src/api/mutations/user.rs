use crate::api::utils::session::SessionTokenReturnType;
use rocket::response::content::Html;
use uuid::Uuid;
use crate::services::follow as FollowService;
use crate::services::template as TemplateService;
use crate::api::utils::structs::{LoginCred, PhotoArg};
use crate::api::utils::Response;
use crate::api::DBPool;
use crate::model::user::{NewUser, User as UserModel};
use crate::model::follow::Follow as FollowModel;
use crate::services::session as SessionService;
use crate::services::user as UserService;
use crate::types::token::SessionToken;
use rocket::response::status;
use rocket::{delete, get, post, put, routes, Route};
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<new_user>", rank = 1)]
fn post_new(conn: DBPool, new_user: Json<NewUser>) -> Response<UserModel> {
    Ok(new_user.into_inner())
        .and_then(|user| UserService::insert_user(user, conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[post("/", rank = 2)]
fn post_new_invalid() -> status::BadRequest<()> {
    status::BadRequest(None)
}

#[post("/login", format = "application/json", data = "<login_cred>", rank = 1)]
fn login(conn: DBPool, login_cred: Json<LoginCred>) -> Response<SessionTokenReturnType> {
    Ok(())
        .map(|_| login_cred.into_inner())
        .and_then(|login_cred| SessionService::login(login_cred, &conn))
        .map(|token| Json(SessionTokenReturnType { token }))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[post("/login", rank = 2)]
fn login_invalid() -> status::BadRequest<&'static str> {
    status::BadRequest(Some("Invalid payload"))
}

#[put("/photo", format = "application/json", data = "<arg>")]
fn put_photo(
    arg: Json<PhotoArg>,
    session_token: SessionToken,
    conn: DBPool,
) -> Result<Option<String>, status::BadRequest<&'static str>> {
    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| {
            UserService::update_photo(arg.into_inner().photo, session.user_id, &conn)
        })
        .map(|r| r.photo)
        .map_err(|e| status::BadRequest(Some(e)))
}

#[get("/verify_email/<token_arg>")]
fn verify_account(token_arg: String, conn: DBPool) -> Result<Html<String>,status::BadRequest<&'static str>> {
    Ok(token_arg.as_str())
        .and_then(|token| UserService::verify_account(token.to_string(), &conn))
        .map(|r| Json(r))
        .and_then(|_| TemplateService::get_close_this_template() )
        .and_then(|s| Ok(Html(s)))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[get("/confirm_session/<token_arg>", format="text/html")]
fn confirm_session(token_arg: String, conn: DBPool) -> Result<Html<String>,status::BadRequest<&'static str>> {
    Ok(token_arg.as_str())
        .and_then(|token| SessionService::authorize(token.to_string(), &conn))
        .and_then(|_| TemplateService::get_close_this_template() )
        .and_then(|s| Ok(Html(s)))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[post("/follow/<id_string>", rank=1)]
pub fn follow(session_token: SessionToken, conn: DBPool, id_string: String) -> Response<FollowModel> {
    let user_id=Uuid::parse_str(id_string.as_str()).map_err(|_| status::BadRequest(Some("Invalid id")))?;
    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| FollowService::follow(session.user_id, user_id, &conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

#[post("/follow/<_id_string>", rank=2)]
pub fn follow_unauthorized(_id_string: String) -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

#[delete("/unfollow/<id_string>", rank=1)]
pub fn unfollow(session_token: SessionToken, conn: DBPool, id_string: String) -> Response<FollowModel> {
    let user_id=Uuid::parse_str(id_string.as_str()).map_err(|_| status::BadRequest(Some("Invalid id")))?;
    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| FollowService::unfollow(session.user_id, user_id, &conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

#[delete("/unfollow/<_id_string>", rank=2)]
pub fn unfollow_unauthorized(_id_string: String) -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![
        follow,
        unfollow,
        follow_unauthorized,
        unfollow_unauthorized,
        confirm_session,
        post_new,
        post_new_invalid,
        login,
        login_invalid,
        put_photo,
        verify_account
    ]
}
