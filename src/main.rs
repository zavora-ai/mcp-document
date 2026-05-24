mod google;
mod notion;
mod microsoft;
mod server;

use google::GoogleBackend;
use notion::NotionBackend;
use microsoft::MicrosoftBackend;
use rmcp::{ServiceExt, transport::stdio};
use server::DocumentServer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("info".parse()?)).init();

    let google = std::env::var("GOOGLE_DOCS_TOKEN")
        .or_else(|_| std::env::var("GOOGLE_ACCESS_TOKEN"))
        .ok()
        .map(|t| { tracing::info!("Google Docs backend enabled"); Arc::new(GoogleBackend::new(t)) });

    let notion = std::env::var("NOTION_API_KEY")
        .ok()
        .map(|k| { tracing::info!("Notion backend enabled"); Arc::new(NotionBackend::new(k)) });

    let microsoft = std::env::var("MS_GRAPH_TOKEN")
        .ok()
        .map(|t| { tracing::info!("Microsoft OneDrive backend enabled"); Arc::new(MicrosoftBackend::new(t)) });

    if google.is_none() && notion.is_none() && microsoft.is_none() {
        anyhow::bail!("No backend configured. Set GOOGLE_DOCS_TOKEN, NOTION_API_KEY, or MS_GRAPH_TOKEN");
    }

    let server = DocumentServer { google, notion, microsoft };
    tracing::info!("mcp-document starting on stdio");
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
