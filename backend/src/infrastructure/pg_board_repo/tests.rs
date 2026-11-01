use super::PgBoardRepo;
use crate::{
    domain::boards::ports::BoardRepo,
    infrastructure::{
        tenant,
        test_support::{acme, app_pool},
    },
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn lists_only_the_teams_boards(pool: PgPoolOptions, connect: PgConnectOptions) {
    let repo   = PgBoardRepo::new(app_pool(pool, connect).await);
    let boards = repo.list(acme()).await.unwrap();
    let names  = boards.iter().map(|board| board.name.as_str()).collect::<Vec<_>>();
    assert_eq!(names, ["Website Launch"]);
}

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn untrimmed_names_violate_the_schema(pool: PgPoolOptions, connect: PgConnectOptions) {
    let pool   = app_pool(pool, connect).await;
    let mut tx = tenant::begin(&pool, acme()).await.unwrap();
    let err    = sqlx::query("INSERT INTO boards (name, team_id) VALUES (' Launch ', $1)")
        .bind(acme().as_uuid())
        .execute(&mut *tx)
        .await
        .unwrap_err();
    assert_eq!(err.as_database_error().and_then(|db_err| db_err.constraint()), Some("boards_name_trimmed"));
}
