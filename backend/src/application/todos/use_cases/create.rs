use crate::{
    application::todos::errors::TodoAppError,
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{
            ports::{TodoEvents, TodoRepo},
            Todo,
            TodoDescription,
            TodoEvent,
            TodoEventKind,
            TodoStatus,
            TodoTitle,
        },
        users::{User, UserId},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateTodoInput {
    pub actor:       User,
    pub assignee_id: Option<Uuid>,
    pub board_id:    Uuid,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub team_id:     Uuid,
    pub title:       String,
}

pub struct CreateTodoUseCase {
    todo_events: Arc<dyn TodoEvents>,
    todo_repo:   Arc<dyn TodoRepo>,
}

impl CreateTodoUseCase {
    pub fn new(todo_events: Arc<dyn TodoEvents>, todo_repo: Arc<dyn TodoRepo>) -> Self {
        Self { todo_events, todo_repo }
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
        self.todo_events.publish(TodoEvent::by(&input.actor, TodoEventKind::Created, todo.clone()));
        Ok(todo)
    }
}
