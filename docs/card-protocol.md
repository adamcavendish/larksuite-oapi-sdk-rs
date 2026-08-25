# Card protocol alignment

Message cards are an embedded JSON protocol: IM and CardKit APIs transport a
card as opaque content, so REST route parity cannot establish card-schema
coverage. This guide defines the required reference and verification workflow
for every change to the SDK's card support.

## Source roles

Use each reference for the thing it can prove:

1. **Official Lark card documentation** is the wire-contract authority. It
   defines Card JSON 1.0 and 2.0 roots, component tags, field names, value
   domains, constraints, client-version requirements, and deprecations.
2. **The Lark CLI card references and fixtures** are an operational
   cross-check. They supply representative valid payloads and callback/update
   lifecycle details, including `card.action.trigger`.
3. **The official Go SDK** is an implementation cross-check and compatibility
   reference. Its card model may deliberately represent a different layer or
   expose a different surface from a particular documentation page; an apparent
   gap is a discrepancy to investigate, not evidence that the Go SDK is
   obsolete or incomplete.
4. **The existing Rust SDK** is a regression baseline only. An existing
   serializer or a test that repeats its output is not evidence of protocol
   correctness.

When sources disagree, record the discrepancy in the protocol inventory and
investigate its version, schema, and transport context. The official
documentation defines the wire contract, while current Go SDK and CLI behavior
are required implementation evidence; do not assume that either implementation
is stale, or silently preserve a conflicting spelling.

## Required workflow

Before implementing or accepting an embedded-protocol change:

1. Classify it. REST resources and typed events use the Go compatibility
   workflow; message cards use this workflow. A change touching both must pass
   both sets of checks.
2. Refresh the pinned Go SDK and Lark CLI references, and record the source
   revision plus official-documentation URLs and access date in the protocol
   inventory.
3. Update the version-specific inventory before implementation. Card JSON 1.0
   and 2.0 have separate roots and must have separate component, field,
   enumeration, validation, and client-version entries.
4. Add or update exact JSON fixtures for every changed wire shape. Fixtures
   must cover the tag, required fields, optional fields, enum spelling, and at
   least one invalid cross-field combination where the protocol defines one.
5. Cross-check each changed card component against the CLI and Go references.
   Record an absence from either reference as an absence, not as permission to
   omit a documented capability.
6. Run the aggregate reference-alignment check and the focused Rust tests. A
   new card feature cannot merge with only a builder unit test that asserts the
   implementation's own JSON.

## Callback and update coverage

For callback-bearing Card JSON 1.0 components, the inventory and fixtures must
also cover the corresponding `card.action.trigger` fields, callback value or
form-value shape, and the three-second acknowledgement/update response. The
SDK may expose outbound card construction and inbound event types independently,
but their protocol relationship must be tested together.

Card JSON 2.0 and CardKit are separate tracks. CardKit's typed Card JSON 2.0
document transport and ordered text-content streaming have their own manifest
and transport fixtures; they are not optional extensions of a Card JSON 1.0
fixture. CardKit settings, element mutation, and batch-action helpers remain
separate protocol surfaces with their own exact transport fixtures.

## Gate status

The Card JSON 1.0 inventory and fixture corpus extend the deterministic
`reference-alignment-check` command with card fixtures and validation. The
aggregate command receives pinned Go and Lark CLI checkout paths, verifies
their revisions, and checks recorded source artifacts before it runs the Rust
fixture tests. Card checks remain one part of this aggregate gate, alongside
REST, event, and future protocol alignment; they do not get a separate command
or CI job.

The Card JSON 2.0 inventory is separate from the 1.0 corpus. It records the
incompatible root, every documented handwritten component, the root and
cross-field constraints that the typed AST must enforce, and exact valid and
invalid wire fixtures. `partial` means that a component has a typed wire shape
but is not yet eligible to be marked complete: its optional fields, validation,
and exact fixtures must still be covered. The documented Card-Builder-only
recycling container is recorded as a known divergence because it has no
handwritten Card JSON tag.

`implemented` is reserved for a component whose field-level inventory, valid
fixture, invalid constraint fixture where applicable, and direct serde
round-trip coverage are all present. A complete component group does not imply
that the incompatible Card JSON 2.0 root or unrelated interaction and update
surfaces are complete.

## CardKit document, mutation, and instance coverage

`card::cardkit::CardDocument` accepts only a locally validated Card JSON 2.0
document and emits CardKit's escaped `type: "card_json"` payload. The typed
`LarkClient::cardkit_cards()` helper covers entity creation and creates an
ordered update session for full replacement and full-content updates on one
text or Markdown element.

Each CardKit update carries a caller-provided idempotency key and a positive
sequence. One `CardKitUpdateSession` owns the sequence across document and
content, settings, element, and batch updates, advancing it only after a
successful request; a failed update can therefore be retried with the same
sequence and key. Content updates intentionally send the complete current
value: Lark renders a prefix extension as a typewriter update and otherwise
replaces the element content.

The typed mutation surface retains Card JSON 2.0 settings and full elements,
validates partial-element and insertion constraints, and encodes the nested
objects required by CardKit batch actions exactly once. It also covers template
instance creation and variable updates (a name-keyed object with open JSON
values). The pinned Go CardKit source does not expose Card Instance resources,
so those endpoints are normatively tracked against the official documentation
rather than unrelated Go template models. The raw generated resource methods
remain available for any future CardKit endpoint not yet given a typed
composition layer.
