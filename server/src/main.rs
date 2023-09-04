mod auth;
mod endpoints;
mod models;

#[macro_use]
extern crate rocket;

use rocket::serde::json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi_get_routes, openapi_get_spec};
use std::fs::File;
use std::io::Write;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;
use sqlx::{migrate, PgPool};

//TODO Find a way to have one ref of published routes

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {

    // TODO Build spec fairing. Fail launch if write_open_api_spec fails
    write_open_api_spec();
    migrate!().run(&pool).await.expect("Failed to run migrations");

    let rocket = rocket::build()
        .mount("/", openapi_get_routes![endpoints::ping::ping])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        ).manage(pool);

    Ok(rocket.into())
}

fn write_open_api_spec() {
    let spec = openapi_get_spec![endpoints::ping::ping];
    let mut spec_file = File::create("open-api.json").expect("Failed to create open-api.json");
    spec_file
        .write_all(
            json::to_pretty_string(&spec)
                .expect("Failed to unmarshall openapi")
                .as_bytes(),
        )
        .expect("Failed to write to openAPI");
}
