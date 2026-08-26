# Documentation

This directory holds detailed operational guides for the SDK. The root
[README](../README.md) remains the shortest path to installation, quick start,
examples, features, and coverage.

- [Go service and event compatibility](go-compatibility.md) covers raw
  requests, the compatibility bridge, and reproducible Go-to-Rust checks.
- [Cards](cards.md) covers versioned Card JSON delivery, CardKit streaming,
  published Card Builder templates, and callbacks.
- [Card protocol alignment](card-protocol.md) defines the source hierarchy and
  verification workflow for Card JSON, card callbacks, and CardKit mutations.
- [Events, WebSockets, and channels](events-and-channels.md) covers webhook
  dispatch, long connections, trusted user channels, replies, and updates.
- [OAuth SSO](oauth-sso.md) covers Feishu and Lark browser authorization,
  authorization-code exchange, refresh-token rotation, and user identity lookup.
- [Spark db sync](spark-db-sync.md) covers user-token Base-to-database sync
  previews, task lifecycle operations, and opt-in live verification.
- [Base v3 record reads](base-v3-records.md) covers CLI-aligned Base record
  list/search operations and their read-only live verification.
- [Base v3 application mode](base-v3-apps.md) covers CLI-proven workspace,
  BaseApp, page, and block operations with user credentials.
- [Base v3 sharing](base-v3-sharing.md) covers dashboard and form sharing
  reads and partial updates with user or tenant credentials.
- [Docs AI document content](docs-ai.md) covers modern Docx content fetches,
  updates, and version-history operations with user or bot credentials.
- [Slides AI presentation content](slides-ai.md) covers XML presentation,
  slide image, and version-history operations with user or bot credentials.
- [Examples](../examples/README.md) contains runnable service workflows.
