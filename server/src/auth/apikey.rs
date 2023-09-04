const API_KEY: &str = "509fdff0-4236-482b-b1a6-613e7666997d";

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use sqlx::PgPool;
use crate::models::{User, UserType};

pub struct ApiKey(String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey {
    type Error = &'static str;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {

        let db_pool: &PgPool = request.rocket().state::<PgPool>().unwrap();

        match request.headers().get_one("x-api-key") {
            Some(key) => {
                let user: Option<User> = sqlx::query_as!(User, r#"SELECT id, username, user_type AS "user_type: crate::models::UserType", api_key FROM users WHERE api_key = $1"#, key)
                    .fetch_one(db_pool)
                    .await
                    .ok();

                match user {
                    Some(u) => {
                        info!("Found apiKey for user {:?}", u.username);
                        // We found a key - All good
                        Outcome::Success(ApiKey(key.to_owned()))
                    },
                    None => Outcome::Failure((Status::Unauthorized, "Unknown API Key"))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, "Unknown API Key")),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for ApiKey {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let sec_scheme = SecurityScheme {
            description: Some("Requires an API Key access".to_owned()),
            data: SecuritySchemeData::ApiKey {
                name: "x-api-key".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Default::default(),
        };

        let mut sec_req = rocket_okapi::okapi::openapi3::SecurityRequirement::new();
        sec_req.insert("ApiKeyAuth".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            sec_scheme,
            sec_req,
        ))
    }
}
