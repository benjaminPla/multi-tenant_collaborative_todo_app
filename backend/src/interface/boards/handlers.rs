use crate::{
    application::boards::{
        errors::BoardAppError,
        use_cases::{
            CreateBoardInput,
            CreateBoardUseCase,
            DeleteBoardInput,
            DeleteBoardUseCase,
            GetBoardInput,
            GetBoardUseCase,
            ListBoardsInput,
            ListBoardsUseCase,
            UpdateBoardInput,
            UpdateBoardUseCase,
        },
    },
    interface::{
        app_state::AppState,
        boards::dto::{BoardResponse, CreateBoardRequest, UpdateBoardRequest},
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn create(
    State(state):  State<AppState>,
    Path(team_id): Path<Uuid>,
    Json(body):    Json<CreateBoardRequest>,
) -> Result<(StatusCode, Json<BoardResponse>), BoardAppError> {
    let use_case = CreateBoardUseCase::new(state.board_repo);
    let board    = use_case.execute(CreateBoardInput { name: body.name, team_id }).await?;
    Ok((StatusCode::CREATED, Json(board.into())))
}

pub async fn delete(
    State(state):        State<AppState>,
    Path((team_id, id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, BoardAppError> {
    let use_case = DeleteBoardUseCase::new(state.board_repo);
    use_case.execute(DeleteBoardInput { id, team_id }).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    State(state):        State<AppState>,
    Path((team_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<BoardResponse>, BoardAppError> {
    let use_case = GetBoardUseCase::new(state.board_repo);
    let board    = use_case.execute(GetBoardInput { id, team_id }).await?;
    Ok(Json(board.into()))
}

pub async fn list(
    State(state):  State<AppState>,
    Path(team_id): Path<Uuid>,
) -> Result<Json<Vec<BoardResponse>>, BoardAppError> {
    let use_case = ListBoardsUseCase::new(state.board_repo);
    let boards   = use_case.execute(ListBoardsInput { team_id }).await?;
    Ok(Json(boards.into_iter().map(BoardResponse::from).collect()))
}

pub async fn update(
    State(state):        State<AppState>,
    Path((team_id, id)): Path<(Uuid, Uuid)>,
    Json(body):          Json<UpdateBoardRequest>,
) -> Result<Json<BoardResponse>, BoardAppError> {
    let use_case = UpdateBoardUseCase::new(state.board_repo);
    let board    = use_case.execute(UpdateBoardInput { id, name: body.name, team_id }).await?;
    Ok(Json(board.into()))
}
