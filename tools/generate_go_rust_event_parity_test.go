package main

import (
	"encoding/json"
	"testing"
)

func TestParseRustEventHandlersExtractsSingleAndMultiKeyEntries(t *testing.T) {
	source := `event_handlers! {
    on_p2_vc_note_generated_v1 => P2VcNoteGeneratedV1 : "vc.note.generated_v1",
    on_p1_user_changed_v3 => P1UserChangedV3 : ["user_add", "user_leave"],
}`
	events, err := parseRustEventHandlers("src/events/p1.rs", source)
	if err != nil {
		t.Fatal(err)
	}
	if len(events) != 3 {
		t.Fatalf("events = %#v", events)
	}
	if events[0].Protocol != "p1" || events[0].Payload == "" || events[0].Method == "" {
		t.Fatalf("event = %#v", events[0])
	}
}

func TestValidateEventParityRejectsMissingAndDuplicateRegistrations(t *testing.T) {
	report := eventParityReport{Summary: eventParitySummary{MissingGoEventKeys: 1, DuplicateRustEventKeys: 1}}
	if err := validateEventParity(report); err == nil {
		t.Fatal("expected unresolved parity error")
	}
}

func TestCurrentEventParityBaselineHasNoUnresolvedCoverage(t *testing.T) {
	generated, report, err := generateEventParity("go_event_catalog.json", "..")
	if err != nil {
		t.Fatal(err)
	}
	if err := validateEventParity(report); err != nil {
		t.Fatal(err)
	}
	var decoded eventParityReport
	if err := json.Unmarshal(generated, &decoded); err != nil {
		t.Fatal(err)
	}
	if decoded.Summary.GoTypedEventKeys != 227 || decoded.Summary.MatchedEventKeys != 227 {
		t.Fatalf("summary = %#v", decoded.Summary)
	}
}
