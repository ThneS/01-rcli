use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
async fn handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, HeaderMap, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            HeaderMap::new(),
            format!("File {} note found", p.display()),
        );
    }
    match tokio::fs::metadata(&p).await {
        Ok(metadata) => match metadata.is_dir() {
            true => dir_handler(Path(p.to_string_lossy().to_string())).await,
            false => file_handler(Path(p.to_string_lossy().to_string())).await,
        },
        Err(e) => {
            warn!("Error reading file: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                e.to_string(),
            )
        }
    }
}
async fn file_handler(Path(path): Path<String>) -> (StatusCode, HeaderMap, String) {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => {
            info!("Read {} bytes", content.len());
            (StatusCode::OK, HeaderMap::new(), content)
        }
        Err(e) => {
            warn!("Error reading file: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                e.to_string(),
            )
        }
    }
}
async fn dir_handler(Path(path): Path<String>) -> (StatusCode, HeaderMap, String) {
    info!("Reading directory {:?}", path);
    match tokio::fs::read_dir(path).await {
        Ok(mut entries) => {
            let mut content = String::new();
            content.push_str("<html><body><ul>");
            while let Some(entry) = entries.next_entry().await.unwrap() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                let path = entry.path();
                let path = path;
                let path = path.to_string_lossy();
                content.push_str(&format!(r#"<li><a href="/{}">{}</a></li>"#, path, name));
            }
            content.push_str("</ul></body></html>");
            // 设置返回的content-type
            let mut header = HeaderMap::new();
            header.insert("content-type", "text/html".parse().unwrap());
            (StatusCode::OK, header, content)
        }
        Err(e) => {
            warn!("Error reading directory: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                e.to_string(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, _, content) = handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
