mod team_role;
#[cfg(test)]
mod tests;

use crate::domain::{
    authz::ports::{Pip, PipError},
    teams::{TeamId, TeamRole},
    users::UserId,
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgPip {
    pool: PgPool,
}

impl PgPip {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Pip for PgPip {
    async fn team_role(&self, team_id: TeamId, user_id: UserId) -> Result<Option<TeamRole>, PipError> {
        team_role::team_role(&self.pool, team_id, user_id).await
    }
}
