use crate::auth::apikey::ApiKey;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi]
#[get("/ping")]
pub async fn ping(_api_key: ApiKey) -> Json<String> {
    Json::from("Pong".to_string())
}
