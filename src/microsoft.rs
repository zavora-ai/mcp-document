//! Microsoft OneDrive/SharePoint backend via Graph API
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MsDocument {
    pub id: String,
    pub name: String,
    pub web_url: Option<String>,
}

#[derive(Clone)]
pub struct MicrosoftBackend {
    http: reqwest::Client,
    token: String,
}

impl MicrosoftBackend {
    pub fn new(token: String) -> Self {
        Self { http: reqwest::Client::new(), token }
    }

    pub async fn list_documents(&self, max: u32) -> Result<Vec<MsDocument>> {
        let resp: serde_json::Value = self.http
            .get("https://graph.microsoft.com/v1.0/me/drive/root/children")
            .bearer_auth(&self.token)
            .query(&[("$top", max.to_string()), ("$filter", "file ne null".to_string())])
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["value"].as_array().map(|a| a.iter().map(|f| MsDocument {
            id: f["id"].as_str().unwrap_or("").into(),
            name: f["name"].as_str().unwrap_or("").into(),
            web_url: f["webUrl"].as_str().map(String::from),
        }).collect()).unwrap_or_default())
    }

    pub async fn search_documents(&self, query: &str) -> Result<Vec<MsDocument>> {
        let resp: serde_json::Value = self.http
            .get(format!("https://graph.microsoft.com/v1.0/me/drive/root/search(q='{query}')"))
            .bearer_auth(&self.token)
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["value"].as_array().map(|a| a.iter().map(|f| MsDocument {
            id: f["id"].as_str().unwrap_or("").into(),
            name: f["name"].as_str().unwrap_or("").into(),
            web_url: f["webUrl"].as_str().map(String::from),
        }).collect()).unwrap_or_default())
    }

    pub async fn get_document_content(&self, item_id: &str) -> Result<String> {
        let resp = self.http
            .get(format!("https://graph.microsoft.com/v1.0/me/drive/items/{item_id}/content"))
            .bearer_auth(&self.token)
            .send().await?.error_for_status()?;
        Ok(resp.text().await?)
    }

    pub async fn create_document(&self, name: &str, content: &str) -> Result<String> {
        let resp: serde_json::Value = self.http
            .put(format!("https://graph.microsoft.com/v1.0/me/drive/root:/{name}:/content"))
            .bearer_auth(&self.token)
            .header("Content-Type", "text/plain")
            .body(content.to_string())
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["id"].as_str().unwrap_or("").to_string())
    }

    pub async fn update_document(&self, item_id: &str, content: &str) -> Result<()> {
        self.http.put(format!("https://graph.microsoft.com/v1.0/me/drive/items/{item_id}/content"))
            .bearer_auth(&self.token)
            .header("Content-Type", "text/plain")
            .body(content.to_string())
            .send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn delete_document(&self, item_id: &str) -> Result<()> {
        self.http.delete(format!("https://graph.microsoft.com/v1.0/me/drive/items/{item_id}"))
            .bearer_auth(&self.token).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn share_document(&self, item_id: &str, email: &str, role: &str) -> Result<String> {
        let resp: serde_json::Value = self.http
            .post(format!("https://graph.microsoft.com/v1.0/me/drive/items/{item_id}/invite"))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({
                "recipients": [{"email": email}],
                "roles": [role],
                "requireSignIn": true
            }))
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["value"][0]["id"].as_str().unwrap_or("shared").to_string())
    }
}
