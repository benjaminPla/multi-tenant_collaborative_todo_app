use crate::{
    domain::authz::Action,
    interface::authz::loaders::{self, Loader},
};
use axum::http::Method;

#[derive(Clone, Copy)]
pub enum Policy {
    Guarded { action: Action, loader: Loader },
    Public,
}

const fn guarded(action: Action, loader: Loader) -> Policy {
    Policy::Guarded { action, loader }
}

const POLICIES: &[(&str, &str, Policy)] = &[
    ("POST",   "/auth/login",                   Policy::Public),
    ("POST",   "/auth/logout",                  Policy::Public),
    ("GET",    "/auth/me",                      guarded(Action::ReadCurrentUser, loaders::system)),
    ("GET",    "/teams",                        guarded(Action::ListTeams,       loaders::system)),
    ("POST",   "/teams",                        guarded(Action::CreateTeam,      loaders::system)),
    ("DELETE", "/teams/{id}",                   guarded(Action::DeleteTeam,      loaders::team)),
    ("GET",    "/teams/{id}",                   guarded(Action::ReadTeam,        loaders::team)),
    ("PATCH",  "/teams/{id}",                   guarded(Action::UpdateTeam,      loaders::team)),
    ("GET",    "/teams/{id}/members",           guarded(Action::ListMembers,     loaders::team)),
    ("POST",   "/teams/{id}/members",           guarded(Action::AddMember,       loaders::team)),
    ("DELETE", "/teams/{id}/members/{user_id}", guarded(Action::RemoveMember,    loaders::member)),
    ("PATCH",  "/teams/{id}/members/{user_id}", guarded(Action::UpdateMember,    loaders::member)),
    ("GET",    "/users",                        guarded(Action::ListUsers,       loaders::system)),
    ("POST",   "/users",                        guarded(Action::CreateUser,      loaders::system)),
    ("DELETE", "/users/{id}",                   guarded(Action::DeleteUser,      loaders::system)),
    ("GET",    "/users/{id}",                   guarded(Action::ReadUser,        loaders::system)),
    ("PATCH",  "/users/{id}",                   guarded(Action::UpdateUser,      loaders::system)),
];

pub fn lookup(method: &Method, path: &str) -> Option<Policy> {
    POLICIES
        .iter()
        .find(|(m, p, _)| *m == method.as_str() && *p == path)
        .map(|(_, _, policy)| *policy)
}
