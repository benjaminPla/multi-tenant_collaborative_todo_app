pub mod errors;
pub mod ports;
pub mod value_objects;

use chrono::{DateTime, Utc};
pub use errors::TeamError;
pub use value_objects::{TeamId, TeamName, TeamRole};

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
