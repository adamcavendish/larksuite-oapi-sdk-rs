# VC meeting bots

`client.vc().bot` provides `events`, `join`, `leave`, `message`, and
`user_active_meeting`, and `countdown`. These operations accept user or tenant tokens. Join,
leave, and message bodies are `Serialize`-generic; retain the long `meeting_id`
returned by join for subsequent leave and message requests.

Typed Webhook event registration for VC bots already lives in the event
dispatcher (`on_p2_vc_bot_meeting_*_v1`); this resource complements it with
outbound meeting control.

## Reading in-meeting events

`client.vc().bot.events(&query, option)` reads events from an active meeting.
Keep the credential identity that discovered the `meeting_id`: a meeting found
with a user token should be read with that user token, while a meeting found
through the application-bot path should continue with a tenant token. For
tenant-token reads, the application bot must still be in the meeting; switching
identity or reading after the bot has left can result in an empty response or a
platform permission error.

Do not use this endpoint as a post-meeting event archive. Once a meeting has
ended, retrieve the relevant meeting details or post-meeting artifact instead
(for example, minutes, transcript, or recording), according to the capability
and authorization available to the application. The SDK sends the supplied
request and does not infer membership, switch tokens, or retry with another
identity.

## Countdown control and events

`client.vc().bot.countdown(body, option)` accepts a serializable JSON body:

```json
{"meeting_id":"1234567890123456789","action":"set","duration":"5","reminder_before_end":"1","need_play_audio_at_end":false}
```

Use the long meeting ID, not the nine-digit meeting number. Actions are `set`,
`prolong`, `end_in_advance`, and `close_window`. `duration` is a string-valued
number of minutes required for set/prolong. `reminder_before_end` is also a
minute string; for set it must be positive and below duration. The audio flag
only applies to set and may be omitted; explicit false is preserved. The server
enforces the 24-hour limit (set rejects excess, prolong caps it). The SDK does
not schedule meetings or add business-level retries. The configured transport
retry policy still applies: by default it allows two attempts, including a
retry after HTTP 504. A non-idempotent action such as `prolong` may therefore be
resent even if the first attempt took effect. Use the client builder's
`.max_retries(1)` for a single transport attempt, and let the caller decide how
to reconcile an uncertain result before retrying.
Platform refusals propagate as `LarkError::Api` through the standard transport.

Typed `vc.bot.meeting_activity_v1` payloads now preserve `countdown_items`
(uppercase event actions, millisecond timestamp strings, second-valued reminder
arrays) and `magic_share_started_items[].start_reason`. A missing reason means
`share_started` according to the platform; `share_detected` means an existing
share was discovered. Deserialization retains absence rather than inserting a
synthetic default.

VC bot users, including countdown operators and meeting hosts, accept both
legacy string IDs and the newer structured `UserId` objects. The existing
`MeetingAgentEventUser.id: Option<String>` retains legacy IDs. New object IDs
are available through `structured_id`, preserving user/open/union namespaces.
Serialization emits the original `id` shape, never a `structured_id` wire key;
setting both Rust fields is rejected as ambiguous. Existing string-ID callers
remain source-compatible, but should inspect `structured_id` for new payloads.

Meeting search through `GoCompatibilityEndpoint::PostVcV1MeetingsSearch` now
accepts both user and tenant tokens, matching the pinned Go v3.12.0 contract.
