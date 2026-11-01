use crate::{
    application::todos::errors::TodoAppError,
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{
            ports::TodoRepo,
            Todo,
            TodoDescription,
            TodoStatus,
            TodoTitle,
        },
        users::UserId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateTodoInput {
    pub assignee_id: Option<Uuid>,
    pub board_id:    Uuid,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub team_id:     Uuid,
    pub title:       String,
}

pub struct CreateTodoUseCase {
    todo_repo: Arc<dyn TodoRepo>,
}

impl CreateTodoUseCase {
    pub fn new(todo_repo: Arc<dyn TodoRepo>) -> Self {
        Self { todo_repo }
    }

    pub async fn execute(&self, input: CreateTodoInput) -> Result<Todo, TodoAppError> {
        let description = input.description.map(TodoDescription::parse).transpose()?.unwrap_or_default();
        let status      = input.status.as_deref().map(TodoStatus::parse).transpose()?.unwrap_or_default();
        let title       = TodoTitle::parse(&input.title)?;
        let todo        = Todo::create(
            input.assignee_id.map(UserId::from),
            BoardId::from(input.board_id),
            description,
            status,
            TeamId::from(input.team_id),
            title,
        );
        let todo = self.todo_repo.create(&todo).await?;
        Ok(todo)
    }
}
