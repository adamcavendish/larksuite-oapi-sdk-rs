# Contributing

Thanks for improving `larksuite-oapi-sdk-rs`. This project is a non-official
Rust SDK for Lark / Feishu Open Platform APIs.

## Before opening an issue

Search existing issues first. For a bug report, include the crate version,
enabled Cargo features, Rust version, a minimal reproduction, and the expected
and actual result. Remove access tokens, app secrets, user identifiers, and
request payloads before posting.

For an API request, link the relevant Lark or Feishu Open Platform contract and
describe the Rust API you expect to use. The SDK deliberately keeps typed
resources, raw requests, and compatibility helpers separate, so an endpoint
reference helps maintainers select the right surface.

## Pull requests

Keep a pull request focused on one user-visible behavior. Include tests for
behavior changes and update examples or guides when a public API or workflow
changes. Do not include credentials, tenant data, or copied production payloads.

Before submitting, install [`just`](https://github.com/casey/just) and run the
checks relevant to your change:

```bash
just fmt-check
just clippy
just doc-check
just test-all
```

The repository supports Rust 1.95.0 and newer. A pull request is easier to
review when it explains the user problem, compatibility impact, and validation
performed.
