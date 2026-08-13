# Document MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-document.svg)](https://crates.io/crates/mcp-document)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

The most complete multi-backend document MCP server. **29 tools** across **3 backends** — Google Docs/Drive, Notion, and Microsoft OneDrive/SharePoint. Full CRUD, search, export, comments, and sharing. Single Rust binary with feature-flagged backends and enterprise governance.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-document/main/docs/assets/architecture.svg" alt="MCP Document Architecture" width="800"/>
</p>

## Key Principles

- **Multi-backend** — Google Docs/Drive, Notion, Microsoft OneDrive in one server
- **Feature-flagged** — compile only the backends you need (`--features google,notion,microsoft`)
- **Full document lifecycle** — create, read, update, delete, search, export, comment, share
- **No credential exposure** — tokens stay in env vars, never reach LLM context
- **Single binary** — no Node.js, no Python, no runtime dependencies

## Tools (29)

### Google Docs / Drive (12)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `google_list_docs` | List Google Docs documents | Read-only |
| `google_search_docs` | Search Google Docs by content | Read-only |
| `google_get_doc` | Get document metadata and structure | Read-only |
| `google_get_text` | Get document as plain text | Read-only |
| `google_create_doc` | Create a new Google Doc | Internal write |
| `google_insert_text` | Insert text at a position | Internal write |
| `google_replace_text` | Find and replace text | Internal write |
| `google_export_doc` | Export as text/html/pdf | Read-only |
| `google_list_comments` | List comments on a document | Read-only |
| `google_add_comment` | Add a comment to a document | Internal write |
| `google_share_doc` | Share document with a user | External write |
| `google_delete_doc` | Delete a Google Doc | Destructive |

### Notion (10)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `notion_search` | Search pages and databases | Read-only |
| `notion_list_pages` | List Notion pages | Read-only |
| `notion_get_page` | Get page properties | Read-only |
| `notion_get_content` | Get page content (blocks) | Read-only |
| `notion_create_page` | Create a new page | Internal write |
| `notion_append_blocks` | Append text blocks to a page | Internal write |
| `notion_archive_page` | Archive (soft-delete) a page | Destructive |
| `notion_query_database` | Query a database with filters | Read-only |
| `notion_list_comments` | List comments on a page | Read-only |
| `notion_add_comment` | Add a comment to a page | Internal write |

### Microsoft OneDrive (7)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `ms_list_docs` | List documents in OneDrive | Read-only |
| `ms_search_docs` | Search documents in OneDrive | Read-only |
| `ms_get_content` | Get document content | Read-only |
| `ms_create_doc` | Create a document | Internal write |
| `ms_update_doc` | Update document content | Internal write |
| `ms_delete_doc` | Delete a document | Destructive |
| `ms_share_doc` | Share document with a user | External write |

## Backends

| Backend | Env Vars | Auth Method |
|---------|----------|-------------|
| **Google Docs/Drive** | `GOOGLE_ACCESS_TOKEN` | OAuth2 (access token) |
| **Notion** | `NOTION_API_KEY` | Integration API key |
| **Microsoft OneDrive** | `MS_GRAPH_TOKEN` | OAuth2 (Graph token) |

## Installation

```bash
cargo install mcp-document
```

Or build from source:

```bash
git clone https://github.com/zavora-ai/mcp-document
cd mcp-document
cargo build --release
```

### Feature flags

```bash
# Default: Google + Notion
cargo install mcp-document

# All backends
cargo install mcp-document --features all-backends

# Only Google
cargo install mcp-document --no-default-features --features google

# Only Microsoft
cargo install mcp-document --no-default-features --features microsoft
```

## Configuration Examples

### Google Docs only

```bash
export GOOGLE_ACCESS_TOKEN="ya29.xxxx"
```

### Notion only

```bash
export NOTION_API_KEY="ntn_xxxx"
```

### Microsoft OneDrive only

```bash
export MS_GRAPH_TOKEN="eyJ0eXAi..."
```

### All backends

```bash
export GOOGLE_ACCESS_TOKEN="ya29.xxxx"
export NOTION_API_KEY="ntn_xxxx"
export MS_GRAPH_TOKEN="eyJ0eXAi..."
```

## Client Configuration

### Claude Desktop

```json
{
  "mcpServers": {
    "document": {
      "command": "mcp-document",
      "args": [],
      "env": {
        "GOOGLE_ACCESS_TOKEN": "ya29.xxxx",
        "NOTION_API_KEY": "ntn_xxxx"
      }
    }
  }
}
```

### Kiro

Add to `.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "document": {
      "command": "mcp-document",
      "args": [],
      "env": {
        "GOOGLE_ACCESS_TOKEN": "ya29.xxxx",
        "NOTION_API_KEY": "ntn_xxxx",
        "MS_GRAPH_TOKEN": "eyJ0eXAi..."
      }
    }
  }
}
```

### Cursor

Add to `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "document": {
      "command": "mcp-document",
      "args": [],
      "env": {
        "GOOGLE_ACCESS_TOKEN": "ya29.xxxx",
        "NOTION_API_KEY": "ntn_xxxx"
      }
    }
  }
}
```

### Windsurf

Add to `~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "document": {
      "command": "mcp-document",
      "args": [],
      "env": {
        "NOTION_API_KEY": "ntn_xxxx"
      }
    }
  }
}
```

## Usage Examples

### List and read Google Docs
```
"List my recent Google Docs"
→ calls google_list_docs

"Get the text content of the Q4 planning doc"
→ calls google_search_docs → google_get_text
```

### Create and edit documents
```
"Create a new Google Doc called 'Meeting Notes'"
→ calls google_create_doc

"Add a paragraph about the budget to the meeting notes"
→ calls google_insert_text
```

### Search across Notion
```
"Find all Notion pages about the product roadmap"
→ calls notion_search

"Query the tasks database for items assigned to me"
→ calls notion_query_database
```

### Share and collaborate
```
"Share the design doc with sarah@company.com as a commenter"
→ calls google_share_doc

"Add a comment on the Notion page saying 'Approved'"
→ calls notion_add_comment
```

### Microsoft OneDrive
```
"List my OneDrive documents"
→ calls ms_list_docs

"Create a new document called 'report.md' with the summary"
→ calls ms_create_doc
```

### Export documents
```
"Export the proposal as PDF"
→ calls google_export_doc with format="application/pdf"

"Get the HTML version of the announcement"
→ calls google_export_doc with format="text/html"
```

## OAuth Setup

### Google (one-time)

1. Go to [Google Cloud Console](https://console.cloud.google.com/apis/credentials)
2. Create a **Web application** OAuth client
3. Add `http://localhost:8856` as an authorized redirect URI
4. Enable the **Google Docs API** and **Google Drive API**
5. Use the OAuth flow to obtain an access token:

```bash
# Exchange authorization code for token
curl -X POST https://oauth2.googleapis.com/token \
  -d "code=AUTH_CODE" \
  -d "client_id=YOUR_CLIENT_ID" \
  -d "client_secret=YOUR_CLIENT_SECRET" \
  -d "redirect_uri=http://localhost:8856" \
  -d "grant_type=authorization_code"
```

6. Set the token: `export GOOGLE_ACCESS_TOKEN="ya29.xxxx"`

### Notion

1. Go to [Notion Integrations](https://www.notion.so/my-integrations)
2. Create a new integration
3. Copy the **Internal Integration Secret**
4. Share your pages/databases with the integration
5. Set: `export NOTION_API_KEY="ntn_xxxx"`

### Microsoft

1. Register an app in [Azure Portal](https://portal.azure.com/#blade/Microsoft_AAD_RegisteredApps)
2. Add `Files.ReadWrite.All` permission
3. Use OAuth2 to obtain a Graph API token
4. Set: `export MS_GRAPH_TOKEN="eyJ0eXAi..."`

## Documentation

| Document | Description |
|----------|-------------|
| [API Reference](docs/api-reference.md) | All 29 tools with parameters, types, and examples |
| [Backends](docs/backends.md) | Configuration for all 3 backends with setup guides |
| [Architecture](docs/assets/architecture.svg) | System diagram |
| [mcp-server.toml](mcp-server.toml) | ADK-Rust Enterprise registry manifest |

## Registry Compliance

This server implements the [ADK MCP SDK](https://crates.io/crates/adk-mcp-sdk) contract:

- **HealthCheck** — async health probe for registry monitoring
- **mcp-server.toml** — manifest declaring tools, risk classes, and credentials
- **Structured tracing** — `RUST_LOG` env-filter for observability

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)

## rmcp and MCP compatibility

This server is built with [`rmcp` 3.1.2](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.2) and requires Rust 1.94.1 or newer. The rmcp 3 rollout retains legacy MCP initialization compatibility and targets MCP protocol revisions `2025-11-25` and `2026-07-28`.

## MCP 2026-07-28 rollout (P4 workflow/business)

This server uses `rmcp` 3.1.2 and `adk-mcp-sdk` 0.2 with a minimum supported
Rust version of **1.94.1**. It accepts stateless MCP 2026 requests with
per-request protocol, client identity, and capability metadata while retaining
the legacy MCP 2025-11-25 initialize flow for ordinary tools.

- **Tasks:** `google_export_doc`
- **MRTR approvals:** `google_create_doc`, `google_insert_text`, `google_replace_text`, `google_share_doc`, `google_delete_doc`, `notion_create_page`, `notion_append_blocks`, `notion_archive_page`, `notion_update_page`, `ms_create_doc`, `ms_update_doc`, `ms_delete_doc`, `ms_share_doc`
- **Discovery and routing:** rmcp serves on-demand discovery and validates the
  per-request protocol envelope; HTTP deployments can route with `Mcp-Method`
  and `Mcp-Name`. The packaged binary currently uses stdio.
- **Caching:** `tools/list` returns a public `ttlMs` of 60,000 for MCP 2026;
  rmcp omits the cache fields for legacy clients.
- **Deprecated extensions:** this server does not add new Roots, Sampling, or
  dynamic client-registration dependencies.

Protected tools require `MCP_REQUEST_STATE_KEY` with at least 32 high-entropy
bytes. All replicas must share that key so sealed approval state can resume on
another instance. Approval state is bound to the client identity, tool, and
arguments and expires after two minutes. Missing identity, invalid state,
rejection, or legacy protocol use fails closed. Task records are process-local
for the current stdio runtime; use a durable task store before deploying the
server behind scale-to-zero HTTP infrastructure.
