mod create;
mod delete;
mod find_by_id;
mod list;
mod row;
mod update;

use crate::domain::{
    boards::BoardId,
    teams::TeamId,
    todos::{
        ports::{TodoRepo, TodoRepoError},
        Todo,
        TodoId,
    },
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgTodoRepo {
    pool: PgPool,
}

impl PgTodoRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepo for PgTodoRepo {
    async fn create(&self, todo: &Todo) -> Result<Todo, TodoRepoError> {
        create::create(&self.pool, todo).await
    }

    async fn delete(&self, team_id: TeamId, board_id: BoardId, id: TodoId) -> Result<Todo, TodoRepoError> {
        delete::delete(&self.pool, team_id, board_id, id).await
    }

    async fn find_by_id(&self, team_id: TeamId, board_id: BoardId, id: TodoId) -> Result<Option<Todo>, TodoRepoError> {
        find_by_id::find_by_id(&self.pool, team_id, board_id, id).await
    }

    async fn list(&self, team_id: TeamId, board_id: BoardId) -> Result<Vec<Todo>, TodoRepoError> {
        list::list(&self.pool, team_id, board_id).await
    }

    async fn update(&self, todo: &Todo) -> Result<Todo, TodoRepoError> {
        update::update(&self.pool, todo).await
    }
}
