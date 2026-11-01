use super::PgTeamRepo;
use crate::{
    domain::{
        teams::{
            ports::{TeamRepo, TeamRepoError},
            TeamRole,
        },
        users::Email,
    },
    infrastructure::test_support::{acme, alice, app_pool},
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn adding_an_existing_member_conflicts(pool: PgPoolOptions, connect: PgConnectOptions) {
    let repo  = PgTeamRepo::new(app_pool(pool, connect).await);
    let email = Email::parse("bob@acme.test").unwrap();
    assert!(matches!(repo.add_member(acme(), &email, TeamRole::Member).await, Err(TeamRepoError::AlreadyMember)));
}

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn adding_an_unknown_email_is_not_found(pool: PgPoolOptions, connect: PgConnectOptions) {
    let repo  = PgTeamRepo::new(app_pool(pool, connect).await);
    let email = Email::parse("nobody@plaza.test").unwrap();
    assert!(matches!(repo.add_member(acme(), &email, TeamRole::Member).await, Err(TeamRepoError::UserNotFound)));
}

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn last_owner_cannot_leave(pool: PgPoolOptions, connect: PgConnectOptions) {
    let repo = PgTeamRepo::new(app_pool(pool, connect).await);
    assert!(matches!(repo.remove_member(acme(), alice()).await, Err(TeamRepoError::LastOwner)));
    assert!(matches!(repo.update_member_role(acme(), alice(), TeamRole::Member).await, Err(TeamRepoError::LastOwner)));
}
