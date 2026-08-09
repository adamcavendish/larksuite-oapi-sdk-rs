// generate_go_rust_event_parity compares Go typed webhook registrations with Rust registrations.
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

const (
	defaultGoEventCatalog = "tools/go_event_catalog.json"
	defaultEventParity    = "tools/go_rust_event_parity.json"
)

type eventCatalogRegistration struct {
	DispatcherFile string `json:"dispatcher_file"`
	Line           int    `json:"line"`
	Method         string `json:"method"`
	Package        string `json:"package"`
	Payload        string `json:"payload"`
}

type catalogEvent struct {
	Protocol  string                   `json:"protocol"`
	EventKeys []string                 `json:"event_keys"`
	Family    string                   `json:"family"`
	Version   string                   `json:"version"`
	Go        eventCatalogRegistration `json:"go"`
}

type eventCatalog struct {
	SchemaVersion int            `json:"schema_version"`
	Events        []catalogEvent `json:"events"`
}

type rustEventRegistration struct {
	SourceFile string `json:"source_file"`
	Line       int    `json:"line"`
	Method     string `json:"method"`
	Payload    string `json:"payload"`
	Protocol   string `json:"protocol"`
	EventKey   string `json:"event_key"`
}

func (event rustEventRegistration) identity() string {
	return event.SourceFile + ":" + event.Method + ":" + event.EventKey
}

type missingGoEvent struct {
	EventKey string       `json:"event_key"`
	Go       catalogEvent `json:"go"`
}

type protocolMismatch struct {
	EventKey string                `json:"event_key"`
	Go       catalogEvent          `json:"go"`
	Rust     rustEventRegistration `json:"rust"`
}

type eventParitySummary struct {
	GoTypedEventKeys       int `json:"go_typed_event_keys"`
	RustTypedEventKeys     int `json:"rust_typed_event_keys"`
	MatchedEventKeys       int `json:"matched_event_keys"`
	MissingGoEventKeys     int `json:"missing_go_event_keys"`
	DuplicateRustEventKeys int `json:"duplicate_rust_event_keys"`
	ProtocolMismatches     int `json:"protocol_mismatches"`
	RustOnlyEventKeys      int `json:"rust_only_event_keys"`
}

type eventParityReport struct {
	SchemaVersion          int                       `json:"schema_version"`
	GoEventCatalogSHA256   string                    `json:"go_event_catalog_sha256"`
	Summary                eventParitySummary        `json:"summary"`
	MissingGoEventKeys     []missingGoEvent          `json:"missing_go_event_keys"`
	DuplicateRustEventKeys [][]rustEventRegistration `json:"duplicate_rust_event_keys"`
	ProtocolMismatches     []protocolMismatch        `json:"protocol_mismatches"`
	RustOnlyEventKeys      []rustEventRegistration   `json:"rust_only_event_keys"`
}

var eventHandlerEntry = regexp.MustCompile(`(?s)(on_[a-z0-9_]+)\s*=>\s*([A-Za-z_][A-Za-z0-9_:]*)\s*:\s*("(?:[^"\\]|\\.)*"|\[[^\]]*\])\s*,`)
var eventKeyLiteral = regexp.MustCompile(`"([^"\\]*(?:\\.[^"\\]*)*)"`)

func main() {
	goCatalog := flag.String("go-catalog", defaultGoEventCatalog, "path to the generated Go event catalog")
	rustSDK := flag.String("rust-sdk", ".", "path to the Rust SDK checkout")
	output := flag.String("output", defaultEventParity, "generated parity report path")
	check := flag.Bool("check", false, "fail when the generated report is stale or has unresolved typed webhook coverage")
	flag.Parse()

	generated, report, err := generateEventParity(*goCatalog, *rustSDK)
	if err != nil {
		failEventParity(err)
	}
	if *check {
		existing, err := os.ReadFile(*output)
		if err != nil {
			failEventParity(fmt.Errorf("read %s: %w", *output, err))
		}
		if !bytes.Equal(existing, generated) {
			failEventParity(fmt.Errorf("%s is stale; rerun this command without --check", *output))
		}
		if err := validateEventParity(report); err != nil {
			failEventParity(err)
		}
		return
	}
	if existing, err := os.ReadFile(*output); err == nil && bytes.Equal(existing, generated) {
		return
	}
	if err := os.WriteFile(*output, generated, 0o644); err != nil {
		failEventParity(fmt.Errorf("write %s: %w", *output, err))
	}
}

func failEventParity(err error) {
	fmt.Fprintln(os.Stderr, "generate_go_rust_event_parity:", err)
	os.Exit(1)
}

func generateEventParity(goCatalogPath, rustSDK string) ([]byte, eventParityReport, error) {
	goCatalogBytes, err := os.ReadFile(goCatalogPath)
	if err != nil {
		return nil, eventParityReport{}, fmt.Errorf("read Go event catalog: %w", err)
	}
	var catalog eventCatalog
	if err := json.Unmarshal(goCatalogBytes, &catalog); err != nil {
		return nil, eventParityReport{}, fmt.Errorf("decode Go event catalog: %w", err)
	}
	if catalog.SchemaVersion != 1 || len(catalog.Events) == 0 {
		return nil, eventParityReport{}, errors.New("unsupported or empty Go event catalog")
	}
	rustEvents, err := rustTypedEvents(rustSDK)
	if err != nil {
		return nil, eventParityReport{}, err
	}

	goByKey := make(map[string]catalogEvent)
	for _, event := range catalog.Events {
		for _, key := range event.EventKeys {
			if _, exists := goByKey[key]; exists {
				return nil, eventParityReport{}, fmt.Errorf("duplicate Go event key %s", key)
			}
			goByKey[key] = event
		}
	}
	rustByKey := make(map[string][]rustEventRegistration)
	for _, event := range rustEvents {
		rustByKey[event.EventKey] = append(rustByKey[event.EventKey], event)
	}

	report := eventParityReport{
		SchemaVersion:        1,
		GoEventCatalogSHA256: sha256Hex(goCatalogBytes),
		Summary: eventParitySummary{
			GoTypedEventKeys: len(goByKey), RustTypedEventKeys: len(rustByKey),
		},
	}
	for key, goEvent := range goByKey {
		rustMatches := rustByKey[key]
		switch len(rustMatches) {
		case 0:
			report.MissingGoEventKeys = append(report.MissingGoEventKeys, missingGoEvent{EventKey: key, Go: goEvent})
		case 1:
			if rustMatches[0].Protocol != goEvent.Protocol {
				report.ProtocolMismatches = append(report.ProtocolMismatches, protocolMismatch{EventKey: key, Go: goEvent, Rust: rustMatches[0]})
			} else {
				report.Summary.MatchedEventKeys++
			}
		default:
			report.DuplicateRustEventKeys = append(report.DuplicateRustEventKeys, rustMatches)
		}
	}
	for key, events := range rustByKey {
		if _, exists := goByKey[key]; !exists && len(events) == 1 {
			report.RustOnlyEventKeys = append(report.RustOnlyEventKeys, events[0])
		}
	}
	report.Summary.MissingGoEventKeys = len(report.MissingGoEventKeys)
	report.Summary.DuplicateRustEventKeys = len(report.DuplicateRustEventKeys)
	report.Summary.ProtocolMismatches = len(report.ProtocolMismatches)
	report.Summary.RustOnlyEventKeys = len(report.RustOnlyEventKeys)
	sort.Slice(report.MissingGoEventKeys, func(i, j int) bool {
		return report.MissingGoEventKeys[i].EventKey < report.MissingGoEventKeys[j].EventKey
	})
	sort.Slice(report.DuplicateRustEventKeys, func(i, j int) bool {
		return report.DuplicateRustEventKeys[i][0].EventKey < report.DuplicateRustEventKeys[j][0].EventKey
	})
	sort.Slice(report.ProtocolMismatches, func(i, j int) bool {
		return report.ProtocolMismatches[i].EventKey < report.ProtocolMismatches[j].EventKey
	})
	sort.Slice(report.RustOnlyEventKeys, func(i, j int) bool {
		return report.RustOnlyEventKeys[i].EventKey < report.RustOnlyEventKeys[j].EventKey
	})

	var output bytes.Buffer
	encoder := json.NewEncoder(&output)
	encoder.SetEscapeHTML(false)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(report); err != nil {
		return nil, eventParityReport{}, fmt.Errorf("encode event parity report: %w", err)
	}
	return output.Bytes(), report, nil
}

func rustTypedEvents(rustSDK string) ([]rustEventRegistration, error) {
	eventsRoot := filepath.Join(rustSDK, "src", "events")
	entries, err := os.ReadDir(eventsRoot)
	if err != nil {
		return nil, fmt.Errorf("read Rust event modules: %w", err)
	}
	var events []rustEventRegistration
	for _, entry := range entries {
		if entry.IsDir() || filepath.Ext(entry.Name()) != ".rs" || entry.Name() == "common.rs" || entry.Name() == "mod.rs" {
			continue
		}
		path := filepath.Join(eventsRoot, entry.Name())
		source, err := os.ReadFile(path)
		if err != nil {
			return nil, fmt.Errorf("read %s: %w", path, err)
		}
		relative, err := filepath.Rel(rustSDK, path)
		if err != nil {
			return nil, err
		}
		parsed, err := parseRustEventHandlers(filepath.ToSlash(relative), string(source))
		if err != nil {
			return nil, err
		}
		events = append(events, parsed...)
	}
	sort.Slice(events, func(i, j int) bool { return events[i].identity() < events[j].identity() })
	return events, nil
}

func parseRustEventHandlers(sourceFile, source string) ([]rustEventRegistration, error) {
	var events []rustEventRegistration
	for _, match := range eventHandlerEntry.FindAllStringSubmatchIndex(source, -1) {
		method := source[match[2]:match[3]]
		payload := source[match[4]:match[5]]
		keys, err := eventKeys(source[match[6]:match[7]])
		if err != nil {
			return nil, fmt.Errorf("parse %s %s: %w", sourceFile, method, err)
		}
		protocol := "p2"
		if strings.HasPrefix(method, "on_p1_") || strings.HasSuffix(sourceFile, "/p1.rs") {
			protocol = "p1"
		}
		line := 1 + strings.Count(source[:match[0]], "\n")
		for _, key := range keys {
			events = append(events, rustEventRegistration{SourceFile: sourceFile, Line: line, Method: method, Payload: payload, Protocol: protocol, EventKey: key})
		}
	}
	return events, nil
}

func eventKeys(expression string) ([]string, error) {
	if strings.HasPrefix(expression, "\"") {
		key, err := strconv.Unquote(expression)
		if err != nil {
			return nil, err
		}
		return []string{key}, nil
	}
	matches := eventKeyLiteral.FindAllStringSubmatch(expression, -1)
	if len(matches) == 0 {
		return nil, errors.New("no literal event keys")
	}
	keys := make([]string, 0, len(matches))
	for _, match := range matches {
		key, err := strconv.Unquote("\"" + match[1] + "\"")
		if err != nil {
			return nil, err
		}
		keys = append(keys, key)
	}
	return keys, nil
}

func validateEventParity(report eventParityReport) error {
	if report.Summary.MissingGoEventKeys != 0 || report.Summary.DuplicateRustEventKeys != 0 || report.Summary.ProtocolMismatches != 0 {
		return fmt.Errorf("unresolved typed webhook parity: missing=%d duplicate_rust=%d protocol_mismatches=%d", report.Summary.MissingGoEventKeys, report.Summary.DuplicateRustEventKeys, report.Summary.ProtocolMismatches)
	}
	return nil
}

func sha256Hex(value []byte) string {
	sum := sha256.Sum256(value)
	return hex.EncodeToString(sum[:])
}
