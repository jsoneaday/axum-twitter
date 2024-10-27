use axum::{extract::Path, Json};
use crate::{controllers::lib::{errors::AppErrors, response::AppResponse}, repository::profiles::profile_model::Profile};
use super::profile_parameters::ProfileParameter;

pub async fn create_profile(Json(_new_profile): Json<ProfileParameter>) -> Result<i64, AppErrors> {
    Ok(1)
}

pub async fn get_profile(Path(id): Path<i64>) -> Result<AppResponse<Profile>, AppErrors> {
    Ok(AppResponse::JsonData(Profile {
        id,
        user_name: "dave".to_string(),
        full_name: "David Choi".to_string()
    }))
}