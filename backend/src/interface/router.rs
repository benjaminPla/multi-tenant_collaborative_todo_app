use crate::interface::{
    app_state::AppState,
    authn::router::authn_router,
    authz::pep::pep,
    teams::router::teams_router,
    users::router::users_router,
};
use axum::{middleware::from_fn_with_state, Router};

pub fn app(state: AppState) -> Router {
    Router::new()
        .nest("/auth", authn_router())
        .nest("/teams", teams_router())
        .nest("/users", users_router())
        .route_layer(from_fn_with_state(state.clone(), pep))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::{
        argon2_hasher_service::Argon2HasherService,
        pg_pip::PgPip,
        pg_session_repo::PgSessionRepo,
        pg_team_repo::PgTeamRepo,
        pg_user_repo::PgUserRepo,
    };
    use axum::routing::get;
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    const ID: &str = "00000000-0000-0000-0000-000000000001";

    fn state() -> AppState {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://unused").unwrap();
        AppState {
            hasher_service: Arc::new(Argon2HasherService::new()),
            pip:            Arc::new(PgPip::new(pool.clone())),
            session_repo:   Arc::new(PgSessionRepo::new(pool.clone())),
            session_ttl:    chrono::Duration::hours(1),
            team_repo:      Arc::new(PgTeamRepo::new(pool.clone())),
            user_repo:      Arc::new(PgUserRepo::new(pool)),
        }
    }

    async fn status(app: Router, method: &str, path: &str) -> u16 {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address  = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let mut stream   = tokio::net::TcpStream::connect(address).await.unwrap();
        let request      = format!("{method} {path} HTTP/1.1\r\nhost: test\r\ncontent-length: 0\r\nconnection: close\r\n\r\n");
        let mut response = String::new();
        stream.write_all(request.as_bytes()).await.unwrap();
        stream.read_to_string(&mut response).await.unwrap();
        response[9..12].parse().unwrap()
    }

    #[tokio::test]
    async fn guarded_routes_require_a_session() {
        let app = app(state());
        for (method, path) in [
            ("GET",    "/auth/me".to_owned()),
            ("GET",    "/teams".to_owned()),
            ("POST",   "/teams".to_owned()),
            ("DELETE", format!("/teams/{ID}")),
            ("GET",    format!("/teams/{ID}")),
            ("PATCH",  format!("/teams/{ID}")),
            ("GET",    format!("/teams/{ID}/members")),
            ("POST",   format!("/teams/{ID}/members")),
            ("DELETE", format!("/teams/{ID}/members/{ID}")),
            ("PATCH",  format!("/teams/{ID}/members/{ID}")),
            ("GET",    "/users".to_owned()),
            ("POST",   "/users".to_owned()),
            ("DELETE", format!("/users/{ID}")),
            ("GET",    format!("/users/{ID}")),
            ("PATCH",  format!("/users/{ID}")),
        ] {
            assert_eq!(status(app.clone(), method, &path).await, 401, "{method} {path}");
        }
    }

    #[tokio::test]
    async fn public_routes_skip_authn() {
        assert_eq!(status(app(state()), "POST", "/auth/logout").await, 204);
    }

    #[tokio::test]
    async fn unregistered_routes_are_denied() {
        let state = state();
        let app   = Router::new()
            .route("/unregistered", get(|| async {}))
            .route_layer(from_fn_with_state(state.clone(), pep))
            .with_state(state);
        assert_eq!(status(app, "GET", "/unregistered").await, 403);
    }
}
