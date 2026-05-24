//! Google Docs + Drive backend
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DocInfo {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author: Option<String>,
    pub content: String,
    pub resolved: bool,
}

#[derive(Clone)]
pub struct GoogleBackend {
    http: reqwest::Client,
    token: String,
}

impl GoogleBackend {
    pub fn new(token: String) -> Self {
        Self { http: reqwest::Client::new(), token }
    }

    pub async fn list_documents(&self, max: u32) -> Result<Vec<DocInfo>> {
        let resp: serde_json::Value = self.http
            .get("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(&self.token)
            .query(&[("q", "mimeType='application/vnd.google-apps.document'"), ("fields", "files(id,name,webViewLink)"), ("pageSize", &max.to_string())])
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["files"].as_array().map(|a| a.iter().map(|f| DocInfo {
            id: f["id"].as_str().unwrap_or("").into(),
            title: f["name"].as_str().unwrap_or("").into(),
            url: f["webViewLink"].as_str().map(String::from),
        }).collect()).unwrap_or_default())
    }

    pub async fn search_documents(&self, query: &str, max: u32) -> Result<Vec<DocInfo>> {
        let escaped = query.replace('\\', "\\\\").replace('\'', "\\'");
        let q = format!("mimeType='application/vnd.google-apps.document' and fullText contains '{escaped}'");
        let resp: serde_json::Value = self.http
            .get("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(&self.token)
            .query(&[("q", q.as_str()), ("fields", "files(id,name,webViewLink)"), ("pageSize", &max.to_string())])
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["files"].as_array().map(|a| a.iter().map(|f| DocInfo {
            id: f["id"].as_str().unwrap_or("").into(),
            title: f["name"].as_str().unwrap_or("").into(),
            url: f["webViewLink"].as_str().map(String::from),
        }).collect()).unwrap_or_default())
    }

    pub async fn get_document(&self, doc_id: &str) -> Result<serde_json::Value> {
        Ok(self.http.get(format!("https://docs.googleapis.com/v1/documents/{doc_id}"))
            .bearer_auth(&self.token).send().await?.error_for_status()?.json().await?)
    }

    pub async fn get_document_text(&self, doc_id: &str) -> Result<String> {
        let resp = self.http.get(format!("https://www.googleapis.com/drive/v3/files/{doc_id}/export"))
            .bearer_auth(&self.token)
            .query(&[("mimeType", "text/plain")])
            .send().await?.error_for_status()?;
        Ok(resp.text().await?)
    }

    pub async fn create_document(&self, title: &str) -> Result<String> {
        let resp: serde_json::Value = self.http
            .post("https://docs.googleapis.com/v1/documents")
            .bearer_auth(&self.token)
            .json(&serde_json::json!({"title": title}))
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["documentId"].as_str().unwrap_or("").to_string())
    }

    pub async fn insert_text(&self, doc_id: &str, text: &str, index: u32) -> Result<()> {
        let body = serde_json::json!({"requests": [{"insertText": {"location": {"index": index}, "text": text}}]});
        self.http.post(format!("https://docs.googleapis.com/v1/documents/{doc_id}:batchUpdate"))
            .bearer_auth(&self.token).json(&body).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn replace_text(&self, doc_id: &str, find: &str, replace: &str) -> Result<()> {
        let body = serde_json::json!({"requests": [{"replaceAllText": {"containsText": {"text": find, "matchCase": true}, "replaceText": replace}}]});
        self.http.post(format!("https://docs.googleapis.com/v1/documents/{doc_id}:batchUpdate"))
            .bearer_auth(&self.token).json(&body).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn export_document(&self, doc_id: &str, mime_type: &str) -> Result<Vec<u8>> {
        let resp = self.http.get(format!("https://www.googleapis.com/drive/v3/files/{doc_id}/export"))
            .bearer_auth(&self.token)
            .query(&[("mimeType", mime_type)])
            .send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
    }

    pub async fn list_comments(&self, doc_id: &str) -> Result<Vec<Comment>> {
        let resp: serde_json::Value = self.http
            .get(format!("https://www.googleapis.com/drive/v3/files/{doc_id}/comments"))
            .bearer_auth(&self.token)
            .query(&[("fields", "comments(id,author/displayName,content,resolved)")])
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["comments"].as_array().map(|a| a.iter().map(|c| Comment {
            id: c["id"].as_str().unwrap_or("").into(),
            author: c["author"]["displayName"].as_str().map(String::from),
            content: c["content"].as_str().unwrap_or("").into(),
            resolved: c["resolved"].as_bool().unwrap_or(false),
        }).collect()).unwrap_or_default())
    }

    pub async fn add_comment(&self, doc_id: &str, content: &str) -> Result<String> {
        let resp: serde_json::Value = self.http
            .post(format!("https://www.googleapis.com/drive/v3/files/{doc_id}/comments"))
            .bearer_auth(&self.token)
            .query(&[("fields", "id")])
            .json(&serde_json::json!({"content": content}))
            .send().await?.error_for_status()?.json().await?;
        Ok(resp["id"].as_str().unwrap_or("").to_string())
    }

    pub async fn share_document(&self, doc_id: &str, email: &str, role: &str) -> Result<()> {
        self.http.post(format!("https://www.googleapis.com/drive/v3/files/{doc_id}/permissions"))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({"type": "user", "role": role, "emailAddress": email}))
            .send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn delete_document(&self, doc_id: &str) -> Result<()> {
        self.http.delete(format!("https://www.googleapis.com/drive/v3/files/{doc_id}"))
            .bearer_auth(&self.token).send().await?.error_for_status()?;
        Ok(())
    }
}
