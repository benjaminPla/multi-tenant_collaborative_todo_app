use crate::domain::boards::Board;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct BoardResponse {
    pub created_at: DateTime<Utc>,
    pub id:         Uuid,
    pub name:       String,
    pub team_id:    Uuid,
}

impl From<Board> for BoardResponse {
    fn from(board: Board) -> Self {
        Self {
            created_at: board.created_at,
            id:         board.id.as_uuid(),
            name:       board.name.as_str().to_owned(),
            team_id:    board.team_id.as_uuid(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateBoardRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateBoardRequest {
    pub name: String,
}
