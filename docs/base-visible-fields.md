# Base view visible fields

`client.base_v3().view.get_visible_fields(base_token, table_id, view_id, option)`
reads the visible-field configuration. `set_visible_fields` takes those IDs,
`&SetViewVisibleFieldsReqBody`, and the request option. Both return `JsonResp`
so array- or object-valued response data remains available without coercion.
Both accept user or tenant tokens and send the configured application's
`X-App-Id`, using the same authorization boundary as other Base v3 resources.

Build the PUT body with `SetViewVisibleFieldsReqBody::new(field_ids)`.
It sends `{"visible_fields":["fld-first","fld-second"]}`: the **complete ordered
field-ID list**, not question IDs or a partial sort command. This also applies
to form views. Omitted fields become hidden, and the platform may force the
primary field to the first position. An empty list is serialized explicitly,
not omitted; the platform determines whether that configuration is allowed.

Read the current configuration before composing a replacement if unrelated
fields should remain visible. The SDK does not fetch/merge lists, sort them,
add business-level retries, or make implicit read-modify-write changes. The
configured transport retry policy still applies: by default it allows two
attempts, including a retry after HTTP 504. Use the client builder's
`.max_retries(1)` if the caller requires a single transport attempt; this does
not resolve whether a timed-out write took effect. Check `success()` and the
server response before assuming that the requested order was accepted.
