use narratics_backend_sync::{app_state::AppState, create_router};
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "narratics_backend_sync=info,tower_http=info".into()),
        )
        .init();

    let projects_dir = std::env::var("NARRATICS_PROJECTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/home/cycorld/projects/narratics/data/projects"));

    let project_path = std::env::var("NARRATICS_CONTAINER_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| projects_dir.join("moonlight_chronicles.narr"));

    info!(
        "Initializing Narratics AppState with projects_dir: {:?}, initial_file: {:?}",
        projects_dir, project_path
    );
    let app_state = AppState::new(projects_dir, project_path)?;

    let app = create_router(app_state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3901);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Narratics Studio Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
