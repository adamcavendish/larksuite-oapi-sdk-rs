# Changelog

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

- Rust 1.94.0+, edition 2024

[0.1.1]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.1.1
[0.1.0]: https://github.com/adamcavendish/larksuite-oapi-sdk-rs/releases/tag/0.1.0
