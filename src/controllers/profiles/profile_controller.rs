use crate::{controllers::lib::response::AppResponse, repository::profiles::profile_model::Profile};

pub async fn get_profile() -> AppResponse<Profile> {
    AppResponse::JsonData(Profile {
        id: 1,
        user_name: "dave".to_string(),
        full_name: "David Choi".to_string()
    })
}