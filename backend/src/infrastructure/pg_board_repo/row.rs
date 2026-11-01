use crate::domain::{
    boards::{ports::BoardRepoError, Board, BoardId, BoardName},
    teams::TeamId,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct BoardRow {
    created_at: DateTime<Utc>,
    id:         Uuid,
    name:       String,
    team_id:    Uuid,
}

impl TryFrom<BoardRow> for Board {
    type Error = BoardRepoError;

    fn try_from(row: BoardRow) -> Result<Self, Self::Error> {
        Ok(Self {
            created_at: row.created_at,
            id:         BoardId::from(row.id),
            name:       BoardName::parse(&row.name).map_err(|err| BoardRepoError::Unexpected(err.into()))?,
            team_id:    TeamId::from(row.team_id),
        })
    }
}

pub fn map_err(err: sqlx::Error) -> BoardRepoError {
    match err.as_database_error() {
        Some(db_err) if db_err.is_foreign_key_violation() => BoardRepoError::NotFound,
        _                                                 => BoardRepoError::Unexpected(err.into()),
    }
}
