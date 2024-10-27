use mockall::automock;
use sqlx::{PgPool, Error, query_as};
use async_trait::async_trait;
use crate::repository::repo::EntityId;
use super::profile_model::NewProfile;
use super::profile_model::Profile;

#[automock]
#[async_trait]
pub trait ProfileDb {
    async fn insert_profile(&self, conn: &PgPool, new_profile: NewProfile) -> Result<EntityId, Error>;
    async fn select_profile(&self, conn: &PgPool, id: i64) -> Result<Option<Profile>, Error>;
}

pub struct ProfileRepo;
#[async_trait]
impl ProfileDb for ProfileRepo {
    async fn insert_profile(&self, conn: &PgPool, new_profile: NewProfile) -> Result<EntityId, Error> {
        query_as::<_, EntityId>(r"
            insert into Profile
            (user_name, full_name, description, region, main_url, avatar)
            values
            ($1, $2, $3, $4, $5, $6)
            returning id
        ")
            .bind(new_profile.user_name)
            .bind(new_profile.full_name)
            .bind(new_profile.description)
            .bind(new_profile.region)
            .bind(new_profile.main_url)
            .bind(new_profile.avatar)
            .fetch_one(conn)
            .await
    }

    async fn select_profile(&self, conn: &PgPool, id: i64) -> Result<Option<Profile>, Error> {
        query_as::<_, Profile>(r"
            select *
            from Profile
            where id = $1
        ")
        .bind(id)
        .fetch_optional(conn)
        .await
    }
}

pub struct ProfileService<T: ProfileDb> {
    pub db: T
}
impl<T: ProfileDb> ProfileService<T> {
    pub fn new(db: T) -> Self {
        Self {
            db
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_profile() {
        let mut mock_db = MockProfileDb::new();

        mock_db
            .expect_select_profile();


    }
}