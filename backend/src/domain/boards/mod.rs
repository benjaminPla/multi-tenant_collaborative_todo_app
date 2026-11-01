pub mod errors;
pub mod ports;
pub mod value_objects;

use crate::domain::teams::TeamId;
use chrono::{DateTime, Utc};
pub use errors::BoardError;
pub use value_objects::{BoardId, BoardName};

#[derive(Clone, Debug)]
pub struct Board {
    pub created_at: DateTime<Utc>,
    pub id:         BoardId,
    pub name:       BoardName,
    pub team_id:    TeamId,
}

impl Board {
    pub fn create(team_id: TeamId, name: BoardName) -> Self {
        Self { created_at: Utc::now(), id: BoardId::generate(), name, team_id }
    }
}
