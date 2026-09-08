# Mail v1 user-mailbox threads

`client.mail().user_mailbox_thread` exposes the Open Platform conversation
routes for a user mailbox:

- `list` reads conversations with pagination plus optional folder, unread, and
  label filters.
- `get` reads a conversation using the documented `format` and
  `include_spam_trash` controls.
- `batch_modify` changes labels or moves conversations to a non-Trash folder.
- `batch_trash` moves conversations to Trash.

All four operations accept a user or tenant access token. The mailbox ID is a
path parameter; callers using a user token may pass the platform-supported
`me` value directly, while the SDK deliberately does not resolve aliases.

`BatchModifyUserMailboxThreadReqBody::thread_ids` is required. Optional label
and folder fields are omitted when `None`; use `Some(vec![])` only when the
wire contract requires an explicit empty label list. Trash is intentionally a
separate operation, matching the platform contract.

Batch size limits, chunking, retries, confirmation prompts, and partial-failure
aggregation are application policy. The SDK sends exactly one request per
method invocation and returns the platform response unchanged.
