use super::PgPip;
use crate::{
    domain::{authz::ports::Pip, teams::TeamRole},
    infrastructure::test_support::{acme, alice, app_pool, bob, carol},
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test(fixtures("../../../../db/seed.sql"))]
#[ignore = "needs postgres"]
async fn reads_team_roles(pool: PgPoolOptions, connect: PgConnectOptions) {
    let pip = PgPip::new(app_pool(pool, connect).await);
    assert_eq!(pip.team_role(acme(), alice()).await.unwrap(), Some(TeamRole::Owner));
    assert_eq!(pip.team_role(acme(), bob()).await.unwrap(), Some(TeamRole::Member));
    assert_eq!(pip.team_role(acme(), carol()).await.unwrap(), None);
}
