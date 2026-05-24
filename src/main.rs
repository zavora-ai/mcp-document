mod client;
mod server;

use client::DocumentClient;
use rmcp::{ServiceExt, transport::stdio};
use server::DocumentServer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("info".parse()?)).init();
    let google_token = std::env::var("GOOGLE_DOCS_TOKEN").ok();
    let notion_key = std::env::var("NOTION_API_KEY").ok();
    if google_token.is_none() && notion_key.is_none() {
        panic!("GOOGLE_DOCS_TOKEN or NOTION_API_KEY required");
    }
    let client = Arc::new(DocumentClient::new(google_token, notion_key));
    let server = DocumentServer { client };
    tracing::info!("mcp-document starting on stdio");
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
