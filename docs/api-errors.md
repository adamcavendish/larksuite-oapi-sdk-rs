# API error diagnostics

`CodeError::msg` retains the original top-level OpenAPI `msg`. The server may
return a generic label there (such as `invalid param`) and put the actionable
diagnostic in `error.message`, exposed as `CodeErrorInfo::message`.

Use `CodeError::effective_message()` for a human-readable diagnostic. It prefers
a nonempty nested message and otherwise returns `msg`. `CodeError` and
`LarkError::Api` use this message in their `Display` output, retaining the
numeric code and nested log ID. No CLI permission rewrites or hints are applied.
Use the numeric code, not displayed text, for programmatic error classification.

Missing, null, or non-string nested messages deserialize as `None` rather than
preventing the rest of the API error from being decoded. An empty string is
preserved but falls back to `msg`; whitespace-only strings are used unchanged.
`None` is omitted when serialized. Other structured fields, including permission
and field violations, retain their existing behavior. Message selection does
not change `success()`: only code zero denotes success.

Where an `ApiResp` is retained, `raw_body` provides the original response bytes,
including malformed message values that the typed representation ignores.
The typed request path returns nonzero business codes as `LarkError::Api`,
not as a successful response tuple.

## Source compatibility

Adding the public `CodeErrorInfo::message` field requires callers using exhaustive
struct literals to add `message: None` (or their diagnostic). Callers already
using `..Default::default()` need no changes:

```rust
use larksuite_oapi_sdk_rs::resp::CodeErrorInfo;

let info = CodeErrorInfo {
    log_id: Some("request-log-id".into()),
    ..Default::default()
};
```

Existing top-level `msg` values are unchanged; displayed errors may now contain
the more specific server diagnostic.
