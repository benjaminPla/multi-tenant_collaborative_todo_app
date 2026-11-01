use crate::{
    application::todos::errors::TodoAppError,
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{
            ports::{TodoEvents, TodoRepo},
            TodoEvent,
            TodoEventKind,
            TodoId,
        },
        users::User,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteTodoInput {
    pub actor:    User,
    pub board_id: Uuid,
    pub id:       Uuid,
    pub team_id:  Uuid,
}

pub struct DeleteTodoUseCase {
    todo_events: Arc<dyn TodoEvents>,
    todo_repo:   Arc<dyn TodoRepo>,
}

impl DeleteTodoUseCase {
    pub fn new(todo_events: Arc<dyn TodoEvents>, todo_repo: Arc<dyn TodoRepo>) -> Self {
        Self { todo_events, todo_repo }
    }

    pub async fn execute(&self, input: DeleteTodoInput) -> Result<(), TodoAppError> {
        let todo = self.todo_repo
            .delete(TeamId::from(input.team_id), BoardId::from(input.board_id), TodoId::from(input.id))
            .await?;
        self.todo_events.publish(TodoEvent::by(&input.actor, TodoEventKind::Deleted, todo));
        Ok(())
    }
}
