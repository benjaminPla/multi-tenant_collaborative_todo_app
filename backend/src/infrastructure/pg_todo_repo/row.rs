use crate::domain::{
    boards::BoardId,
    teams::TeamId,
    todos::{
        ports::TodoRepoError,
        Todo,
        TodoDescription,
        TodoError,
        TodoId,
        TodoStatus,
        TodoTitle,
    },
    users::UserId,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct TodoRow {
    assignee_id: Option<Uuid>,
    board_id:    Uuid,
    created_at:  DateTime<Utc>,
    description: String,
    id:          Uuid,
    status:      String,
    team_id:     Uuid,
    title:       String,
    updated_at:  DateTime<Utc>,
    version:     i32,
}

impl TryFrom<TodoRow> for Todo {
    type Error = TodoRepoError;

    fn try_from(row: TodoRow) -> Result<Self, Self::Error> {
        let unexpected = |err: TodoError| TodoRepoError::Unexpected(err.into());
        Ok(Self {
            assignee_id: row.assignee_id.map(UserId::from),
            board_id:    BoardId::from(row.board_id),
            created_at:  row.created_at,
            description: TodoDescription::parse(row.description).map_err(unexpected)?,
            id:          TodoId::from(row.id),
            status:      TodoStatus::parse(&row.status).map_err(unexpected)?,
            team_id:     TeamId::from(row.team_id),
            title:       TodoTitle::parse(&row.title).map_err(unexpected)?,
            updated_at:  row.updated_at,
            version:     row.version,
        })
    }
}

pub fn map_err(err: sqlx::Error) -> TodoRepoError {
    match err.as_database_error().and_then(|db_err| db_err.constraint()) {
        Some("todos_team_id_assignee_id_fkey") => TodoRepoError::AssigneeNotMember,
        Some("todos_team_id_board_id_fkey")    => TodoRepoError::BoardNotFound,
        _                                      => TodoRepoError::Unexpected(err.into()),
    }
}
