mod add_member;
mod create;
mod delete;
mod find_by_id;
mod find_role;
mod list;
mod list_by_user_id;
mod list_members;
mod lock_member;
mod remove_member;
mod row;
mod update;
mod update_member_role;

use crate::domain::{
    teams::{
        ports::{TeamRepo, TeamRepoError},
        Member,
        Team,
        TeamId,
        TeamRole,
    },
    users::{Email, UserId},
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
    async fn add_member(&self, id: TeamId, email: &Email, role: TeamRole) -> Result<Member, TeamRepoError> {
        add_member::add_member(&self.pool, id, email, role).await
    }

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

    async fn list_members(&self, id: TeamId) -> Result<Vec<Member>, TeamRepoError> {
        list_members::list_members(&self.pool, id).await
    }

    async fn remove_member(&self, id: TeamId, user_id: UserId) -> Result<(), TeamRepoError> {
        remove_member::remove_member(&self.pool, id, user_id).await
    }

    async fn update(&self, team: &Team) -> Result<Team, TeamRepoError> {
        update::update(&self.pool, team).await
    }

    async fn update_member_role(&self, id: TeamId, user_id: UserId, role: TeamRole) -> Result<Member, TeamRepoError> {
        update_member_role::update_member_role(&self.pool, id, user_id, role).await
    }
}
