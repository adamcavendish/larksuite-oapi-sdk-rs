package main

import (
	"strings"
	"testing"
)

func TestParseGoEventDispatcherExtractsTypedP2Registration(t *testing.T) {
	source := []byte(`package dispatcher
import larkvc "github.com/larksuite/oapi-sdk-go/v3/service/vc/v1"
func (d *EventDispatcher) OnP2NoteGeneratedV1(handler func(ctx context.Context, event *larkvc.P2NoteGeneratedV1) error) *EventDispatcher {
 d.eventType2EventHandler["vc.note.generated_v1"] = larkvc.NewP2NoteGeneratedV1Handler(handler)
 return d
}`)
	events, err := parseGoEventDispatcher("event/dispatcher/vc_v1_event_dispatch.go", source)
	if err != nil {
		t.Fatal(err)
	}
	if len(events) != 1 {
		t.Fatalf("events = %#v", events)
	}
	event := events[0]
	if event.Protocol != "p2" || event.Family != "vc" || event.Version != "v1" {
		t.Fatalf("event = %#v", event)
	}
	if event.Go.Method != "OnP2NoteGeneratedV1" || event.Go.Payload != "P2NoteGeneratedV1" {
		t.Fatalf("registration = %#v", event.Go)
	}
	if strings.Join(event.EventKeys, ",") != "vc.note.generated_v1" {
		t.Fatalf("keys = %#v", event.EventKeys)
	}
}

func TestParseGoEventDispatcherExtractsP1MultiKeyRegistration(t *testing.T) {
	source := []byte(`package dispatcher
import larkcontact "github.com/larksuite/oapi-sdk-go/v3/service/contact/v3"
func (d *EventDispatcher) OnP1UserChangedV3(handler func(ctx context.Context, event *larkcontact.P1UserChangedV3) error) *EventDispatcher {
 d.eventType2EventHandler["user_leave"] = larkcontact.NewP1UserChangedV3Handler(handler)
 d.eventType2EventHandler["user_add"] = larkcontact.NewP1UserChangedV3Handler(handler)
 return d
}`)
	events, err := parseGoEventDispatcher("event/dispatcher/ext_event_dispatch.go", source)
	if err != nil {
		t.Fatal(err)
	}
	if len(events) != 1 || events[0].Protocol != "p1" {
		t.Fatalf("events = %#v", events)
	}
	if strings.Join(events[0].EventKeys, ",") != "user_add,user_leave" {
		t.Fatalf("keys = %#v", events[0].EventKeys)
	}
}

func TestParseGoEventDispatcherSkipsNonServiceCallback(t *testing.T) {
	source := []byte(`package dispatcher
import larkevent "github.com/larksuite/oapi-sdk-go/v3/event"
func (d *EventDispatcher) OnP2CardAction(handler func(ctx context.Context, event *larkevent.CardAction) error) *EventDispatcher {
 d.eventType2EventHandler["card.action.trigger"] = handler
 return d
}`)
	events, err := parseGoEventDispatcher("event/dispatcher/card_event_dispatch.go", source)
	if err != nil {
		t.Fatal(err)
	}
	if len(events) != 0 {
		t.Fatalf("events = %#v, want none", events)
	}
}
