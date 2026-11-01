use crate::domain::{authz::Resource, teams::TeamId, users::UserId};
use axum::extract::RawPathParams;
use uuid::Uuid;

pub type Loader = fn(&RawPathParams) -> Option<Resource>;

pub fn member(params: &RawPathParams) -> Option<Resource> {
    let team_id = TeamId::from(uuid(params, "id")?);
    let user_id = UserId::from(uuid(params, "user_id")?);
    Some(Resource::Member { team_id, user_id })
}

pub fn system(_params: &RawPathParams) -> Option<Resource> {
    Some(Resource::System)
}

pub fn team(params: &RawPathParams) -> Option<Resource> {
    Some(Resource::Team(TeamId::from(uuid(params, "id")?)))
}

fn uuid(params: &RawPathParams, name: &str) -> Option<Uuid> {
    params.iter().find(|(key, _)| *key == name).and_then(|(_, value)| value.parse().ok())
}
