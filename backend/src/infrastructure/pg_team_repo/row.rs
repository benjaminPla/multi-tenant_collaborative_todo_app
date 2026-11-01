use crate::domain::teams::{ports::TeamRepoError, Team, TeamId, TeamName, TeamRole};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct TeamRow {
    created_at: DateTime<Utc>,
    id:         Uuid,
    name:       String,
}

impl TryFrom<TeamRow> for Team {
    type Error = TeamRepoError;

    fn try_from(row: TeamRow) -> Result<Self, Self::Error> {
        Ok(Self {
            created_at: row.created_at,
            id:         TeamId::from(row.id),
            name:       TeamName::parse(&row.name).map_err(|err| TeamRepoError::Unexpected(err.into()))?,
        })
    }
}

pub fn map_err(err: sqlx::Error) -> TeamRepoError {
    match err.as_database_error() {
        Some(db_err) if db_err.is_foreign_key_violation() => TeamRepoError::NotFound,
        _                                                 => TeamRepoError::Unexpected(err.into()),
    }
}

pub fn parse_role(raw: &str) -> Result<TeamRole, TeamRepoError> {
    TeamRole::parse(raw).map_err(|err| TeamRepoError::Unexpected(err.into()))
}
