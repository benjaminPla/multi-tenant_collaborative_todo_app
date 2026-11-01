use crate::domain::teams::TeamId;
use sqlx::{PgPool, Postgres, Transaction};

pub async fn begin(pool: &PgPool, team_id: TeamId) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.team_id', $1, true)")
        .bind(team_id.as_uuid().to_string())
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}
