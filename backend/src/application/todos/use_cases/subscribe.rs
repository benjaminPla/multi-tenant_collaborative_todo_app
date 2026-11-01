use crate::{
    application::todos::errors::TodoAppError,
    domain::{
        boards::{
            ports::{BoardRepo, BoardRepoError},
            BoardId,
        },
        teams::TeamId,
        todos::ports::{TodoEvents, TodoFeed, TodoFeedItem},
    },
};
use std::sync::Arc;
use tokio_stream::StreamExt;
use uuid::Uuid;

pub struct SubscribeTodosInput {
    pub board_id: Uuid,
    pub team_id:  Uuid,
}

pub struct SubscribeTodosUseCase {
    board_repo:  Arc<dyn BoardRepo>,
    todo_events: Arc<dyn TodoEvents>,
}

impl SubscribeTodosUseCase {
    pub fn new(board_repo: Arc<dyn BoardRepo>, todo_events: Arc<dyn TodoEvents>) -> Self {
        Self { board_repo, todo_events }
    }

    pub async fn execute(&self, input: SubscribeTodosInput) -> Result<TodoFeed, TodoAppError> {
        let board_id = BoardId::from(input.board_id);
        self.board_repo.find_by_id(TeamId::from(input.team_id), board_id).await?.ok_or(BoardRepoError::NotFound)?;
        let feed = self.todo_events.subscribe().filter(move |item| match item {
            TodoFeedItem::Event(event) => event.todo.board_id == board_id,
            TodoFeedItem::Lagged       => true,
        });
        Ok(Box::pin(feed))
    }
}
