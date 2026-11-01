pub mod errors;
pub mod ports;
pub mod value_objects;

use crate::domain::{boards::BoardId, teams::TeamId, users::UserId};
use chrono::{DateTime, Utc};
pub use errors::TodoError;
pub use value_objects::{TodoDescription, TodoId, TodoStatus, TodoTitle};

#[derive(Clone, Debug)]
pub struct Todo {
    pub assignee_id: Option<UserId>,
    pub board_id:    BoardId,
    pub created_at:  DateTime<Utc>,
    pub description: TodoDescription,
    pub id:          TodoId,
    pub status:      TodoStatus,
    pub team_id:     TeamId,
    pub title:       TodoTitle,
    pub updated_at:  DateTime<Utc>,
    pub version:     i32,
}

impl Todo {
    pub fn create(
        assignee_id: Option<UserId>,
        board_id:    BoardId,
        description: TodoDescription,
        status:      TodoStatus,
        team_id:     TeamId,
        title:       TodoTitle,
    ) -> Self {
        let now = Utc::now();
        Self {
            assignee_id,
            board_id,
            created_at: now,
            description,
            id: TodoId::generate(),
            status,
            team_id,
            title,
            updated_at: now,
            version: 1,
        }
    }
}
