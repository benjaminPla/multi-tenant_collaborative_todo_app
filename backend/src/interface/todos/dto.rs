use crate::domain::todos::Todo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub assignee_id: Option<Uuid>,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub title:       String,
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub assignee_id: Option<Uuid>,
    pub board_id:    Uuid,
    pub created_at:  DateTime<Utc>,
    pub description: String,
    pub id:          Uuid,
    pub status:      &'static str,
    pub team_id:     Uuid,
    pub title:       String,
    pub updated_at:  DateTime<Utc>,
    pub version:     i32,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            assignee_id: todo.assignee_id.map(|id| id.as_uuid()),
            board_id:    todo.board_id.as_uuid(),
            created_at:  todo.created_at,
            description: todo.description.as_str().to_owned(),
            id:          todo.id.as_uuid(),
            status:      todo.status.as_str(),
            team_id:     todo.team_id.as_uuid(),
            title:       todo.title.as_str().to_owned(),
            updated_at:  todo.updated_at,
            version:     todo.version,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    #[serde(default, deserialize_with = "present")]
    pub assignee_id: Option<Option<Uuid>>,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub title:       Option<String>,
    pub version:     i32,
}

fn present<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Option<Uuid>>, D::Error> {
    Option::<Uuid>::deserialize(deserializer).map(Some)
}
