use crate::{
    application::teams::{
        errors::TeamAppError,
        use_cases::{
            CreateTeamInput,
            CreateTeamUseCase,
            DeleteTeamInput,
            DeleteTeamUseCase,
            GetTeamInput,
            GetTeamUseCase,
            ListTeamsInput,
            ListTeamsUseCase,
            UpdateTeamInput,
            UpdateTeamUseCase,
        },
    },
    interface::{
        app_state::AppState,
        auth::extractors::CurrentUser,
        teams::dto::{CreateTeamRequest, TeamResponse, UpdateTeamRequest},
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn create(
    CurrentUser(actor): CurrentUser,
    State(state):       State<AppState>,
    Json(body):         Json<CreateTeamRequest>,
) -> Result<(StatusCode, Json<TeamResponse>), TeamAppError> {
    let use_case = CreateTeamUseCase::new(state.team_repo);
    let team     = use_case.execute(CreateTeamInput { actor, name: body.name }).await?;
    Ok((StatusCode::CREATED, Json(team.into())))
}

pub async fn delete(
    CurrentUser(actor): CurrentUser,
    State(state):       State<AppState>,
    Path(id):           Path<Uuid>,
) -> Result<StatusCode, TeamAppError> {
    let use_case = DeleteTeamUseCase::new(state.team_repo);
    use_case.execute(DeleteTeamInput { actor, id }).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    CurrentUser(actor): CurrentUser,
    State(state):       State<AppState>,
    Path(id):           Path<Uuid>,
) -> Result<Json<TeamResponse>, TeamAppError> {
    let use_case = GetTeamUseCase::new(state.team_repo);
    let team     = use_case.execute(GetTeamInput { actor, id }).await?;
    Ok(Json(team.into()))
}

pub async fn list(
    CurrentUser(actor): CurrentUser,
    State(state):       State<AppState>,
) -> Result<Json<Vec<TeamResponse>>, TeamAppError> {
    let use_case = ListTeamsUseCase::new(state.team_repo);
    let teams    = use_case.execute(ListTeamsInput { actor }).await?;
    Ok(Json(teams.into_iter().map(TeamResponse::from).collect()))
}

pub async fn update(
    CurrentUser(actor): CurrentUser,
    State(state):       State<AppState>,
    Path(id):           Path<Uuid>,
    Json(body):         Json<UpdateTeamRequest>,
) -> Result<Json<TeamResponse>, TeamAppError> {
    let use_case = UpdateTeamUseCase::new(state.team_repo);
    let team     = use_case.execute(UpdateTeamInput { actor, id, name: body.name }).await?;
    Ok(Json(team.into()))
}
