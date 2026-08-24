# Go service and event compatibility

The Rust SDK uses the official Go SDK as a compatibility reference for REST
resources and typed webhook registrations. This guide covers the fallback path
for an uncovered endpoint and the checked-in evidence that keeps those
contracts aligned.

This is deliberately not a universal protocol-parity claim. The generated
checks inspect Go `service/*/*/resource.go` and typed event registrations; they
do not inspect embedded JSON protocols such as message cards. Each embedded
protocol needs its own source-of-truth inventory, fixtures, and CI gate. See
[Card protocol alignment](card-protocol.md) for the card workflow.

## Raw requests and the Go bridge

Prefer a dedicated service resource when one exists. Named `*_by_query` methods
keep request paths, filters, and bodies explicit, while older positional methods
remain as compatibility adapters where they were already public.

For an endpoint without a generated Rust resource, use `ApiReq` through the raw
request APIs. Token selection, request IDs, retries, and error handling remain
the same as for typed resources. The `client.go_compatibility()` bridge provides
the same transport behavior for its curated newer-Go-SDK endpoint set. See
[`examples/raw_api.rs`](../examples/raw_api.rs) and
[`examples/go_compatibility_endpoint.rs`](../examples/go_compatibility_endpoint.rs).

## Compatibility metadata and service contracts

The GoCompatibility endpoint metadata is generated from the Go SDK's
`v3.6.1..v3.10.0` resource delta. `GoV397*` names and `go_v397()` accessors
remain as deprecated aliases. Regenerate or verify it with:

```sh
GO_SDK_DIR=/path/to/larksuite-oapi-sdk-go
go run ./tools/generate_go_compatibility_metadata.go --go-sdk "$GO_SDK_DIR" --from v3.6.1 --to v3.10.0
just go-compatibility-check "$GO_SDK_DIR"
```

The checked-in Go service contract catalog records the complete Go `v3.10.0`
request surface: source resource, receiver, operation, HTTP method, path, token
types, and upload behavior. It is a reproducible tooling input for future
service generation; it does not alter runtime request dispatch or generate Rust
resource implementations. It intentionally excludes platform APIs that the
pinned Go SDK has not generated, such as Application v7 slash commands.

```sh
GO_SDK_DIR=/path/to/larksuite-oapi-sdk-go
go run ./tools/generate_go_service_catalog.go --go-sdk "$GO_SDK_DIR" --revision v3.10.0
just go-service-catalog-check "$GO_SDK_DIR"
go run ./tools/generate_go_rust_service_parity.go
just go-rust-service-parity-check
```

`tools/go_rust_service_parity.json` compares the Go catalog with Rust's literal
and `format!`-based `RestRequest` wiring, supported macro-generated request
methods, and the GoCompatibility bridge. It classifies matches, bridge matches,
metadata mismatches, missing Go contracts, and unsupported request templates.
It is a deterministic request-contract baseline, not a generator for Rust
resource implementations.

## Typed webhook catalog

The checked-in Go event catalog records every typed P1 and P2 webhook route in
the same pinned Go revision. `tools/go_rust_event_parity.json` compares each Go
event key with the Rust `event_handlers!` registrations. It fails for a missing,
duplicate, or wrong-protocol Rust registration and reports Rust-only routes
without treating them as failures. It intentionally does not compare payload
field shapes.

```sh
GO_SDK_DIR=/path/to/larksuite-oapi-sdk-go
go run ./tools/generate_go_event_catalog.go --go-sdk "$GO_SDK_DIR" --revision v3.10.0
go run ./tools/generate_go_rust_event_parity.go
just go-event-catalog-check "$GO_SDK_DIR"
just go-rust-event-parity-check
```

CI runs `just reference-alignment-check` against pinned Go and Lark CLI
references. It runs the Go extractor tests and verifies compatibility metadata,
service contracts, webhook routes, both Rust parity reports, and the checked-in
embedded-protocol inventories and fixtures. Protocol families join this
aggregate gate instead of receiving parallel CI jobs.
