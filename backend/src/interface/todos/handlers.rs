use crate::{
    application::todos::{
        errors::TodoAppError,
        use_cases::{
            CreateTodoInput,
            CreateTodoUseCase,
            DeleteTodoInput,
            DeleteTodoUseCase,
            GetTodoInput,
            GetTodoUseCase,
            ListTodosInput,
            ListTodosUseCase,
            SubscribeTodosInput,
            SubscribeTodosUseCase,
            UpdateTodoInput,
            UpdateTodoUseCase,
        },
    },
    interface::{
        app_state::AppState,
        authz::authorized::Authorized,
        todos::{
            dto::{CreateTodoRequest, TodoResponse, UpdateTodoRequest},
            stream,
        },
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

pub async fn create(
    Authorized { actor, .. }:  Authorized,
    State(state):              State<AppState>,
    Path((team_id, board_id)): Path<(Uuid, Uuid)>,
    Json(body):                Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<TodoResponse>), TodoAppError> {
    let use_case = CreateTodoUseCase::new(state.todo_events, state.todo_repo);
    let todo     = use_case
        .execute(CreateTodoInput {
            actor,
            assignee_id: body.assignee_id,
            board_id,
            description: body.description,
            status:      body.status,
            team_id,
            title:       body.title,
        })
        .await?;
    Ok((StatusCode::CREATED, Json(todo.into())))
}

pub async fn delete(
    Authorized { actor, .. }:      Authorized,
    State(state):                  State<AppState>,
    Path((team_id, board_id, id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, TodoAppError> {
    let use_case = DeleteTodoUseCase::new(state.todo_events, state.todo_repo);
    use_case.execute(DeleteTodoInput { actor, board_id, id, team_id }).await?;
    Ok(StatusCode::NO_CONTENT)
}

// proxies like nginx buffer responses by default, which would hold events back
pub async fn events(
    State(state):              State<AppState>,
    Path((team_id, board_id)): Path<(Uuid, Uuid)>,
) -> Result<Response, TodoAppError> {
    let Ok(permit) = state.sse_permits.clone().try_acquire_owned() else {
        return Ok((StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "error": "too many live connections" }))).into_response());
    };
    let use_case = SubscribeTodosUseCase::new(state.board_repo, state.todo_events);
    let feed     = use_case.execute(SubscribeTodosInput { board_id, team_id }).await?;
    Ok(([("x-accel-buffering", "no")], stream::sse(feed, permit, state.shutdown)).into_response())
}

pub async fn get(
    State(state):                  State<AppState>,
    Path((team_id, board_id, id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<TodoResponse>, TodoAppError> {
    let use_case = GetTodoUseCase::new(state.todo_repo);
    let todo     = use_case.execute(GetTodoInput { board_id, id, team_id }).await?;
    Ok(Json(todo.into()))
}

pub async fn list(
    State(state):              State<AppState>,
    Path((team_id, board_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<TodoResponse>>, TodoAppError> {
    let use_case = ListTodosUseCase::new(state.board_repo, state.todo_repo);
    let todos    = use_case.execute(ListTodosInput { board_id, team_id }).await?;
    Ok(Json(todos.into_iter().map(TodoResponse::from).collect()))
}

pub async fn update(
    Authorized { actor, .. }:      Authorized,
    State(state):                  State<AppState>,
    Path((team_id, board_id, id)): Path<(Uuid, Uuid, Uuid)>,
    Json(body):                    Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, TodoAppError> {
    let use_case = UpdateTodoUseCase::new(state.todo_events, state.todo_repo);
    let todo     = use_case
        .execute(UpdateTodoInput {
            actor,
            assignee_id: body.assignee_id,
            board_id,
            description: body.description,
            id,
            status:      body.status,
            team_id,
            title:       body.title,
            version:     body.version,
        })
        .await?;
    Ok(Json(todo.into()))
}
