use crate::{
    application::teams::{
        errors::TeamAppError,
        use_cases::{
            AddMemberInput,
            AddMemberUseCase,
            CreateTeamInput,
            CreateTeamUseCase,
            DeleteTeamInput,
            DeleteTeamUseCase,
            GetTeamInput,
            GetTeamUseCase,
            ListMembersInput,
            ListMembersUseCase,
            ListTeamsInput,
            ListTeamsUseCase,
            RemoveMemberInput,
            RemoveMemberUseCase,
            UpdateMemberInput,
            UpdateMemberUseCase,
            UpdateTeamInput,
            UpdateTeamUseCase,
        },
    },
    interface::{
        app_state::AppState,
        authz::authorized::Authorized,
        teams::dto::{
            AddMemberRequest,
            CreateTeamRequest,
            MemberResponse,
            TeamResponse,
            UpdateMemberRequest,
            UpdateTeamRequest,
        },
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn add_member(
    State(state):  State<AppState>,
    Path(team_id): Path<Uuid>,
    Json(body):    Json<AddMemberRequest>,
) -> Result<(StatusCode, Json<MemberResponse>), TeamAppError> {
    let use_case = AddMemberUseCase::new(state.team_repo);
    let member   = use_case.execute(AddMemberInput { email: body.email, role: body.role, team_id }).await?;
    Ok((StatusCode::CREATED, Json(member.into())))
}

pub async fn create(
    Authorized { actor, .. }: Authorized,
    State(state):             State<AppState>,
    Json(body):               Json<CreateTeamRequest>,
) -> Result<(StatusCode, Json<TeamResponse>), TeamAppError> {
    let use_case = CreateTeamUseCase::new(state.team_repo);
    let team     = use_case.execute(CreateTeamInput { actor, name: body.name }).await?;
    Ok((StatusCode::CREATED, Json(team.into())))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id):     Path<Uuid>,
) -> Result<StatusCode, TeamAppError> {
    let use_case = DeleteTeamUseCase::new(state.team_repo);
    use_case.execute(DeleteTeamInput { id }).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    State(state): State<AppState>,
    Path(id):     Path<Uuid>,
) -> Result<Json<TeamResponse>, TeamAppError> {
    let use_case = GetTeamUseCase::new(state.team_repo);
    let team     = use_case.execute(GetTeamInput { id }).await?;
    Ok(Json(team.into()))
}

pub async fn list(
    Authorized { actor, scope }: Authorized,
    State(state):                State<AppState>,
) -> Result<Json<Vec<TeamResponse>>, TeamAppError> {
    let use_case = ListTeamsUseCase::new(state.team_repo);
    let teams    = use_case.execute(ListTeamsInput { actor, scope }).await?;
    Ok(Json(teams.into_iter().map(TeamResponse::from).collect()))
}

pub async fn list_members(
    State(state):  State<AppState>,
    Path(team_id): Path<Uuid>,
) -> Result<Json<Vec<MemberResponse>>, TeamAppError> {
    let use_case = ListMembersUseCase::new(state.team_repo);
    let members  = use_case.execute(ListMembersInput { team_id }).await?;
    Ok(Json(members.into_iter().map(MemberResponse::from).collect()))
}

pub async fn remove_member(
    State(state):             State<AppState>,
    Path((team_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, TeamAppError> {
    let use_case = RemoveMemberUseCase::new(state.team_repo);
    use_case.execute(RemoveMemberInput { team_id, user_id }).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update(
    State(state): State<AppState>,
    Path(id):     Path<Uuid>,
    Json(body):   Json<UpdateTeamRequest>,
) -> Result<Json<TeamResponse>, TeamAppError> {
    let use_case = UpdateTeamUseCase::new(state.team_repo);
    let team     = use_case.execute(UpdateTeamInput { id, name: body.name }).await?;
    Ok(Json(team.into()))
}

pub async fn update_member(
    State(state):             State<AppState>,
    Path((team_id, user_id)): Path<(Uuid, Uuid)>,
    Json(body):               Json<UpdateMemberRequest>,
) -> Result<Json<MemberResponse>, TeamAppError> {
    let use_case = UpdateMemberUseCase::new(state.team_repo);
    let member   = use_case.execute(UpdateMemberInput { role: body.role, team_id, user_id }).await?;
    Ok(Json(member.into()))
}
