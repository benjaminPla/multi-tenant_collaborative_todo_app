use crate::domain::teams::Team;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct TeamResponse {
    pub created_at: DateTime<Utc>,
    pub id:         Uuid,
    pub name:       String,
}

impl From<Team> for TeamResponse {
    fn from(team: Team) -> Self {
        Self {
            created_at: team.created_at,
            id:         team.id.as_uuid(),
            name:       team.name.as_str().to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateTeamRequest {
    pub name: String,
}
