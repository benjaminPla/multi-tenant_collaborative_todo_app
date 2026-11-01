pub mod pdp;
pub mod ports;

use crate::domain::{
    teams::{TeamId, TeamRole},
    users::UserId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    AddMember,
    CreateBoard,
    CreateTeam,
    CreateTodo,
    CreateUser,
    DeleteBoard,
    DeleteTeam,
    DeleteTodo,
    DeleteUser,
    ListBoards,
    ListMembers,
    ListTeams,
    ListTodos,
    ListUsers,
    ReadBoard,
    ReadCurrentUser,
    ReadTeam,
    ReadTodo,
    ReadUser,
    RemoveMember,
    UpdateBoard,
    UpdateMember,
    UpdateTeam,
    UpdateTodo,
    UpdateUser,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    Deny,
    Permit(Scope),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    Member { team_id: TeamId, user_id: UserId },
    System,
    Team(TeamId),
}

impl Resource {
    pub fn team_id(&self) -> Option<TeamId> {
        match self {
            Self::Member { team_id, .. } | Self::Team(team_id) => Some(*team_id),
            Self::System                                       => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scope {
    All,
    Memberships,
}

#[derive(Clone, Copy, Debug)]
pub struct Subject {
    pub id:        UserId,
    pub is_admin:  bool,
    pub team_role: Option<TeamRole>,
}
