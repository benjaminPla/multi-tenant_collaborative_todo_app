use crate::domain::{
    teams::{
        ports::TeamRepoError,
        Member,
        Team,
        TeamId,
        TeamName,
        TeamRole,
    },
    users::{Email, UserId},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct MemberRow {
    created_at: DateTime<Utc>,
    email:      String,
    role:       String,
    user_id:    Uuid,
}

impl TryFrom<MemberRow> for Member {
    type Error = TeamRepoError;

    fn try_from(row: MemberRow) -> Result<Self, Self::Error> {
        Ok(Self {
            created_at: row.created_at,
            email:      Email::parse(&row.email).map_err(|err| TeamRepoError::Unexpected(err.into()))?,
            role:       parse_role(&row.role)?,
            user_id:    UserId::from(row.user_id),
        })
    }
}

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
        Some(db_err) if db_err.is_unique_violation()      => TeamRepoError::AlreadyMember,
        Some(db_err) if db_err.is_foreign_key_violation() => TeamRepoError::NotFound,
        _                                                 => TeamRepoError::Unexpected(err.into()),
    }
}

pub fn parse_role(raw: &str) -> Result<TeamRole, TeamRepoError> {
    TeamRole::parse(raw).map_err(|err| TeamRepoError::Unexpected(err.into()))
}
