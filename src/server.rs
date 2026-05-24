use adk_mcp_sdk::{HealthCheck, HealthStatus};
use crate::client::DocumentClient;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetDocInput { pub document_id: String }

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateDocInput { pub title: String }

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AppendDocInput { pub document_id: String, pub text: String }

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchDocsInput { pub query: String }

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetNotionPageInput { pub page_id: String }

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateNotionPageInput {
    pub parent_id: String,
    pub title: String,
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Clone)]
pub struct DocumentServer { pub client: Arc<DocumentClient> }

#[tool_router(server_handler)]
impl DocumentServer {
    #[tool(description = "List Google Docs documents")]
    async fn list_documents(&self) -> String {
        match self.client.list_documents().await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get a Google Doc by ID")]
    async fn get_document(&self, Parameters(i): Parameters<GetDocInput>) -> String {
        match self.client.get_document(&i.document_id).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Create a new Google Doc")]
    async fn create_document(&self, Parameters(i): Parameters<CreateDocInput>) -> String {
        match self.client.create_document(&i.title).await {
            Ok(id) => format!("Document created (id: {id})"), Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Append text to a Google Doc")]
    async fn append_to_document(&self, Parameters(i): Parameters<AppendDocInput>) -> String {
        match self.client.append_to_document(&i.document_id, &i.text).await {
            Ok(()) => "Text appended".into(), Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Search Google Docs by query")]
    async fn search_documents(&self, Parameters(i): Parameters<SearchDocsInput>) -> String {
        match self.client.search_documents(&i.query).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List Notion pages")]
    async fn list_notion_pages(&self) -> String {
        match self.client.list_notion_pages().await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get a Notion page by ID")]
    async fn get_notion_page(&self, Parameters(i): Parameters<GetNotionPageInput>) -> String {
        match self.client.get_notion_page(&i.page_id).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Create a Notion page")]
    async fn create_notion_page(&self, Parameters(i): Parameters<CreateNotionPageInput>) -> String {
        match self.client.create_notion_page(&i.parent_id, &i.title, i.content.as_deref()).await {
            Ok(id) => format!("Notion page created (id: {id})"), Err(e) => format!("Error: {e}"),
        }
    }
}

#[async_trait::async_trait]
impl HealthCheck for DocumentServer {
    async fn check_health(&self) -> HealthStatus {
        HealthStatus { healthy: true, message: Some("operational".into()), latency_ms: Some(1) }
    }
}
