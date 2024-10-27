use crate::{controllers::lib::{errors::AppErrors, response::AppResponse}, repository::profiles::profile_model::Profile};

pub async fn get_profile() -> Result<AppResponse<Profile>, AppErrors> {
    Ok(AppResponse::JsonData(Profile {
        id: 1,
        user_name: "dave".to_string(),
        full_name: "David Choi".to_string()
    }))
}