use sqlx::{FromRow, Type};
use crate::models::UserType::USER;

#[derive(Debug, FromRow)]
pub struct User {
    #[sqlx(default)]
    pub id: i64,
    pub username: Option<String>,
    pub user_type: UserType,
    pub api_key: Option<String>,
}

#[derive(Debug, Type)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    ADMIN,
    USER
}

impl From<Option<UserType>> for UserType{
    fn from(value: Option<UserType>) -> Self {
        match value {
            None => {USER} // Probably want to return an err
            Some(x) => {x}
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            username: Some("unknown".to_string()),
            user_type: USER,
            api_key: Some("xxx".to_string())
        }
    }
}