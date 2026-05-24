# API Reference

All 29 tools provided by `mcp-document`.

---

## Google Docs / Drive (12 tools)

### `google_list_docs`

List Google Docs documents.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `limit` | u32 | No | 20 | Maximum documents to return |

### `google_search_docs`

Search Google Docs by content.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | String | Yes | — | Full-text search query |
| `limit` | u32 | No | 20 | Maximum results |

### `google_get_doc`

Get Google Doc metadata and structure.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |

### `google_get_text`

Get Google Doc as plain text.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |

### `google_create_doc`

Create a new Google Doc.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `title` | String | Yes | — | Document title |

### `google_insert_text`

Insert text into a Google Doc at a position.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |
| `text` | String | Yes | — | Text to insert |
| `index` | u32 | No | 1 | Character index position |

### `google_replace_text`

Find and replace text in a Google Doc.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |
| `find` | String | Yes | — | Text to find (case-sensitive) |
| `replace` | String | Yes | — | Replacement text |

### `google_export_doc`

Export Google Doc as text/plain, text/html, or application/pdf.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |
| `format` | String | No | "text/plain" | MIME type: `text/plain`, `text/html`, or `application/pdf` |

### `google_list_comments`

List comments on a Google Doc.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |

### `google_add_comment`

Add a comment to a Google Doc.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |
| `content` | String | Yes | — | Comment text |

### `google_share_doc`

Share a Google Doc with a user.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |
| `email` | String | Yes | — | Email address to share with |
| `role` | String | No | "reader" | Permission: `reader`, `writer`, or `commenter` |

### `google_delete_doc`

Delete a Google Doc.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `doc_id` | String | Yes | — | Google Doc ID |

---

## Notion (10 tools)

### `notion_search`

Search Notion pages and databases.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | String | Yes | — | Search query |
| `limit` | u32 | No | 20 | Maximum results |

### `notion_list_pages`

List Notion pages.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `limit` | u32 | No | 20 | Maximum pages to return |

### `notion_get_page`

Get a Notion page properties.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page_id` | String | Yes | — | Notion page ID |

### `notion_get_content`

Get Notion page content (blocks).

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page_id` | String | Yes | — | Notion page ID |

### `notion_create_page`

Create a Notion page.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `parent_id` | String | Yes | — | Parent page ID |
| `title` | String | Yes | — | Page title |
| `content` | String | No | null | Optional initial paragraph text |

### `notion_append_blocks`

Append text blocks to a Notion page.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page_id` | String | Yes | — | Notion page ID |
| `markdown` | String | Yes | — | Text content (each line becomes a paragraph block) |

### `notion_archive_page`

Archive (soft-delete) a Notion page.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page_id` | String | Yes | — | Notion page ID |

### `notion_query_database`

Query a Notion database with optional filter.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `database_id` | String | Yes | — | Notion database ID |
| `filter` | JSON | No | null | Notion filter object (see [Notion API docs](https://developers.notion.com/reference/post-database-query-filter)) |

### `notion_list_comments`

List comments on a Notion page.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page_id` | String | Yes | — | Notion page ID |

### `notion_add_comment`

Add a comment to a Notion page.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page_id` | String | Yes | — | Notion page ID |
| `text` | String | Yes | — | Comment text |

---

## Microsoft OneDrive (7 tools)

### `ms_list_docs`

List documents in OneDrive.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `limit` | u32 | No | 20 | Maximum documents to return |

### `ms_search_docs`

Search documents in OneDrive.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | String | Yes | — | Search query |

### `ms_get_content`

Get document content from OneDrive.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `item_id` | String | Yes | — | OneDrive item ID |

### `ms_create_doc`

Create a document in OneDrive.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `name` | String | Yes | — | File name (e.g., `report.md`) |
| `content` | String | Yes | — | File content |

### `ms_update_doc`

Update document content in OneDrive.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `item_id` | String | Yes | — | OneDrive item ID |
| `content` | String | Yes | — | New file content |

### `ms_delete_doc`

Delete a document from OneDrive.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `item_id` | String | Yes | — | OneDrive item ID |

### `ms_share_doc`

Share a OneDrive document with a user.

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `item_id` | String | Yes | — | OneDrive item ID |
| `email` | String | Yes | — | Email address to share with |
| `role` | String | No | "reader" | Permission: `reader` or `writer` |
