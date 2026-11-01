use crate::{
    application::todos::errors::TodoAppError,
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{
            ports::{TodoEvents, TodoRepo, TodoRepoError},
            Todo,
            TodoDescription,
            TodoEvent,
            TodoEventKind,
            TodoId,
            TodoStatus,
            TodoTitle,
        },
        users::{User, UserId},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateTodoInput {
    pub actor:       User,
    pub assignee_id: Option<Option<Uuid>>,
    pub board_id:    Uuid,
    pub description: Option<String>,
    pub id:          Uuid,
    pub status:      Option<String>,
    pub team_id:     Uuid,
    pub title:       Option<String>,
    pub version:     i32,
}

pub struct UpdateTodoUseCase {
    todo_events: Arc<dyn TodoEvents>,
    todo_repo:   Arc<dyn TodoRepo>,
}

impl UpdateTodoUseCase {
    pub fn new(todo_events: Arc<dyn TodoEvents>, todo_repo: Arc<dyn TodoRepo>) -> Self {
        Self { todo_events, todo_repo }
    }

    pub async fn execute(&self, input: UpdateTodoInput) -> Result<Todo, TodoAppError> {
        let description = input.description.map(TodoDescription::parse).transpose()?;
        let status      = input.status.as_deref().map(TodoStatus::parse).transpose()?;
        let title       = input.title.as_deref().map(TodoTitle::parse).transpose()?;

        let mut todo = self.todo_repo
            .find_by_id(TeamId::from(input.team_id), BoardId::from(input.board_id), TodoId::from(input.id))
            .await?
            .ok_or(TodoRepoError::NotFound)?;
        if todo.version != input.version {
            return Err(TodoRepoError::VersionConflict.into());
        }

        if let Some(assignee_id) = input.assignee_id {
            todo.assignee_id = assignee_id.map(UserId::from);
        }
        if let Some(description) = description {
            todo.description = description;
        }
        if let Some(status) = status {
            todo.status = status;
        }
        if let Some(title) = title {
            todo.title = title;
        }

        let todo = self.todo_repo.update(&todo).await?;
        self.todo_events.publish(TodoEvent::by(&input.actor, TodoEventKind::Updated, todo.clone()));
        Ok(todo)
    }
}
