mod create;
mod delete;
mod find_by_id;
mod list;
mod row;
mod update;

use crate::domain::users::{
    ports::{UserRepo, UserRepoError},
    User,
    UserId,
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for PgUserRepo {
    async fn create(&self, user: &User) -> Result<User, UserRepoError> {
        create::create(&self.pool, user).await
    }

    async fn delete(&self, id: UserId) -> Result<(), UserRepoError> {
        delete::delete(&self.pool, id).await
    }

    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, UserRepoError> {
        find_by_id::find_by_id(&self.pool, id).await
    }

    async fn list(&self) -> Result<Vec<User>, UserRepoError> {
        list::list(&self.pool).await
    }

    async fn update(&self, user: &User) -> Result<User, UserRepoError> {
        update::update(&self.pool, user).await
    }
}
