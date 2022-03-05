use crate::api::DBPool;
use crate::model::{token as TokenModel, user as UserModel};
use crate::services::user as UserService;
use rocket::request::{FromRequest, Outcome, Request};

impl<'a, 'r> FromRequest<'a, 'r> for UserModel::User {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<UserModel::User, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        let token = keys[0].to_string();

        // let validated_token = TokenModel::validate(token).unwrap();
        // let res = UserService::get_user(validated_token.user_id, conn: DBPool);

        let user = UserModel::User {
            id: uuid::Uuid::new_v4(),
            email: "eeee".to_string(),
            name: "eee".to_string(),
            email_verified: false,
        };

        Outcome::Success(user)
    }
}
