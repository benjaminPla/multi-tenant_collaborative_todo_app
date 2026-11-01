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
            UpdateTodoInput,
            UpdateTodoUseCase,
        },
    },
    interface::{
        app_state::AppState,
        todos::dto::{CreateTodoRequest, TodoResponse, UpdateTodoRequest},
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn create(
    State(state):              State<AppState>,
    Path((team_id, board_id)): Path<(Uuid, Uuid)>,
    Json(body):                Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<TodoResponse>), TodoAppError> {
    let use_case = CreateTodoUseCase::new(state.todo_repo);
    let todo     = use_case
        .execute(CreateTodoInput {
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
    State(state):                  State<AppState>,
    Path((team_id, board_id, id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, TodoAppError> {
    let use_case = DeleteTodoUseCase::new(state.todo_repo);
    use_case.execute(DeleteTodoInput { board_id, id, team_id }).await?;
    Ok(StatusCode::NO_CONTENT)
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
    State(state):                  State<AppState>,
    Path((team_id, board_id, id)): Path<(Uuid, Uuid, Uuid)>,
    Json(body):                    Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, TodoAppError> {
    let use_case = UpdateTodoUseCase::new(state.todo_repo);
    let todo     = use_case
        .execute(UpdateTodoInput {
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
