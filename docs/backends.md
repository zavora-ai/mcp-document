# Backends

mcp-document supports 3 document providers. Each activates independently via env vars. Feature-flagged at compile time.

## Google Docs + Drive

**Env var:** `GOOGLE_DOCS_TOKEN` or `GOOGLE_ACCESS_TOKEN`

**Capabilities:** 12 tools — list, search, get structure, get text, create, insert text, find/replace, export (PDF/HTML/text), comments, share, delete

**Setup:**
1. Create OAuth Web App at https://console.cloud.google.com/apis/credentials
2. Add redirect URI: `http://localhost:8856/callback`
3. Enable Google Docs API and Google Drive API
4. Get token via OAuth flow or use a service account

**Scopes needed:**
- `https://www.googleapis.com/auth/documents`
- `https://www.googleapis.com/auth/drive`

---

## Notion

**Env var:** `NOTION_API_KEY`

**Capabilities:** 10 tools — search, list pages, get page/content, create page, append blocks, archive, query database, comments

**Setup:**
1. Go to https://www.notion.so/my-integrations
2. Create a new integration
3. Copy the "Internal Integration Secret"
4. Share your Notion pages/databases with the integration

---

## Microsoft OneDrive/SharePoint

**Env var:** `MS_GRAPH_TOKEN`

**Capabilities:** 7 tools — list, search, get content, create, update, delete, share

**Setup:**
1. Register app at https://portal.azure.com/#blade/Microsoft_AAD_RegisteredApps
2. Add permissions: `Files.ReadWrite.All`, `Sites.ReadWrite.All`
3. Get token via OAuth or use client credentials flow

---

## Combined Configuration

Use any combination:

```bash
# All three backends
export GOOGLE_DOCS_TOKEN="ya29.xxx"
export NOTION_API_KEY="ntn_xxx"
export MS_GRAPH_TOKEN="eyJ0xxx"

# Just Google + Notion (default features)
export GOOGLE_DOCS_TOKEN="ya29.xxx"
export NOTION_API_KEY="ntn_xxx"

# Just Notion
export NOTION_API_KEY="ntn_xxx"
```

Each backend's tools gracefully return "backend not configured" if the env var is missing.
