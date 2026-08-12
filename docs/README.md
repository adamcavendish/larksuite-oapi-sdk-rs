# Documentation

This directory holds detailed operational guides for the SDK. The root
[README](../README.md) remains the shortest path to installation, quick start,
examples, features, and coverage.

- [Go compatibility and contract provenance](go-compatibility.md) covers raw
  requests, the compatibility bridge, and reproducible Go-to-Rust checks.
- [Events, WebSockets, and channels](events-and-channels.md) covers webhook
  dispatch, long connections, trusted user channels, replies, and updates.
- [OAuth SSO](oauth-sso.md) covers Feishu and Lark browser authorization,
  authorization-code exchange, refresh-token rotation, and user identity lookup.
- [Spark db sync](spark-db-sync.md) covers user-token Base-to-database sync
  previews, task lifecycle operations, and opt-in live verification.
- [Base v3 record reads](base-v3-records.md) covers CLI-aligned Base record
  list/search operations and their read-only live verification.
- [Examples](../examples/README.md) contains runnable service workflows.
