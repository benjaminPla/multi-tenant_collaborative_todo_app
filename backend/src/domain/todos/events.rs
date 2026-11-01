use crate::domain::{
    todos::Todo,
    users::{Email, User, UserId},
};

#[derive(Clone, Debug)]
pub struct TodoEvent {
    pub actor_email: Email,
    pub actor_id:    UserId,
    pub kind:        TodoEventKind,
    pub todo:        Todo,
}

impl TodoEvent {
    pub fn by(actor: &User, kind: TodoEventKind, todo: Todo) -> Self {
        Self { actor_email: actor.email.clone(), actor_id: actor.id, kind, todo }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TodoEventKind {
    Created,
    Deleted,
    Updated,
}
