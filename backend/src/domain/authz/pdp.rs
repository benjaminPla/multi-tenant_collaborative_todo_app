use crate::domain::{
    authz::{Action, Decision, Resource, Scope, Subject},
    teams::TeamRole,
};

pub fn decide(subject: &Subject, action: Action, resource: &Resource) -> Decision {
    if subject.is_admin {
        return Decision::Permit(Scope::All);
    }

    let is_member = subject.team_role.is_some();
    let is_owner  = subject.team_role == Some(TeamRole::Owner);
    let is_self   = matches!(resource, Resource::Member { user_id, .. } if *user_id == subject.id);

    let permitted = match action {
        Action::ListTeams                                            => return Decision::Permit(Scope::Memberships),
        Action::CreateTeam | Action::ReadCurrentUser                 => true,
        Action::CreateBoard | Action::ListBoards | Action::ListMembers
        | Action::ReadBoard | Action::ReadTeam | Action::UpdateBoard => is_member,
        Action::RemoveMember                                         => is_owner || (is_member && is_self),
        Action::AddMember | Action::DeleteBoard | Action::DeleteTeam
        | Action::UpdateMember | Action::UpdateTeam                  => is_owner,
        Action::CreateUser | Action::DeleteUser
        | Action::ListUsers | Action::ReadUser | Action::UpdateUser  => false,
    };

    match permitted {
        true  => Decision::Permit(Scope::All),
        false => Decision::Deny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{teams::TeamId, users::UserId};

    const ALL: [Action; 20] = [
        Action::AddMember,
        Action::CreateBoard,
        Action::CreateTeam,
        Action::CreateUser,
        Action::DeleteBoard,
        Action::DeleteTeam,
        Action::DeleteUser,
        Action::ListBoards,
        Action::ListMembers,
        Action::ListTeams,
        Action::ListUsers,
        Action::ReadBoard,
        Action::ReadCurrentUser,
        Action::ReadTeam,
        Action::ReadUser,
        Action::RemoveMember,
        Action::UpdateBoard,
        Action::UpdateMember,
        Action::UpdateTeam,
        Action::UpdateUser,
    ];

    const PERMIT: Decision = Decision::Permit(Scope::All);

    fn subject(is_admin: bool, team_role: Option<TeamRole>) -> Subject {
        Subject { id: UserId::generate(), is_admin, team_role }
    }

    #[test]
    fn admin_is_permitted_everything_unscoped() {
        let admin = subject(true, None);
        for action in ALL {
            assert_eq!(decide(&admin, action, &Resource::Team(TeamId::generate())), PERMIT, "{action:?}");
        }
    }

    #[test]
    fn list_teams_is_scoped_to_memberships() {
        let user = subject(false, None);
        assert_eq!(decide(&user, Action::ListTeams, &Resource::System), Decision::Permit(Scope::Memberships));
    }

    #[test]
    fn member_reads_but_does_not_manage() {
        let member = subject(false, Some(TeamRole::Member));
        let team   = Resource::Team(TeamId::generate());
        for action in [Action::CreateBoard, Action::ListBoards, Action::ListMembers, Action::ReadBoard, Action::ReadTeam, Action::UpdateBoard] {
            assert_eq!(decide(&member, action, &team), PERMIT, "{action:?}");
        }
        for action in [Action::AddMember, Action::DeleteBoard, Action::DeleteTeam, Action::UpdateTeam] {
            assert_eq!(decide(&member, action, &team), Decision::Deny, "{action:?}");
        }
    }

    #[test]
    fn member_removes_only_self() {
        let member  = subject(false, Some(TeamRole::Member));
        let team_id = TeamId::generate();
        let own     = Resource::Member { team_id, user_id: member.id };
        let other   = Resource::Member { team_id, user_id: UserId::generate() };
        assert_eq!(decide(&member, Action::RemoveMember, &own), PERMIT);
        assert_eq!(decide(&member, Action::RemoveMember, &other), Decision::Deny);
    }

    #[test]
    fn non_member_is_denied_team_actions() {
        let outsider = subject(false, None);
        let team     = Resource::Team(TeamId::generate());
        for action in [Action::ListBoards, Action::ListMembers, Action::ReadBoard, Action::ReadTeam, Action::UpdateTeam] {
            assert_eq!(decide(&outsider, action, &team), Decision::Deny, "{action:?}");
        }
        assert_eq!(decide(&outsider, Action::CreateTeam, &Resource::System), PERMIT);
        assert_eq!(decide(&outsider, Action::ReadCurrentUser, &Resource::System), PERMIT);
    }

    #[test]
    fn owner_manages_team() {
        let owner = subject(false, Some(TeamRole::Owner));
        let team  = Resource::Team(TeamId::generate());
        for action in [Action::AddMember, Action::DeleteBoard, Action::DeleteTeam, Action::UpdateMember, Action::UpdateTeam] {
            assert_eq!(decide(&owner, action, &team), PERMIT, "{action:?}");
        }
    }

    #[test]
    fn user_management_is_admin_only() {
        let owner = subject(false, Some(TeamRole::Owner));
        for action in [Action::CreateUser, Action::DeleteUser, Action::ListUsers, Action::ReadUser, Action::UpdateUser] {
            assert_eq!(decide(&owner, action, &Resource::System), Decision::Deny, "{action:?}");
        }
    }
}
