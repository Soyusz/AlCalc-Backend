use dotenv::dotenv;
use rocket::config::{Config, Environment, Value};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method};
use rocket::{Request, Response, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::collections::HashMap;

pub mod mutations;
pub mod queries;
pub mod utils;

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PUT, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
    // Access-Control-Allow-Methods: POST, GET, OPTIONS, DELETE
}

#[database("alcalc-db")]
pub struct DBPool(diesel::PgConnection);

pub fn init_routes() -> Rocket {
    dotenv().ok();
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", std::env::var("DATABASE_URL").unwrap());
    databases.insert("alcalc-db", Value::from(database_config));
    let port_number: u16 = std::env::var("ROCKET_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let config = Config::build(Environment::Staging)
        .port(port_number)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Delete,
                Method::Put,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);

    rocket::custom(config)
        .attach(cors.to_cors().unwrap())
        .attach(DBPool::fairing())
        .mount("/user", mutations::user::get_routes())
        .mount("/user", queries::user::get_routes())
        .mount("/entry", queries::entry::get_routes())
        .mount("/entry", mutations::entry::get_routes())
        .mount("/post", mutations::post::get_routes())
        .mount("/post", queries::post::get_routes())
        .mount("/like", queries::like::get_routes())
        .mount("/like", mutations::like::get_routes())
}
