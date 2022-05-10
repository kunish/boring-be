use std::env;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use dotenv::dotenv;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod actions;
mod api;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager)?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials(),
            )
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(api::retrieve)
            .service(api::create)
            .service(api::update_title)
            .service(api::update_body)
            .service(api::publish)
            .service(api::unpublish)
            .service(api::delete)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
