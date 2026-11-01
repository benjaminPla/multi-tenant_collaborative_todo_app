use crate::domain::teams::{Member, Team};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AddMemberRequest {
    pub email: String,
    pub role:  Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct MemberResponse {
    pub created_at: DateTime<Utc>,
    pub email:      String,
    pub role:       &'static str,
    pub user_id:    Uuid,
}

impl From<Member> for MemberResponse {
    fn from(member: Member) -> Self {
        Self {
            created_at: member.created_at,
            email:      member.email.as_str().to_owned(),
            role:       member.role.as_str(),
            user_id:    member.user_id.as_uuid(),
        }
    }
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
pub struct UpdateMemberRequest {
    pub role: String,
}

#[derive(Deserialize)]
pub struct UpdateTeamRequest {
    pub name: String,
}
