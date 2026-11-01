mod create;
mod delete;
mod find_by_id;
mod find_role;
mod list;
mod list_by_user_id;
mod row;
mod update;

use crate::domain::{
    teams::{
        ports::{TeamRepo, TeamRepoError},
        Team,
        TeamId,
        TeamRole,
    },
    users::UserId,
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgTeamRepo {
    pool: PgPool,
}

impl PgTeamRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamRepo for PgTeamRepo {
    async fn create(&self, team: &Team, owner_id: UserId) -> Result<Team, TeamRepoError> {
        create::create(&self.pool, team, owner_id).await
    }

    async fn delete(&self, id: TeamId) -> Result<(), TeamRepoError> {
        delete::delete(&self.pool, id).await
    }

    async fn find_by_id(&self, id: TeamId) -> Result<Option<Team>, TeamRepoError> {
        find_by_id::find_by_id(&self.pool, id).await
    }

    async fn find_role(&self, id: TeamId, user_id: UserId) -> Result<Option<TeamRole>, TeamRepoError> {
        find_role::find_role(&self.pool, id, user_id).await
    }

    async fn list(&self) -> Result<Vec<Team>, TeamRepoError> {
        list::list(&self.pool).await
    }

    async fn list_by_user_id(&self, user_id: UserId) -> Result<Vec<Team>, TeamRepoError> {
        list_by_user_id::list_by_user_id(&self.pool, user_id).await
    }

    async fn update(&self, team: &Team) -> Result<Team, TeamRepoError> {
        update::update(&self.pool, team).await
    }
}
