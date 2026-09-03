# VC meeting bots

`client.vc().bot` provides `events`, `join`, `leave`, `message`, and
`user_active_meeting`. These operations accept user or tenant tokens. Join,
leave, and message bodies are `Serialize`-generic; retain the long `meeting_id`
returned by join for subsequent leave and message requests.

Typed Webhook event registration for VC bots already lives in the event
dispatcher (`on_p2_vc_bot_meeting_*_v1`); this resource complements it with
outbound meeting control.
