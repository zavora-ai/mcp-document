use adk_mcp_sdk::{HealthCheck, HealthStatus};
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::Deserialize;
use std::sync::Arc;
use crate::google::GoogleBackend;
use crate::notion::NotionBackend;
use crate::microsoft::MicrosoftBackend;

// ─── Input types ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListDocsInput { #[serde(default = "d20")] pub limit: u32 }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchInput { pub query: String, #[serde(default = "d20")] pub limit: u32 }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DocIdInput { pub doc_id: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateDocInput { pub title: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InsertTextInput { pub doc_id: String, pub text: String, #[serde(default = "d1")] pub index: u32 }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReplaceTextInput { pub doc_id: String, pub find: String, pub replace: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExportInput { pub doc_id: String, /// "text/plain", "text/html", or "application/pdf"
    #[serde(default = "d_text")] pub format: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CommentInput { pub doc_id: String, pub content: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ShareInput { pub doc_id: String, pub email: String, /// "reader", "writer", or "commenter"
    #[serde(default = "d_reader")] pub role: String }
// Notion inputs
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NotionPageIdInput { pub page_id: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateNotionPageInput { pub parent_id: String, pub title: String, #[serde(default)] pub content: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AppendBlocksInput { pub page_id: String, pub markdown: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct QueryDatabaseInput { pub database_id: String, #[serde(default)] pub filter: Option<serde_json::Value> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NotionCommentInput { pub page_id: String, pub text: String }
// Microsoft inputs
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MsCreateInput { pub name: String, pub content: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MsUpdateInput { pub item_id: String, pub content: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MsItemInput { pub item_id: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MsShareInput { pub item_id: String, pub email: String, #[serde(default = "d_reader")] pub role: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MsSearchInput { pub query: String }

fn d20() -> u32 { 20 }
fn d1() -> u32 { 1 }
fn d_text() -> String { "text/plain".into() }
fn d_reader() -> String { "reader".into() }

// ─── Server ──────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct DocumentServer {
    pub google: Option<Arc<GoogleBackend>>,
    pub notion: Option<Arc<NotionBackend>>,
    pub microsoft: Option<Arc<MicrosoftBackend>>,
}

#[tool_router(server_handler)]
impl DocumentServer {
    // ─── Google Docs ─────────────────────────────────────────────────────────
    #[tool(description = "List Google Docs documents")]
    async fn google_list_docs(&self, Parameters(i): Parameters<ListDocsInput>) -> String {
        match &self.google { Some(g) => match g.list_documents(i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Search Google Docs by content")]
    async fn google_search_docs(&self, Parameters(i): Parameters<SearchInput>) -> String {
        match &self.google { Some(g) => match g.search_documents(&i.query, i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Get Google Doc metadata and structure")]
    async fn google_get_doc(&self, Parameters(i): Parameters<DocIdInput>) -> String {
        match &self.google { Some(g) => match g.get_document(&i.doc_id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Get Google Doc as plain text")]
    async fn google_get_text(&self, Parameters(i): Parameters<DocIdInput>) -> String {
        match &self.google { Some(g) => match g.get_document_text(&i.doc_id).await { Ok(v) => v, Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Create a new Google Doc")]
    async fn google_create_doc(&self, Parameters(i): Parameters<CreateDocInput>) -> String {
        match &self.google { Some(g) => match g.create_document(&i.title).await { Ok(id) => format!("Created: {id}"), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Insert text into a Google Doc at a position")]
    async fn google_insert_text(&self, Parameters(i): Parameters<InsertTextInput>) -> String {
        match &self.google { Some(g) => match g.insert_text(&i.doc_id, &i.text, i.index).await { Ok(()) => "Text inserted".into(), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Find and replace text in a Google Doc")]
    async fn google_replace_text(&self, Parameters(i): Parameters<ReplaceTextInput>) -> String {
        match &self.google { Some(g) => match g.replace_text(&i.doc_id, &i.find, &i.replace).await { Ok(()) => "Replaced".into(), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Export Google Doc as text/plain, text/html, or application/pdf")]
    async fn google_export_doc(&self, Parameters(i): Parameters<ExportInput>) -> String {
        match &self.google { Some(g) => match g.export_document(&i.doc_id, &i.format).await { Ok(bytes) => { if i.format.contains("text") { String::from_utf8_lossy(&bytes).to_string() } else { format!("Exported {} bytes (base64: {}...)", bytes.len(), &base64_encode(&bytes[..bytes.len().min(100)]) ) } }, Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "List comments on a Google Doc")]
    async fn google_list_comments(&self, Parameters(i): Parameters<DocIdInput>) -> String {
        match &self.google { Some(g) => match g.list_comments(&i.doc_id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Add a comment to a Google Doc")]
    async fn google_add_comment(&self, Parameters(i): Parameters<CommentInput>) -> String {
        match &self.google { Some(g) => match g.add_comment(&i.doc_id, &i.content).await { Ok(id) => format!("Comment added: {id}"), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Share a Google Doc with a user")]
    async fn google_share_doc(&self, Parameters(i): Parameters<ShareInput>) -> String {
        match &self.google { Some(g) => match g.share_document(&i.doc_id, &i.email, &i.role).await { Ok(()) => "Shared".into(), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }
    #[tool(description = "Delete a Google Doc")]
    async fn google_delete_doc(&self, Parameters(i): Parameters<DocIdInput>) -> String {
        match &self.google { Some(g) => match g.delete_document(&i.doc_id).await { Ok(()) => "Deleted".into(), Err(e) => format!("Error: {e}") }, None => "Google backend not configured".into() }
    }

    // ─── Notion ──────────────────────────────────────────────────────────────
    #[tool(description = "Search Notion pages and databases")]
    async fn notion_search(&self, Parameters(i): Parameters<SearchInput>) -> String {
        match &self.notion { Some(n) => match n.search(&i.query, i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "List Notion pages")]
    async fn notion_list_pages(&self, Parameters(i): Parameters<ListDocsInput>) -> String {
        match &self.notion { Some(n) => match n.list_pages(i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "Get a Notion page properties")]
    async fn notion_get_page(&self, Parameters(i): Parameters<NotionPageIdInput>) -> String {
        match &self.notion { Some(n) => match n.get_page(&i.page_id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "Get Notion page content (blocks)")]
    async fn notion_get_content(&self, Parameters(i): Parameters<NotionPageIdInput>) -> String {
        match &self.notion { Some(n) => match n.get_page_content(&i.page_id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "Create a Notion page")]
    async fn notion_create_page(&self, Parameters(i): Parameters<CreateNotionPageInput>) -> String {
        match &self.notion { Some(n) => match n.create_page(&i.parent_id, &i.title, i.content.as_deref()).await { Ok(id) => format!("Created: {id}"), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "Append text blocks to a Notion page")]
    async fn notion_append_blocks(&self, Parameters(i): Parameters<AppendBlocksInput>) -> String {
        match &self.notion { Some(n) => match n.append_blocks(&i.page_id, &i.markdown).await { Ok(()) => "Blocks appended".into(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "Archive (soft-delete) a Notion page")]
    async fn notion_archive_page(&self, Parameters(i): Parameters<NotionPageIdInput>) -> String {
        match &self.notion { Some(n) => match n.archive_page(&i.page_id).await { Ok(()) => "Archived".into(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "Query a Notion database with optional filter")]
    async fn notion_query_database(&self, Parameters(i): Parameters<QueryDatabaseInput>) -> String {
        match &self.notion { Some(n) => match n.query_database(&i.database_id, i.filter.as_ref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "List comments on a Notion page")]
    async fn notion_list_comments(&self, Parameters(i): Parameters<NotionPageIdInput>) -> String {
        match &self.notion { Some(n) => match n.list_comments(&i.page_id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }
    #[tool(description = "Add a comment to a Notion page")]
    async fn notion_add_comment(&self, Parameters(i): Parameters<NotionCommentInput>) -> String {
        match &self.notion { Some(n) => match n.add_comment(&i.page_id, &i.text).await { Ok(()) => "Comment added".into(), Err(e) => format!("Error: {e}") }, None => "Notion backend not configured".into() }
    }

    // ─── Microsoft OneDrive/SharePoint ───────────────────────────────────────
    #[tool(description = "List documents in OneDrive")]
    async fn ms_list_docs(&self, Parameters(i): Parameters<ListDocsInput>) -> String {
        match &self.microsoft { Some(m) => match m.list_documents(i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Microsoft backend not configured".into() }
    }
    #[tool(description = "Search documents in OneDrive")]
    async fn ms_search_docs(&self, Parameters(i): Parameters<MsSearchInput>) -> String {
        match &self.microsoft { Some(m) => match m.search_documents(&i.query).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }, None => "Microsoft backend not configured".into() }
    }
    #[tool(description = "Get document content from OneDrive")]
    async fn ms_get_content(&self, Parameters(i): Parameters<MsItemInput>) -> String {
        match &self.microsoft { Some(m) => match m.get_document_content(&i.item_id).await { Ok(v) => v, Err(e) => format!("Error: {e}") }, None => "Microsoft backend not configured".into() }
    }
    #[tool(description = "Create a document in OneDrive")]
    async fn ms_create_doc(&self, Parameters(i): Parameters<MsCreateInput>) -> String {
        match &self.microsoft { Some(m) => match m.create_document(&i.name, &i.content).await { Ok(id) => format!("Created: {id}"), Err(e) => format!("Error: {e}") }, None => "Microsoft backend not configured".into() }
    }
    #[tool(description = "Update document content in OneDrive")]
    async fn ms_update_doc(&self, Parameters(i): Parameters<MsUpdateInput>) -> String {
        match &self.microsoft { Some(m) => match m.update_document(&i.item_id, &i.content).await { Ok(()) => "Updated".into(), Err(e) => format!("Error: {e}") }, None => "Microsoft backend not configured".into() }
    }
    #[tool(description = "Delete a document from OneDrive")]
    async fn ms_delete_doc(&self, Parameters(i): Parameters<MsItemInput>) -> String {
        match &self.microsoft { Some(m) => match m.delete_document(&i.item_id).await { Ok(()) => "Deleted".into(), Err(e) => format!("Error: {e}") }, None => "Microsoft backend not configured".into() }
    }
    #[tool(description = "Share a OneDrive document with a user")]
    async fn ms_share_doc(&self, Parameters(i): Parameters<MsShareInput>) -> String {
        match &self.microsoft { Some(m) => match m.share_document(&i.item_id, &i.email, &i.role).await { Ok(id) => format!("Shared: {id}"), Err(e) => format!("Error: {e}") }, None => "Microsoft backend not configured".into() }
    }
}

#[async_trait::async_trait]
impl HealthCheck for DocumentServer {
    async fn check_health(&self) -> HealthStatus {
        HealthStatus { healthy: true, message: Some("operational".into()), latency_ms: Some(1) }
    }
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    data.chunks(3).map(|c| {
        let t = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        let mut s = String::new();
        s.push(CHARS[((t >> 18) & 0x3F) as usize] as char);
        s.push(CHARS[((t >> 12) & 0x3F) as usize] as char);
        if c.len() > 1 { s.push(CHARS[((t >> 6) & 0x3F) as usize] as char); }
        if c.len() > 2 { s.push(CHARS[(t & 0x3F) as usize] as char); }
        s
    }).collect()
}
