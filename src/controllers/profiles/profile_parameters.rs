use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProfileParameter {
    user_name: String
}