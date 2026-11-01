use crate::{
    application::users::{
        errors::UserAppError,
        use_cases::{
            CreateUserInput,
            CreateUserUseCase,
            DeleteUserInput,
            DeleteUserUseCase,
            GetUserInput,
            GetUserUseCase,
            ListUsersUseCase,
            UpdateUserInput,
            UpdateUserUseCase,
        },
    },
    interface::{
        app_state::AppState,
        auth::extractors::AdminUser,
        users::dto::{CreateUserRequest, UpdateUserRequest, UserResponse},
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn create(
    _admin:       AdminUser,
    State(state): State<AppState>,
    Json(body):   Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), UserAppError> {
    let use_case = CreateUserUseCase::new(state.hasher_service, state.user_repo);
    let user     = use_case.execute(CreateUserInput { email: body.email, password: body.password }).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn delete(
    _admin:       AdminUser,
    State(state): State<AppState>,
    Path(id):     Path<Uuid>,
) -> Result<StatusCode, UserAppError> {
    let use_case = DeleteUserUseCase::new(state.user_repo);
    use_case.execute(DeleteUserInput { id }).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    _admin:       AdminUser,
    State(state): State<AppState>,
    Path(id):     Path<Uuid>,
) -> Result<Json<UserResponse>, UserAppError> {
    let use_case = GetUserUseCase::new(state.user_repo);
    let user     = use_case.execute(GetUserInput { id }).await?;
    Ok(Json(user.into()))
}

pub async fn list(
    _admin:       AdminUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, UserAppError> {
    let use_case = ListUsersUseCase::new(state.user_repo);
    let users    = use_case.execute().await?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

pub async fn update(
    _admin:       AdminUser,
    State(state): State<AppState>,
    Path(id):     Path<Uuid>,
    Json(body):   Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, UserAppError> {
    let use_case = UpdateUserUseCase::new(state.hasher_service, state.session_repo, state.user_repo);
    let user     = use_case.execute(UpdateUserInput { email: body.email, id, password: body.password }).await?;
    Ok(Json(user.into()))
}
