pub mod errors;
pub mod ports;
pub mod value_objects;

use crate::domain::users::{Email, UserId};
use chrono::{DateTime, Utc};
pub use errors::TeamError;
pub use value_objects::{TeamId, TeamName, TeamRole};

#[derive(Clone, Debug)]
pub struct Member {
    pub created_at: DateTime<Utc>,
    pub email:      Email,
    pub role:       TeamRole,
    pub user_id:    UserId,
}

#[derive(Clone, Debug)]
pub struct Team {
    pub created_at: DateTime<Utc>,
    pub id:         TeamId,
    pub name:       TeamName,
}

impl Team {
    pub fn create(name: TeamName) -> Self {
        Self { created_at: Utc::now(), id: TeamId::generate(), name }
    }
}

pub fn keeps_an_owner(current: TeamRole, next: Option<TeamRole>, owner_count: i64) -> bool {
    current != TeamRole::Owner || next == Some(TeamRole::Owner) || owner_count > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_owner_cannot_leave_or_be_demoted() {
        assert!(!keeps_an_owner(TeamRole::Owner, None, 1));
        assert!(!keeps_an_owner(TeamRole::Owner, Some(TeamRole::Member), 1));
    }

    #[test]
    fn other_changes_keep_an_owner() {
        assert!(keeps_an_owner(TeamRole::Member, None, 1));
        assert!(keeps_an_owner(TeamRole::Owner, None, 2));
        assert!(keeps_an_owner(TeamRole::Owner, Some(TeamRole::Member), 2));
        assert!(keeps_an_owner(TeamRole::Owner, Some(TeamRole::Owner), 1));
    }
}
