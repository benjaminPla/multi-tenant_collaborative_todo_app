use crate::domain::{boards::BoardId, teams::TeamId, todos::TodoId, users::UserId};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use uuid::uuid;

// seed ids from db/seed.sql
pub fn acme() -> TeamId {
    TeamId::from(uuid!("20000000-0000-0000-0000-000000000001"))
}

pub fn alice() -> UserId {
    UserId::from(uuid!("10000000-0000-0000-0000-000000000001"))
}

pub fn bob() -> UserId {
    UserId::from(uuid!("10000000-0000-0000-0000-000000000002"))
}

pub fn carol() -> UserId {
    UserId::from(uuid!("10000000-0000-0000-0000-000000000003"))
}

pub fn deploy_to_production() -> TodoId {
    TodoId::from(uuid!("40000000-0000-0000-0000-000000000001"))
}

pub fn globex() -> TeamId {
    TeamId::from(uuid!("20000000-0000-0000-0000-000000000002"))
}

pub fn globex_secret_plan() -> TodoId {
    TodoId::from(uuid!("40000000-0000-0000-0000-000000000004"))
}

pub fn q4_roadmap() -> BoardId {
    BoardId::from(uuid!("30000000-0000-0000-0000-000000000002"))
}

pub fn website_launch() -> BoardId {
    BoardId::from(uuid!("30000000-0000-0000-0000-000000000001"))
}

// sqlx::test connects as a superuser, which bypasses RLS; the app role doesn't
pub async fn app_pool(pool: PgPoolOptions, connect: PgConnectOptions) -> PgPool {
    pool.connect_with(connect.username("plaza_app").password("plaza_app"))
        .await
        .expect("Failed to connect as plaza_app")
}
