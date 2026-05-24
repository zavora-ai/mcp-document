//! Notion backend
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionPage {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
}

#[derive(Clone)]
pub struct NotionBackend {
    http: reqwest::Client,
    api_key: String,
}

impl NotionBackend {
    pub fn new(api_key: String) -> Self {
        Self { http: reqwest::Client::new(), api_key }
    }

    fn headers(&self) -> Vec<(&str, String)> {
        vec![("Notion-Version", "2022-06-28".into())]
    }

    async fn post(&self, url: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .json(body).send().await?.error_for_status()?.json().await?)
    }

    async fn get(&self, url: &str) -> Result<serde_json::Value> {
        Ok(self.http.get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .send().await?.error_for_status()?.json().await?)
    }

    async fn patch(&self, url: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.patch(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", "2022-06-28")
            .json(body).send().await?.error_for_status()?.json().await?)
    }

    pub async fn search(&self, query: &str, max: u32) -> Result<Vec<NotionPage>> {
        let resp = self.post("https://api.notion.com/v1/search", &serde_json::json!({
            "query": query, "page_size": max
        })).await?;
        Ok(parse_pages(&resp))
    }

    pub async fn list_pages(&self, max: u32) -> Result<Vec<NotionPage>> {
        let resp = self.post("https://api.notion.com/v1/search", &serde_json::json!({
            "filter": {"property": "object", "value": "page"}, "page_size": max
        })).await?;
        Ok(parse_pages(&resp))
    }

    pub async fn get_page(&self, page_id: &str) -> Result<serde_json::Value> {
        self.get(&format!("https://api.notion.com/v1/pages/{page_id}")).await
    }

    pub async fn get_page_content(&self, page_id: &str) -> Result<serde_json::Value> {
        self.get(&format!("https://api.notion.com/v1/blocks/{page_id}/children?page_size=100")).await
    }

    pub async fn create_page(&self, parent_id: &str, title: &str, content: Option<&str>) -> Result<String> {
        let mut body = serde_json::json!({
            "parent": {"page_id": parent_id},
            "properties": {"title": {"title": [{"text": {"content": title}}]}}
        });
        if let Some(text) = content {
            body["children"] = serde_json::json!([{
                "object": "block", "type": "paragraph",
                "paragraph": {"rich_text": [{"type": "text", "text": {"content": text}}]}
            }]);
        }
        let resp = self.post("https://api.notion.com/v1/pages", &body).await?;
        Ok(resp["id"].as_str().unwrap_or("").to_string())
    }

    pub async fn update_page_properties(&self, page_id: &str, properties: &serde_json::Value) -> Result<()> {
        self.patch(&format!("https://api.notion.com/v1/pages/{page_id}"), &serde_json::json!({"properties": properties})).await?;
        Ok(())
    }

    pub async fn archive_page(&self, page_id: &str) -> Result<()> {
        self.patch(&format!("https://api.notion.com/v1/pages/{page_id}"), &serde_json::json!({"archived": true})).await?;
        Ok(())
    }

    pub async fn append_blocks(&self, page_id: &str, markdown: &str) -> Result<()> {
        let blocks: Vec<serde_json::Value> = markdown.lines().map(|line| {
            serde_json::json!({
                "object": "block", "type": "paragraph",
                "paragraph": {"rich_text": [{"type": "text", "text": {"content": line}}]}
            })
        }).collect();
        self.patch(&format!("https://api.notion.com/v1/blocks/{page_id}/children"), &serde_json::json!({"children": blocks})).await?;
        Ok(())
    }

    pub async fn query_database(&self, database_id: &str, filter: Option<&serde_json::Value>) -> Result<serde_json::Value> {
        let body = filter.cloned().unwrap_or_else(|| serde_json::json!({}));
        self.post(&format!("https://api.notion.com/v1/databases/{database_id}/query"), &body).await
    }

    pub async fn list_comments(&self, page_id: &str) -> Result<serde_json::Value> {
        self.get(&format!("https://api.notion.com/v1/comments?block_id={page_id}")).await
    }

    pub async fn add_comment(&self, page_id: &str, text: &str) -> Result<()> {
        self.post("https://api.notion.com/v1/comments", &serde_json::json!({
            "parent": {"page_id": page_id},
            "rich_text": [{"type": "text", "text": {"content": text}}]
        })).await?;
        Ok(())
    }
}

fn parse_pages(resp: &serde_json::Value) -> Vec<NotionPage> {
    resp["results"].as_array().map(|a| a.iter().filter_map(|p| {
        let id = p["id"].as_str()?.to_string();
        let title = p["properties"]["title"]["title"].as_array()
            .or_else(|| p["properties"]["Name"]["title"].as_array())
            .and_then(|t| t.first())
            .and_then(|t| t["plain_text"].as_str())
            .unwrap_or("Untitled").to_string();
        let url = p["url"].as_str().map(String::from);
        Some(NotionPage { id, title, url })
    }).collect()).unwrap_or_default()
}
