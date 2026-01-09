use axum::{
    extract::State,
    response::sse::{Event as SseEvent, Sse},
    routing::get,
    Router,
};
use futures_util::stream::Stream;
use serde::Serialize;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize)]
pub struct DashboardEvent {
    pub timestamp: String,
    pub event_type: String,
    pub paths: Vec<String>,
}

pub struct ServerState {
    pub tx: broadcast::Sender<DashboardEvent>,
}

use axum::response::Html;
use tower_http::services::ServeDir;

pub async fn start(state: Arc<ServerState>) {
    let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/events", get(sse_handler))
        .fallback_service(ServeDir::new("assets"))
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8081").await {
        Ok(l) => l,
        Err(e) => {
            error!(
                "Failed to bind Web Dashboard to port 8081: {}. Is it already in use?",
                e
            );
            return;
        }
    };
    info!("Web Dashboard available at http://localhost:8081");
    if let Err(e) = axum::serve(listener, app).await {
        error!("Web Dashboard server error: {:?}", e);
    }
}

async fn dashboard_handler() -> Html<String> {
    let content = std::fs::read_to_string("assets/index.html")
        .unwrap_or_else(|_| "Dashboard UI not found".to_string());
    Html(content)
}

async fn sse_handler(
    State(state): State<Arc<ServerState>>,
) -> Sse<impl Stream<Item = Result<SseEvent, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(event) => {
            let data = serde_json::to_string(&event).unwrap();
            Some(Ok(SseEvent::default().data(data)))
        }
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}
