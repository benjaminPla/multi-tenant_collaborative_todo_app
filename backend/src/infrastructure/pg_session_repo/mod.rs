mod create;
mod delete;
mod delete_by_user_id;
mod find_by_token_hash;
mod row;

use crate::domain::{
    sessions::{
        ports::{SessionRepo, SessionRepoError},
        Session,
        TokenHash,
    },
    users::UserId,
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgSessionRepo {
    pool: PgPool,
}

impl PgSessionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepo for PgSessionRepo {
    async fn create(&self, session: &Session) -> Result<(), SessionRepoError> {
        create::create(&self.pool, session).await
    }

    async fn delete(&self, token_hash: &TokenHash) -> Result<(), SessionRepoError> {
        delete::delete(&self.pool, token_hash).await
    }

    async fn delete_by_user_id(&self, user_id: UserId) -> Result<(), SessionRepoError> {
        delete_by_user_id::delete_by_user_id(&self.pool, user_id).await
    }

    async fn find_by_token_hash(&self, token_hash: &TokenHash) -> Result<Option<Session>, SessionRepoError> {
        find_by_token_hash::find_by_token_hash(&self.pool, token_hash).await
    }
}
