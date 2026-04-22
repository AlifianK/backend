use crate::models::user::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            email: u.email,
        }
    }
}
