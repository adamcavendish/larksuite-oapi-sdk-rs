# Examples

These examples are intentionally curated rather than generated from every
OpenAPI operation. They mirror the most common Go SDK sample patterns while
keeping each Rust example small enough to maintain.

Most examples use this shape:

- read credentials and resource IDs from environment variables
- build a `LarkClient` with `LarkClient::builder`
- create a `RequestOption::default()`
- call a typed service resource such as `client.im().message`
- use `*_by_query` methods when an API has several query parameters

## Core examples

| Example | Service | Scenario | Required env vars | Command |
| --- | --- | --- | --- | --- |
| `client_config` | LarkClient | Build default and tuned clients with Go-SDK-style builder options | optional `APP_ID`, `APP_SECRET` | `cargo run --example client_config` |
| `raw_api` | Raw OpenAPI | Call endpoints through `ApiReq` with path/query params and explicit token mode | `APP_ID`, `APP_SECRET`, `USER_ID`, `USER_ACCESS_TOKEN`, optional `CHAT_ID` | `cargo run --example raw_api` |
| `spark_db_sync` | Spark v1 | Safely preview a Base-to-database sync configuration with a user token | `APP_ID`, `APP_SECRET`, `USER_ACCESS_TOKEN`, `SPARK_APP_ID`, `SPARK_DB_SYNC_CONFIG` | `cargo run --example spark_db_sync` |
| `base_v3_record_read` | Base v3 | Read Base table records with an offset-paginated, user-token request | `APP_ID`, `APP_SECRET`, `USER_ACCESS_TOKEN`, `BASE_V3_BASE_TOKEN`, `BASE_V3_TABLE_ID` | `cargo run --example base_v3_record_read` |
| `base_v3_app_read` | Base v3 | Discover BaseApps in a workspace, then read their pages and optional blocks | `APP_ID`, `APP_SECRET`, `USER_ACCESS_TOKEN`, `BASE_V3_WORKSPACE_TOKEN`, `BASE_V3_APP_TOKEN`, optional `BASE_V3_PAGE_ID` | `cargo run --example base_v3_app_read` |
| `docs_ai_fetch` | Docs AI v1 | Read modern Docx content, including comment metadata when enabled | `APP_ID`, `APP_SECRET`, `USER_ACCESS_TOKEN`, `DOCS_AI_DOCUMENT_ID` | `cargo run --example docs_ai_fetch` |
| `slides_ai_read` | Slides AI v1 | Read XML presentation content with a user access token | `APP_ID`, `APP_SECRET`, `USER_ACCESS_TOKEN`, `SLIDES_AI_PRESENTATION_ID` | `cargo run --example slides_ai_read` |
| `app_registration` | OAuth app registration | Create or update an app through the device-code registration flow | optional `APP_AVATAR_URL`, `CREATE_ONLY`, `EXISTING_APP_ID`, `REGISTRATION_DOMAIN`, `REGISTRATION_LARK_DOMAIN` | `cargo run --example app_registration` |
| `send_message` | IM v1 | Send a text IM message through the generated IM service | `APP_ID`, `APP_SECRET`, `CHAT_ID` | `cargo run --example send_message` |
| `event_handler` | Event dispatcher | Handle encrypted HTTP callback events | none for compilation | `cargo run --example event_handler` |
| `card_action_handler` | Card callbacks | Handle an interactive card callback and return a toast JSON body | none for compilation | `cargo run --example card_action_handler` |
| `ws_client` | WebSocket events | Receive events through WebSocket long connections; optionally attach a trusted user channel | `APP_ID`, `APP_SECRET`, optional `WS_CHANNEL_TAG`, `USER_ACCESS_TOKEN` | `cargo run --features ws --example ws_client` |
| `channel_send` | Channel messaging | Send markdown through channel messaging with an optional idempotency UUID | `APP_ID`, `APP_SECRET`, `CHAT_ID`, optional `MESSAGE_UUID` | `cargo run --features messaging --example channel_send` |
| `channel_normalize` | Channel helpers | Normalize incoming channel messages | `APP_ID`, `APP_SECRET` | `cargo run --features channel --example channel_normalize` |

## Generated service examples

| Example | Service | Scenario | Required env vars | Command |
| --- | --- | --- | --- | --- |
| `authen_oauth` | Authen v1 | Exchange an authorization code, refresh a user token, and fetch user info | `APP_ID`, `APP_SECRET`, `AUTH_CODE`, optional `REDIRECT_URI`, `REFRESH_TOKEN` | `cargo run --example authen_oauth` |
| `im_message_query` | IM v1 | List recent messages in a chat with query parameters | `APP_ID`, `APP_SECRET`, `CHAT_ID`, optional `START_TIME`, `END_TIME`, `PAGE_TOKEN` | `cargo run --example im_message_query` |
| `im_upload_download` | IM v1 | Upload image/file bytes from local paths and download existing image/file keys | `APP_ID`, `APP_SECRET`, optional `IMAGE_PATH`, `FILE_PATH`, `IMAGE_KEY`, `FILE_KEY` | `cargo run --example im_upload_download` |
| `im_stream_download` | IM v1 | Stream a message resource to disk while hashing and enforcing an optional byte limit | `APP_ID`, `APP_SECRET`, `MESSAGE_ID`, `FILE_KEY`, optional `RESOURCE_TYPE`, `OUTPUT_PATH`, `MAX_BYTES` | `cargo run --example im_stream_download` |
| `drive_files` | Drive v1 | List Drive files and optionally download/export files | `APP_ID`, `APP_SECRET`, optional `FOLDER_TOKEN`, `DRIVE_FILE_TOKEN`, `EXPORT_FILE_TOKEN`, `EXPORT_TICKET` | `cargo run --example drive_files` |
| `bitable_records` | Bitable v1 | List tables, views, and records with `PageQuery`; use `*_by_iterator` for lazy scans | `APP_ID`, `APP_SECRET`, `APP_TOKEN`, `TABLE_ID`, optional `VIEW_ID` | `cargo run --example bitable_records` |
| `sheets_read` | Sheets v3 | Fetch spreadsheet metadata and read a range | `APP_ID`, `APP_SECRET`, `SPREADSHEET_TOKEN`, optional `SHEET_RANGE` | `cargo run --example sheets_read` |
| `application_v6` | Application v6 | List and get applications with `lang` and `user_id_type` | `APP_ID`, `APP_SECRET`, optional `APPLICATION_APP_ID` | `cargo run --example application_v6` |
| `app_slash_commands` | Application v7 | List slash commands and optionally create one | `APP_ID`, `APP_SECRET`, optional `SLASH_COMMAND`, `SLASH_COMMAND_DESCRIPTION` | `cargo run --example app_slash_commands` |
| `bot_search` | Bot v4 | Search visible bots with a user access token | `APP_ID`, `APP_SECRET`, `USER_ACCESS_TOKEN`, optional `BOT_QUERY` | `cargo run --example bot_search` |
| `okr_v2` | OKR v2 | List OKR categories with the typed v2 resource | `APP_ID`, `APP_SECRET` | `cargo run --example okr_v2` |
| `calendar_events` | Calendar v4 | List calendars and events with pagination/time filters | `APP_ID`, `APP_SECRET`, `CALENDAR_ID`, optional `START_TIME`, `END_TIME` | `cargo run --example calendar_events` |
| `go_compatibility_endpoint` | Go compatibility bridge | Call a bridged endpoint that has not been promoted to a dedicated Rust resource | `APP_ID`, `APP_SECRET`, `NOTE_ID` | `cargo run --example go_compatibility_endpoint` |

## Mapping from Go SDK samples

Use this table when translating code from the official Go SDK sample tree.

| Go SDK path | Rust example or API | What to copy conceptually |
| --- | --- | --- |
| `sample/client/main.go` | `client_config` | `NewClient(..., With...)` maps to `LarkClient::builder(...).timeout(...).base_url(...).build()` |
| `sample/callrawapi/api.go` | `raw_api` | `larkcore.ApiReq` maps to `ApiReq`; `SupportedAccessTokenTypes` maps to `AccessTokenType` |
| `sample/api/im/im.go` | `send_message`, `im_message_query`, `im_upload_download`, `im_stream_download` | Prefer typed IM resources before raw calls |
| `sample/api/bitable2.go` | `bitable_records` | Use generated resources, `PageQuery`, or `*_by_iterator` for paginated Base records |
| `sample/api/sheets.go` | `sheets_read` | Use generated Sheets resources for metadata and value reads |
| `sample/api/application.go` | `application_v6` | Use versioned accessors such as `client.application_v6()` |
| `sample/event/event.go` | `event_handler` | Register typed event callbacks on `EventDispatcher` |
| `sample/ws/sample.go` | `ws_client` | Build a dispatcher, then start `client.ws_client(dispatcher)` with the `ws` feature |
| `sample/card/card.go` | `card_action_handler`, card builder APIs | Use `CardActionHandler` for callbacks and `larksuite_oapi_sdk_rs::card` for message JSON |
| `sample/channel/main.go` | `channel_send`, `channel_normalize` | Use `messaging` for outbound operations and `channel` for inbound normalization and runtime policy |
| `sample/apiall/botv4`, `sample/apiall/okrv2` | `bot_search`, `okr_v2` | Prefer the dedicated typed resources; retain `go_compatibility` for endpoints not promoted to Rust resources |

## Typed resources vs GoCompatibilityEndpoint

Prefer dedicated typed resources when they exist. For example, use
`client.im().message.list_by_query(...)` for IM messages and
`client.drive().file.list_by_query(...)` for Drive files. These APIs expose
named request body structs, query structs, and typed response data.

Use `client.go_compatibility()` only for newer Go SDK endpoints that are bridged
in this crate but do not yet have a dedicated typed Rust resource. The bridge
preserves SDK token handling while accepting path parameters, query parameters, and an
optional JSON body. Prefer read-only endpoints while learning the bridge, then
move to mutating endpoints once the path and token mode are confirmed.
