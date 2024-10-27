use serde::Serialize;

#[derive(Serialize)]
pub struct Profile {
    pub id: i64,
    pub user_name: String,
    pub full_name: String
}