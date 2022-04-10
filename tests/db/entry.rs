use lib::db::user as UserRepo;
use diesel::{pg::PgConnection, Connection}; 
use dotenv::dotenv;
use diesel::ConnectionError;
use lib::model::user::{User,NewUser};
use diesel::result::Error;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url)
}

#[test]
fn insert_user_test(){
    let conn = establish_connection()
        .unwrap_or_else(|e| panic!("Error establishing conncection: '{}'",e));
    let test_new_user = NewUser {
        name: "User1".to_string(),
        email: "user1@mail.com".to_string()
    };
    let test_user = User::create_user(test_new_user);

    conn.test_transaction::<_,Error,_>(|| {
        UserRepo::add_new(test_user, &conn).unwrap();
        Ok(())
    })
}


