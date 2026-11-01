mod create;
mod delete;
mod find_by_id;
mod list;
mod row;
mod update;

use crate::domain::{
    boards::{
        ports::{BoardRepo, BoardRepoError},
        Board,
        BoardId,
    },
    teams::TeamId,
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgBoardRepo {
    pool: PgPool,
}

impl PgBoardRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BoardRepo for PgBoardRepo {
    async fn create(&self, board: &Board) -> Result<Board, BoardRepoError> {
        create::create(&self.pool, board).await
    }

    async fn delete(&self, team_id: TeamId, id: BoardId) -> Result<(), BoardRepoError> {
        delete::delete(&self.pool, team_id, id).await
    }

    async fn find_by_id(&self, team_id: TeamId, id: BoardId) -> Result<Option<Board>, BoardRepoError> {
        find_by_id::find_by_id(&self.pool, team_id, id).await
    }

    async fn list(&self, team_id: TeamId) -> Result<Vec<Board>, BoardRepoError> {
        list::list(&self.pool, team_id).await
    }

    async fn update(&self, board: &Board) -> Result<Board, BoardRepoError> {
        update::update(&self.pool, board).await
    }
}
