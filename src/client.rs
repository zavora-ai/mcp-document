use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct DocumentClient {
    http: Client,
    google_token: Option<String>,
    notion_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
}

impl DocumentClient {
    pub fn new(google_token: Option<String>, notion_key: Option<String>) -> Self {
        Self { http: Client::new(), google_token, notion_key }
    }

    pub async fn list_documents(&self) -> anyhow::Result<Vec<Document>> {
        let token = self.google_token.as_ref().ok_or_else(|| anyhow::anyhow!("GOOGLE_DOCS_TOKEN not set"))?;
        let resp: serde_json::Value = self.http
            .get("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(token)
            .query(&[("q", "mimeType='application/vnd.google-apps.document'"), ("fields", "files(id,name)")])
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["files"].as_array()
            .map(|a| a.iter().map(|f| Document { id: f["id"].as_str().unwrap_or("").into(), title: f["name"].as_str().unwrap_or("").into() }).collect())
            .unwrap_or_default())
    }

    pub async fn get_document(&self, doc_id: &str) -> anyhow::Result<serde_json::Value> {
        let token = self.google_token.as_ref().ok_or_else(|| anyhow::anyhow!("GOOGLE_DOCS_TOKEN not set"))?;
        let resp: serde_json::Value = self.http
            .get(format!("https://docs.googleapis.com/v1/documents/{doc_id}"))
            .bearer_auth(token)
            .send().await?.error_for_status()?.json().await?;
        Ok(resp)
    }

    pub async fn create_document(&self, title: &str) -> anyhow::Result<String> {
        let token = self.google_token.as_ref().ok_or_else(|| anyhow::anyhow!("GOOGLE_DOCS_TOKEN not set"))?;
        let resp: serde_json::Value = self.http
            .post("https://docs.googleapis.com/v1/documents")
            .bearer_auth(token)
            .json(&serde_json::json!({"title": title}))
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["documentId"].as_str().unwrap_or("").to_string())
    }

    pub async fn append_to_document(&self, doc_id: &str, text: &str) -> anyhow::Result<()> {
        let token = self.google_token.as_ref().ok_or_else(|| anyhow::anyhow!("GOOGLE_DOCS_TOKEN not set"))?;
        let body = serde_json::json!({
            "requests": [{"insertText": {"location": {"index": 1}, "text": text}}]
        });
        self.http.post(format!("https://docs.googleapis.com/v1/documents/{doc_id}:batchUpdate"))
            .bearer_auth(token).json(&body)
            .send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn search_documents(&self, query: &str) -> anyhow::Result<Vec<Document>> {
        let token = self.google_token.as_ref().ok_or_else(|| anyhow::anyhow!("GOOGLE_DOCS_TOKEN not set"))?;
        let q = format!("mimeType='application/vnd.google-apps.document' and fullText contains '{query}'");
        let resp: serde_json::Value = self.http
            .get("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(token)
            .query(&[("q", q.as_str()), ("fields", "files(id,name)")])
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["files"].as_array()
            .map(|a| a.iter().map(|f| Document { id: f["id"].as_str().unwrap_or("").into(), title: f["name"].as_str().unwrap_or("").into() }).collect())
            .unwrap_or_default())
    }

    // Notion methods
    pub async fn list_notion_pages(&self) -> anyhow::Result<serde_json::Value> {
        let key = self.notion_key.as_ref().ok_or_else(|| anyhow::anyhow!("NOTION_API_KEY not set"))?;
        let resp: serde_json::Value = self.http
            .post("https://api.notion.com/v1/search")
            .header("Authorization", format!("Bearer {key}"))
            .header("Notion-Version", "2022-06-28")
            .json(&serde_json::json!({"filter": {"property": "object", "value": "page"}}))
            .send().await?.error_for_status()?.json().await?;
        Ok(resp)
    }

    pub async fn get_notion_page(&self, page_id: &str) -> anyhow::Result<serde_json::Value> {
        let key = self.notion_key.as_ref().ok_or_else(|| anyhow::anyhow!("NOTION_API_KEY not set"))?;
        let resp: serde_json::Value = self.http
            .get(format!("https://api.notion.com/v1/pages/{page_id}"))
            .header("Authorization", format!("Bearer {key}"))
            .header("Notion-Version", "2022-06-28")
            .send().await?.error_for_status()?.json().await?;
        Ok(resp)
    }

    pub async fn create_notion_page(&self, parent_id: &str, title: &str, content: Option<&str>) -> anyhow::Result<String> {
        let key = self.notion_key.as_ref().ok_or_else(|| anyhow::anyhow!("NOTION_API_KEY not set"))?;
        let mut body = serde_json::json!({
            "parent": {"page_id": parent_id},
            "properties": {"title": [{"text": {"content": title}}]}
        });
        if let Some(text) = content {
            body["children"] = serde_json::json!([{
                "object": "block", "type": "paragraph",
                "paragraph": {"rich_text": [{"type": "text", "text": {"content": text}}]}
            }]);
        }
        let resp: serde_json::Value = self.http
            .post("https://api.notion.com/v1/pages")
            .header("Authorization", format!("Bearer {key}"))
            .header("Notion-Version", "2022-06-28")
            .json(&body)
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["id"].as_str().unwrap_or("").to_string())
    }
}
