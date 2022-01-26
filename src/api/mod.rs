use rocket::{routes, Rocket};
use dotenv::dotenv;
use rocket_contrib::databases::diesel;
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};


pub mod entry;

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[database("alcalc-db")]
pub struct DBPool(diesel::PgConnection);

pub fn init_routes() -> Rocket {
    dotenv().ok();
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url",std::env::var("DATABASE_URL").unwrap());
    databases.insert("alcalc-db",Value::from(database_config));
    let port_number: u16 = std::env::var("ROCKET_PORT").unwrap().parse::<u16>().unwrap();
    let config = Config::build(Environment::Staging)
        .port(port_number)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    let entry_routes = routes![
        entry::get_all,
        entry::post_new,
        entry::verify,
        entry::get_verified
    ];

    rocket::custom(config)
        .attach(CORS)
        .attach(DBPool::fairing())
        .mount("/entry", entry_routes)
}
