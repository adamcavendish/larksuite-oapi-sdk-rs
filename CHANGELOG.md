# Changelog

## Unreleased

### Tooling

- Added a reproducible Go `v3.9.9` service contract catalog for future
  resource-generation work. It records 1,562 request definitions without
  changing runtime dispatch or generated Rust resource implementations.
- Added a deterministic Go-to-Rust request parity report. It distinguishes
  typed and GoV397 bridge coverage from metadata mismatches, missing contracts,
  and unsupported request templates that need explicit extractor support.
- Extended the parity extractor to cover CoreHR, Performance, and Hire local
  request macros. The v3.9.9 baseline now resolves all 1,562 Go request
  contracts without missing, mismatched, or unresolved entries.

### WebSocket

- Added branch-parity trusted user channels with channel tags, gateway query
  parameters, connection controls, and explicit user attach/detach operations.

### REST service coverage

- Aligned the implemented Go service request contracts for multipart uploads
  and tenant/user token capabilities. Document AI and upload resources now use
  `FormDataField` bodies instead of JSON placeholders; the parity baseline has
  no remaining request metadata mismatches.
- Replaced public `serde_json::Value` fields, query bodies, request helpers,
  event and card payloads with the SDK-owned `JsonValue` wrapper. Closed API
  contracts remain Go-shaped models; intentionally dynamic JSON now has a
  stable SDK type.
- Replaced every remaining direct raw JSON REST request-body parameter with a
  serializable input across the service clients. Explicit query and iterator
  transport state continues to retain JSON values where requests must be
  replayable or API shapes are genuinely dynamic.
- Replaced the remaining raw Hire v1 request-body parameters with serializable
  typed inputs. Paginated Hire searches now use a replayable request-body
  wrapper, preserving safe multi-page request behavior.
- Replaced raw Hire v1 job-open and talent folder, identity-lookup, onboarding,
  and tagging mutation payloads with Go-shaped request models.
- Replaced raw CoreHR v1, VC v1, Docx v1, and OKR v1 JSON mutation payloads
  with Go-shaped request models. CoreHR now covers organization, metadata,
  offboarding, leave, calendar, and security-group writes; VC covers room,
  meeting, export, recording, reservation, and hierarchy writes.
- Replaced raw Task v2, Directory v1, Application v6, IM v2, and CardKit v1
  mutation bodies with Go-shaped request models, including task collaboration,
  directory employee lifecycle, application configuration, feed-card, tag,
  and card-element operations.
- Added typed Sheets v3 range-value, rich-text cell, find/replace, dimension,
  filter, sheet-property, and dropdown-option payloads. Range responses now
  expose structured range and update metadata instead of raw JSON.
- Replaced Go-backed raw CoreHR v1 and Lingo v1 request and response fields
  with concrete contact, hierarchy, permission, metadata, classification,
  image, status, and search-filter models.
- Replaced Go-backed raw Aily, Helpdesk, and Hire v1 request and response
  fields with concrete session, knowledge, ticket, notification, offer, and
  recruitment-configuration models.
- Replaced concrete Approval v4, Attendance v1, and Search v2 shared and
  response payload fields with Go-shaped models for attachments, i18n,
  approval actions, attendance rules, search schemas, filters, and results.
- Added typed Document AI recognition result graphs for invoice, identity,
  travel permit, license, resume, and contract field-extraction responses.
- Replaced raw Docx document and shared block fields with the existing
  Go-shaped document, text, table, media, task, and embedded-content models.
- Replaced nested JSON response fields across CoreHR, Attendance, VC, Mail,
  Application, Directory, Performance, Payroll, Docx, Approval, and related
  services with Go-shaped model graphs. Response data now reserves raw JSON
  only for Go-defined open-ended values.
- Replaced the remaining raw V2 response payloads across Performance, Approval,
  Directory, Payroll, Docx, Cardkit, OKR, Lingo, Baike, Application, Trust
  Party, Admin, ACS, and Bitable with Go-shaped response-data wrappers or
  code-only responses. No raw `impl_resp_v2!` response aliases remain.
- Added Go-shaped legacy response payloads for Sheets find/replace, Helpdesk
  agent schedules, Task batch deletes, Docx raw content, and Drive Explorer
  file creation. Status-only Auth, Board, Calendar, Directory, and OKR
  responses now discard their data bodies explicitly.
- Replaced Board v1 raw node and theme payloads with the complete Go-shaped
  whiteboard model graph, including typed node creation and PlantUML requests.

## [0.3.0] - 2026-07-12

### Breaking changes

- Removed the redundant `LarkClient::{do_req, do_req_typed, get, post, put,
  patch, delete}` methods. Use the explicit `raw_request*` APIs with an
  `ApiReq` instead.
- Renamed the public `Client` and `ClientBuilder` types to `LarkClient` and
  `LarkClientBuilder`. The legacy names are not retained as aliases.
- Removed the ambiguous `LarkClient::wiki()` compatibility accessor; use
  `wiki_v1()` or `wiki_v2()` explicitly.
- Updated Hire v1 `IdNameObject.name` to the typed `I18n` API shape and removed
  its non-Go `zh_name` and `en_name` fields.
- Made legacy `MessageCard::element` and `MessageCardAction::action` fallible.
  They now return `LarkError::Json` instead of silently inserting JSON `null`
  for invalid generic elements.
- Updated public JSON construction helpers to return `LarkError::Json` rather
  than exposing `serde_json::Error` directly.

### REST service coverage

- Replaced raw response payloads across CoreHR v1 and v2, APaaS, VC,
  Attendance, Mail, Application v6, and Compensation v1 with Go-shaped
  response-data wrappers or code-only responses where the Go SDK has no data.
- Added typed CoreHR v1 reference and job-taxonomy responses for country
  regions, currencies, national ID types, subdivisions, subregions, transfer
  reasons and types, employee types, jobs, job data, and job families.
- Added typed CoreHR v2 recent-change responses for company, cost center,
  custom organization, department, job, job family, job grade, job level,
  location, and position resources.
- Corrected CoreHR v1 and v2 status-only response wrappers so they expose unit
  data rather than untyped JSON payloads.
- Added typed CoreHR v1 common-data identifier conversion and enum-option
  metadata responses.
- Added typed CoreHR v1 company, department, job-level, location, and
  working-hours write responses, plus working-hours reads.
- Added typed Hire v2 interview-record and composite-talent response graphs,
  including nested assessment, custom-data, attachment, talent-pool, tag, and
  I18n models.
- Added Go-shaped typed Task v2 task, attachment, comment, custom-field,
  section, tasklist, and activity-subscription responses.
- Added typed Base v2 role, Block v2 entity/message, Security & Compliance v2
  device-record, MDM v3 country-region, and IM v2 feed-card/tag responses.

### Internal maintenance

- Split `LarkClient` implementation into focused builder, service-accessor,
  raw-request, and token modules without changing its public API.
- Shared identical Hire v1 and v2 I18n, identifier/name, registration, and
  resume-source models while preserving their versioned public paths.
- Routed JSON response creation through the common v2 response conversion
  path.

## [0.2.2] - 2026-07-11

### REST service coverage

- Added typed Hire v1 Offer salary-plan and job-requirement custom-data
  response leaves.
- Added typed Hire v1 Job response data for job configuration reads and writes,
  plus combined job create and update helpers.
- Added a typed Hire v1 application Offer response with structured offer,
  compensation, and send-record data.
- Added a typed Hire v1 Job detail response with structured job, manager,
  requirement, address, configuration, tag, and stage-count data.
- Added typed Hire v1 application-detail summaries and code-only application
  and Job lifecycle action responses.
- Added code-only response wrappers for selected Hire v1 talent, job
  requirement, agreement, advertisement, agency, EHR import, and Offer-field
  mutation helpers.
- Completed Go-backed code-only response wrappers across Hire v1 ECO account,
  background-check, package, exam, and exam-paper mutation helpers.
- Completed typed Hire v1 application-detail response branches for evaluations,
  interviews, Offer, employee, agency, portal, and referral data, plus intern
  Offer-status responses.
- Completed typed Hire v1 application-detail interview assessment, referral
  relationship, and application Offer delivery and address leaf models.
- Added typed Hire v1 interview feedback-form configuration and round-type
  assessment-template response models.
- Added typed Hire v1 background-check order reports, progress, contacts,
  provider, custom-field, and investigation-item response models.
- Added typed Hire v1 role permission scopes and reusable schema, questionnaire,
  and Offer-schema detail response models.
- Added typed Hire v1 job custom-field values, storefronts, stage counts,
  job-requirement custom fields, and Offer application-form configuration.
- Added typed Hire v1 Talent profile custom data, registrations, interview
  summaries, and interview-record scoring and assessment responses.

### App registration

- Added `AppAddons.preset` support, including explicit empty add-ons when the
  preset is disabled.

### Documentation and maintenance

- Reworked the README around installation, quick starts, common flows, feature
  flags, API coverage, and runnable examples.

## [0.2.1] - 2026-07-09

### REST service coverage

- Added lazy pagination iterators for generated Contact and Bitable list/search
  resources, with shared `limit`, `page_token`, and `next_page_token` controls
  matching the existing IM, Drive, and CoreHR iterator behavior.
- Added typed Hire v1 catalog responses and lazy iterators for registration
  schemas, resume sources, job functions, job types, locations, roles, and
  websites.
- Added typed Hire v1 reference responses for subjects, talent folders,
  termination reasons, todos, and user-role assignments, with lazy iterators
  for the paginated reference lists that expose Go SDK iterator support.
- Added typed Hire v1 task list responses and lazy iterators for evaluation,
  exam-marking, and interview tasks.
- Added typed Hire v1 interview-support list responses for application
  interviews, interviewers, feedback forms, registration schemas, and round
  types, with lazy iterators where the Go SDK exposes iterator support.
- Added typed Hire v1 website posting responses for website job posts, referral
  job posts, portal apply schemas, and website channels, with lazy iterators
  for the paginated job-posting and portal schema surfaces.
- Added typed Hire v1 read/list responses for job process and schema config,
  Offer application forms and approval templates, questionnaires, talent tags,
  employees, evaluations, notes, interview records, tripartite agreements, and
  background check orders, with lazy iterators where the Go SDK exposes
  `ListByIterator`.
- Added typed Hire v1 referral and talent read/search responses for referral
  lookup/search, talent object schemas, talent operation logs, and talent
  pools, with a lazy iterator for talent pool search.
- Added typed Hire v1 external read responses for external applications,
  external background checks, external interviews, external offers, and
  talent interview lookups, with lazy iterators for the Go-backed external
  list and batch-query surfaces.
- Added typed Hire v1 job utility read/search responses for job publish
  record search, job requirement lookup by ID, and location query helpers.
- Added typed Hire v1 agency read/query responses for agency batch query,
  agency detail, account lookup, protection search, and agency query helpers.
- Added typed Hire v1 auxiliary read responses for diversity inclusion search,
  interview record attachments, interview minutes, and website delivery task
  lookups.
- Added typed Hire v1 utility responses for job managers, referral account
  assets, talent ID lookup, test search, and website delivery creation, with a
  lazy iterator for test search.
- Added typed Hire v1 write utility responses for employee, interviewer, note,
  attachment, and website channel write helpers.
- Added typed Hire v1 external and adjacent write responses for external
  application, background check, interview, offer, interview assessment,
  referral reward, talent external info, and website site-user helpers.
- Added typed Hire v1 existing-model write responses for job manager batch
  updates, job requirement creation, referral account write actions, referral
  account reconciliation, referral account withdrawals, and tripartite
  agreement creation.
- Added typed Hire v1 simple Talent and Exam write responses for folder
  updates, combined talent create/update, talent-pool moves, and exam creation.
- Added typed Hire v1 action response wrappers for application transfer-onboard
  and job recruiter lookups.

### Documentation and maintenance

- Added an IM streaming-download example that writes chunks to disk, computes
  SHA-256 incrementally, and supports an optional byte limit.

## [0.2.0] - 2026-07-03

### REST service coverage

- Added named query-input structs and `*_by_query` methods across the generated
  service surface, including ACS, Admin, Application, Approval, Attendance,
  Auth/Authen, Baike, Bitable, Calendar, Contact, CoreHR, Directory, Docx,
  Drive, Helpdesk, Hire, IM, Lingo, Mail, Search, Task, VC, and related
  modules.
- Preserved existing positional methods as compatibility adapters while routing
  request construction through shared `RestRequest`, `PageQuery`, JSON,
  download, and upload helpers.
- Added a Go SDK v3.9.7 parity endpoint surface through `client.go_v397()` for
  generated endpoints that are not yet promoted to dedicated Rust resources.
- Added typed resources for bridge groups and expanded generated-service smoke
  coverage for request paths, query parameters, bodies, downloads, and uploads.

### Channel and event handling

- Added channel parity helpers for Go-SDK-style message normalization, send
  flows, upload inputs, runtime policy checks, bot identity caching, and stream
  handling behind the `channel` feature.
- Hardened typed callback decoding before handler dispatch and split callback
  payload, response, card action, and dispatch code into smaller internal
  modules.
- Added typed auth, e-learning, and task event coverage and shared card locale
  generation through the existing macro infrastructure.

### Documentation and maintenance

- Added curated runnable examples for OAuth, generated service calls, IM
  messaging, uploads/downloads, WebSocket events, channel helpers, and Go parity
  endpoints.
- Split the generated smoke-test harness into service-area modules while
  keeping the same request-shape coverage.
- Removed the standalone IM topic and card migration note now that the README
  and typed examples cover the supported channel and IM paths.

## [0.1.2] - 2026-06-15

### Event parity

- Replaced raw or stale P2 event payload shapes with typed structs for the
  security and compliance, compensation, apaas, moments, and helpdesk modules.
- Typed the IM message sender and mentions fields, removing the raw
  `serde_json::Value` shapes in favor of `UserId` and `Mention`.
- Removed stale raw event handler registrations that no longer matched the Go
  SDK event keys.
- Removed unsupported raw event modules and placeholder event modules without
  Go-backed payloads.
- Reduced typed event boilerplate with a shared `event_handlers!` macro and a
  common `events::common` module for shared payload types.

### Internal

- Added a `service_accessor!` macro to generate the `Client` service accessors,
  collapsing the repeated accessor boilerplate.
- Routed `EventDispatcher` and `CardActionHandler` through a shared dispatch
  pipeline for decryption, challenge handling, and signature verification.
- Isolated bearer token resolution into client assertion, direct, and cached
  modes within the transport module.

### Breaking changes

- `MessageSender.sender_id` is now `Option<UserId>` and `Message.mentions` is now
  `Vec<Mention>`; code reading these fields as raw `serde_json::Value` must update.
- Removed the unsupported and placeholder event modules; references to them no
  longer resolve.

## [0.1.1] - 2026-06-14

### Event parity

- Added Go-shaped typed IM event payload helpers for message readers,
  reactions, recalls, chat metadata, bot P2P entry, member lists, and
  topic/thread-aware message fields.
- Replaced raw or stale P2 event payload shapes with typed structs for
  contact, VC, CoreHR, Hire, application, drive, task, approval, calendar,
  ACS, mail, minutes, meeting room, payroll, and performance modules.
- Added missing Go-backed P2 event dispatcher registrations across the typed
  event parity slices.
- Added focused deserialization coverage for the new typed event payloads and
  null/empty payload handling.

### Cards and IM helpers

- Added typed helpers for interactive card content and raw request token access.
- Added typed IM topic and thread helpers for daemon/event routing flows.

### Fixes

- Accepted null IM event subfields without deserialization failures.
- Removed crates.io publishing from the GitHub release workflow so releases only
  create GitHub Release entries.

## [0.1.0] - 2026-06-13

Initial release of larksuite-oapi-sdk-rs, a Rust port of the
[larksuite-oapi-sdk-go](https://github.com/larksuite/oapi-sdk-go) Go SDK.

### Core

- Self-built and marketplace app authentication with automatic token
  management, caching, and retry on token expiry
- Client assertion provider (JWT bearer grant) for secretless deployments
- OAuth 2.0 authorization code exchange and refresh token rotation
  (`AccessToken` client under `authen.v1`)
- Configurable HTTP client with timeouts, default headers, retry policy,
  and proxy routing via `TargetInfo`
- Dial and server-timeout retry with configurable max retries
- `serde`-based typed API request and response infrastructure with
  strongly-typed `CodeError` parsing

### REST services

56+ service modules covering:

- Admin, ACS, Aily, Application, Attendance, Auth, Baike, Bitable,
  Calendar, Contact, CoreHR, Docs, Docx, Drive, EHR, Event, Gray Swan,
  Helpdesk, Hire, HumanAuth, IM, Lingji, Mail, MDM, Meeting Minutes,
  OKR, Passport, Pay, People, Performance, Personal, Report, Room,
  Search, Security, Sheets, Speech, Survey, Task, Tenant, Translate,
  Trust, VC, Verification, Wiki

### Event dispatch

- AES-256-CBC decryption with SHA-1 and SHA-256 signature verification
- `EventDispatcher` with P1 (event.type) and P2 (header event_type)
  protocol support
- `CardActionHandler` with URL verification and typed `CallbackAction`
- Type-safe `CustomResp` for returning custom responses to Lark
- 200+ typed event structs under the `events` module

### WebSocket (feature `ws`)

- Persistent long-connection WebSocket client with protobuf-framed
  messages, fragment reassembly, and ping/pong keepalive
- Server-side reconnect configuration (interval, nonce, count)
- Handshake error header parsing (`Handshake-Status`,
  `Handshake-Msg`, `Handshake-Autherrcode`) matching Go SDK
- Custom headers builder for the bootstrap HTTP request
  (Go SDK `WithHeaders`)

### Card builder

- Fluent `Card` builder (`card` module) for constructing interactive
  Lark message cards
- `MessageCard` legacy builder for IM card messages
- Full i18n support across 16 locales on both text and element levels
- Color templates, multi-column layouts, form elements, and chart
  markdown support

### Axum adapter (feature `axum`)

- `axum_handler` module for integrating event and card action handlers
  into Axum HTTP servers

### Requirements

- Rust 1.95.0+, edition 2024

[0.3.0]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.3.0
[0.2.2]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.2.2
[0.2.1]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.2.1
[0.2.0]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.2.0
[0.1.2]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.1.2
[0.1.1]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.1.1
[0.1.0]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.1.0
