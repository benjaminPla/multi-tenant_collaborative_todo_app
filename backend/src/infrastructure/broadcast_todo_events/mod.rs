use crate::domain::todos::{
    ports::{TodoEvents, TodoFeed, TodoFeedItem},
    TodoEvent,
};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

pub struct BroadcastTodoEvents {
    sender: broadcast::Sender<TodoEvent>,
}

impl BroadcastTodoEvents {
    const CAPACITY: usize = 256;

    pub fn new() -> Self {
        Self { sender: broadcast::channel(Self::CAPACITY).0 }
    }
}

impl TodoEvents for BroadcastTodoEvents {
    fn publish(&self, event: TodoEvent) {
        let _ = self.sender.send(event);
    }

    fn subscribe(&self) -> TodoFeed {
        Box::pin(BroadcastStream::new(self.sender.subscribe()).map(|item| match item {
            Ok(event) => TodoFeedItem::Event(event),
            Err(_)    => TodoFeedItem::Lagged,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{Todo, TodoDescription, TodoEventKind, TodoStatus, TodoTitle},
        users::{Email, PasswordHash, User},
    };

    fn event() -> TodoEvent {
        let actor = User::register(Email::parse("alice@acme.test").unwrap(), PasswordHash::new("hash".into()));
        let todo  = Todo::create(
            None,
            BoardId::generate(),
            TodoDescription::default(),
            TodoStatus::Todo,
            TeamId::generate(),
            TodoTitle::parse("Deploy").unwrap(),
        );
        TodoEvent::by(&actor, TodoEventKind::Created, todo)
    }

    #[tokio::test]
    async fn subscribers_receive_published_events() {
        let bus      = BroadcastTodoEvents::new();
        let mut feed = bus.subscribe();
        bus.publish(event());
        assert!(matches!(feed.next().await, Some(TodoFeedItem::Event(_))));
    }

    #[tokio::test]
    async fn slow_subscribers_are_told_they_lagged() {
        let bus      = BroadcastTodoEvents::new();
        let mut feed = bus.subscribe();
        for _ in 0..=BroadcastTodoEvents::CAPACITY {
            bus.publish(event());
        }
        assert!(matches!(feed.next().await, Some(TodoFeedItem::Lagged)));
    }
}
