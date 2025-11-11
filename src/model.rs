use sqlx::FromRow;
use uuid::Uuid;
use serde::Serialize;

pub type UserId = Uuid;
pub type Password = String;
pub type Username = String;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub password: Password,
}