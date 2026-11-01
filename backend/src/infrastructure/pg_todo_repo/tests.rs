use super::PgTodoRepo;
use crate::{
    domain::todos::ports::{TodoRepo, TodoRepoError},
    infrastructure::test_support::{
        acme,
        app_pool,
        carol,
        deploy_to_production,
        globex,
        globex_secret_plan,
        q4_roadmap,
        website_launch,
    },
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn assignee_must_be_a_team_member(pool: PgPoolOptions, connect: PgConnectOptions) {
    let repo     = PgTodoRepo::new(app_pool(pool, connect).await);
    let mut todo = repo.find_by_id(acme(), website_launch(), deploy_to_production()).await.unwrap().unwrap();
    todo.assignee_id = Some(carol());
    assert!(matches!(repo.update(&todo).await, Err(TodoRepoError::AssigneeNotMember)));
}

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn other_teams_todos_are_invisible(pool: PgPoolOptions, connect: PgConnectOptions) {
    let repo = PgTodoRepo::new(app_pool(pool, connect).await);
    assert!(repo.find_by_id(globex(), q4_roadmap(), globex_secret_plan()).await.unwrap().is_some());
    assert!(repo.find_by_id(acme(), q4_roadmap(), globex_secret_plan()).await.unwrap().is_none());
    assert!(repo.list(acme(), q4_roadmap()).await.unwrap().is_empty());
}

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn stale_version_conflicts(pool: PgPoolOptions, connect: PgConnectOptions) {
    let repo    = PgTodoRepo::new(app_pool(pool, connect).await);
    let todo    = repo.find_by_id(acme(), website_launch(), deploy_to_production()).await.unwrap().unwrap();
    let updated = repo.update(&todo).await.unwrap();
    assert_eq!(updated.version, todo.version + 1);
    assert!(matches!(repo.update(&todo).await, Err(TodoRepoError::VersionConflict)));
}
