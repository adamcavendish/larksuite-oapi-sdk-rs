// generate_go_rust_service_parity compares Go service contracts with Rust request wiring.
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
	defaultCatalog = "tools/go_service_catalog.json"
	defaultOutput  = "tools/go_rust_service_parity.json"
)

type goCatalog struct {
	EndpointCount int          `json:"endpoint_count"`
	Endpoints     []goEndpoint `json:"endpoints"`
}

type goEndpoint struct {
	ResourceFile string   `json:"resource_file"`
	Receiver     string   `json:"receiver"`
	Operation    string   `json:"operation"`
	Method       string   `json:"method"`
	Path         string   `json:"path"`
	TokenTypes   []string `json:"token_types"`
	FileUpload   bool     `json:"file_upload"`
}

func (endpoint goEndpoint) identity() string {
	return endpoint.ResourceFile + ":" + endpoint.Receiver + "." + endpoint.Operation
}

type rustEndpoint struct {
	SourceFile string   `json:"source_file"`
	Function   string   `json:"function"`
	Line       int      `json:"line"`
	Method     string   `json:"method"`
	Path       string   `json:"path"`
	TokenTypes []string `json:"token_types"`
	FileUpload bool     `json:"file_upload"`
}

type unparsedRustRequest struct {
	SourceFile string `json:"source_file"`
	Function   string `json:"function"`
	Line       int    `json:"line"`
	Reason     string `json:"reason"`
}

type metadataMismatch struct {
	Contract goEndpoint     `json:"contract"`
	Rust     []rustEndpoint `json:"rust"`
	Bridge   []rustEndpoint `json:"bridge"`
}

type paritySummary struct {
	GoContracts           int `json:"go_contracts"`
	RustDirectContracts   int `json:"rust_direct_contracts"`
	GoV397BridgeContracts int `json:"go_v397_bridge_contracts"`
	TypedMatches          int `json:"typed_matches"`
	BridgeMatches         int `json:"bridge_matches"`
	MetadataMismatches    int `json:"metadata_mismatches"`
	MissingContracts      int `json:"missing_contracts"`
	UnparsedRustRequests  int `json:"unparsed_rust_requests"`
}

type parityReport struct {
	SchemaVersion        int                   `json:"schema_version"`
	GoCatalogSHA256      string                `json:"go_catalog_sha256"`
	Summary              paritySummary         `json:"summary"`
	MetadataMismatches   []metadataMismatch    `json:"metadata_mismatches"`
	MissingContracts     []goEndpoint          `json:"missing_contracts"`
	UnparsedRustRequests []unparsedRustRequest `json:"unparsed_rust_requests"`
}

var functionStart = regexp.MustCompile(`(?m)^\s*(pub(\([^)]*\))?\s+)?(async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*`)
var methodPattern = regexp.MustCompile(`http::Method::([A-Za-z]+)`)
var tokenPattern = regexp.MustCompile(`AccessTokenType::([A-Za-z]+)`)
var pathVariablePattern = regexp.MustCompile(`(?s)let\s+(mut\s+)?path\s*=\s*(.*?);`)

func main() {
	goCatalogPath := flag.String("go-catalog", defaultCatalog, "path to the generated Go service contract catalog")
	rustSDK := flag.String("rust-sdk", ".", "path to the Rust SDK checkout")
	output := flag.String("output", defaultOutput, "generated parity report path")
	check := flag.Bool("check", false, "fail when the generated report is stale")
	flag.Parse()

	generated, err := generate(*goCatalogPath, *rustSDK)
	if err != nil {
		fail(err)
	}

	existing, err := os.ReadFile(*output)
	if *check {
		if err != nil {
			fail(fmt.Errorf("read %s: %w", *output, err))
		}
		if !bytes.Equal(existing, generated) {
			fail(fmt.Errorf("%s is stale; rerun this command without --check", *output))
		}
		return
	}
	if err == nil && bytes.Equal(existing, generated) {
		return
	}
	if err := os.WriteFile(*output, generated, 0o644); err != nil {
		fail(fmt.Errorf("write %s: %w", *output, err))
	}
}

func fail(err error) {
	fmt.Fprintln(os.Stderr, "generate_go_rust_service_parity:", err)
	os.Exit(1)
}

func generate(goCatalogPath, rustSDK string) ([]byte, error) {
	goCatalogBytes, err := os.ReadFile(goCatalogPath)
	if err != nil {
		return nil, fmt.Errorf("read Go catalog: %w", err)
	}
	var catalog goCatalog
	if err := json.Unmarshal(goCatalogBytes, &catalog); err != nil {
		return nil, fmt.Errorf("decode Go catalog: %w", err)
	}
	if catalog.EndpointCount != len(catalog.Endpoints) {
		return nil, fmt.Errorf("Go catalog endpoint count %d does not match %d entries", catalog.EndpointCount, len(catalog.Endpoints))
	}

	direct, unparsed, err := rustServiceContracts(rustSDK)
	if err != nil {
		return nil, err
	}
	bridge, err := goV397Contracts(rustSDK)
	if err != nil {
		return nil, err
	}

	directByRoute := contractsByRoute(direct)
	bridgeByRoute := contractsByRoute(bridge)
	report := parityReport{
		SchemaVersion:   1,
		GoCatalogSHA256: sha256Hex(goCatalogBytes),
		Summary: paritySummary{
			GoContracts:           len(catalog.Endpoints),
			RustDirectContracts:   len(direct),
			GoV397BridgeContracts: len(bridge),
		},
		UnparsedRustRequests: unparsed,
	}

	for _, endpoint := range catalog.Endpoints {
		route := routeKey(endpoint.Method, endpoint.Path)
		directMatches := directByRoute[route]
		bridgeMatches := bridgeByRoute[route]
		if anyMetadataMatches(endpoint, directMatches) {
			report.Summary.TypedMatches++
			continue
		}
		if anyMetadataMatches(endpoint, bridgeMatches) {
			report.Summary.BridgeMatches++
			continue
		}
		if len(directMatches) > 0 || len(bridgeMatches) > 0 {
			report.MetadataMismatches = append(report.MetadataMismatches, metadataMismatch{
				Contract: endpoint,
				Rust:     directMatches,
				Bridge:   bridgeMatches,
			})
			continue
		}
		report.MissingContracts = append(report.MissingContracts, endpoint)
	}

	report.Summary.MetadataMismatches = len(report.MetadataMismatches)
	report.Summary.MissingContracts = len(report.MissingContracts)
	report.Summary.UnparsedRustRequests = len(report.UnparsedRustRequests)
	sort.Slice(report.MetadataMismatches, func(i, j int) bool {
		return report.MetadataMismatches[i].Contract.identity() < report.MetadataMismatches[j].Contract.identity()
	})
	sort.Slice(report.MissingContracts, func(i, j int) bool {
		return report.MissingContracts[i].identity() < report.MissingContracts[j].identity()
	})
	sort.Slice(report.UnparsedRustRequests, func(i, j int) bool {
		left := report.UnparsedRustRequests[i]
		right := report.UnparsedRustRequests[j]
		if left.SourceFile != right.SourceFile {
			return left.SourceFile < right.SourceFile
		}
		return left.Line < right.Line
	})

	var output bytes.Buffer
	encoder := json.NewEncoder(&output)
	encoder.SetEscapeHTML(false)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(report); err != nil {
		return nil, fmt.Errorf("encode parity report: %w", err)
	}
	return output.Bytes(), nil
}

func rustServiceContracts(rustSDK string) ([]rustEndpoint, []unparsedRustRequest, error) {
	serviceRoot := filepath.Join(rustSDK, "src", "service")
	var contracts []rustEndpoint
	var unparsed []unparsedRustRequest
	err := filepath.WalkDir(serviceRoot, func(path string, entry os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if entry.IsDir() || filepath.Ext(path) != ".rs" || excludedServiceFile(path) {
			return nil
		}
		source, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		relative, err := filepath.Rel(rustSDK, path)
		if err != nil {
			return err
		}
		parsed, unresolved := extractRustContracts(filepath.ToSlash(relative), string(source))
		contracts = append(contracts, parsed...)
		unparsed = append(unparsed, unresolved...)
		return nil
	})
	if err != nil {
		return nil, nil, fmt.Errorf("read Rust service contracts: %w", err)
	}
	sort.Slice(contracts, func(i, j int) bool {
		if contracts[i].SourceFile != contracts[j].SourceFile {
			return contracts[i].SourceFile < contracts[j].SourceFile
		}
		return contracts[i].Line < contracts[j].Line
	})
	return contracts, unparsed, nil
}

func excludedServiceFile(path string) bool {
	name := filepath.Base(path)
	return name == "common.rs" || name == "go_v397.rs" || name == "go_v397_metadata.rs" || name == "mod.rs"
}

func extractRustContracts(sourceFile, source string) ([]rustEndpoint, []unparsedRustRequest) {
	var contracts []rustEndpoint
	var unparsed []unparsedRustRequest
	for _, match := range functionStart.FindAllStringSubmatchIndex(source, -1) {
		function := source[match[8]:match[9]]
		open := strings.Index(source[match[1]:], "{")
		if open < 0 {
			continue
		}
		open += match[1]
		close, ok := matchingDelimiter(source, open, '{', '}')
		if !ok {
			continue
		}
		body := source[open : close+1]
		for offset := 0; ; {
			index := strings.Index(body[offset:], "RestRequest::new(")
			if index < 0 {
				break
			}
			callStart := offset + index
			argsStart := callStart + len("RestRequest::new")
			closeCall, ok := matchingDelimiter(body, argsStart, '(', ')')
			line := lineNumber(source, open+callStart)
			if !ok {
				unparsed = append(unparsed, unparsedRustRequest{sourceFile, function, line, "unclosed RestRequest::new call"})
				break
			}
			args := splitTopLevel(body[argsStart+1 : closeCall])
			endpoint, reason := endpointFromRustCall(sourceFile, function, line, body[:callStart], args, body[closeCall:])
			if reason != "" {
				unparsed = append(unparsed, unparsedRustRequest{sourceFile, function, line, reason})
			} else {
				contracts = append(contracts, endpoint)
			}
			offset = closeCall + 1
		}
	}
	return contracts, unparsed
}

func endpointFromRustCall(sourceFile, function string, line int, prefix string, args []string, suffix string) (rustEndpoint, string) {
	if len(args) < 4 {
		return rustEndpoint{}, "RestRequest::new has fewer than four arguments"
	}
	methodMatch := methodPattern.FindStringSubmatch(args[1])
	if len(methodMatch) != 2 {
		return rustEndpoint{}, "non-literal HTTP method"
	}
	path, ok := rustPath(args[2], prefix)
	if !ok {
		return rustEndpoint{}, "non-literal request path"
	}
	return rustEndpoint{
		SourceFile: sourceFile,
		Function:   function,
		Line:       line,
		Method:     strings.ToUpper(methodMatch[1]),
		Path:       path,
		TokenTypes: rustTokenTypes(args[3]),
		FileUpload: rustFileUpload(nextStatement(suffix)),
	}, ""
}

func rustFileUpload(statement string) bool {
	return strings.Contains(statement, ".file_upload()") || strings.Contains(statement, ".form_body(")
}

func rustPath(expression, prefix string) (string, bool) {
	expression = strings.TrimSpace(expression)
	if path, ok := pathFromExpression(expression); ok {
		return normalizePath(path), true
	}
	if expression != "path" {
		return "", false
	}
	matches := pathVariablePattern.FindAllStringSubmatch(prefix, -1)
	if len(matches) == 0 {
		return "", false
	}
	path, ok := pathFromExpression(matches[len(matches)-1][2])
	if !ok {
		return "", false
	}
	return normalizePath(path), true
}

func pathFromExpression(expression string) (string, bool) {
	expression = strings.TrimSpace(expression)
	if strings.HasPrefix(expression, "format!(") {
		open := strings.Index(expression, "(")
		close, ok := matchingDelimiter(expression, open, '(', ')')
		if !ok {
			return "", false
		}
		arguments := splitTopLevel(expression[open+1 : close])
		if len(arguments) == 0 {
			return "", false
		}
		return rustString(arguments[0])
	}
	return rustString(expression)
}

func rustString(expression string) (string, bool) {
	expression = strings.TrimSpace(expression)
	if !strings.HasPrefix(expression, "\"") {
		return "", false
	}
	value, err := strconv.Unquote(expression)
	return value, err == nil
}

func normalizePath(path string) string {
	segments := strings.Split(path, "/")
	for index, segment := range segments {
		if strings.HasPrefix(segment, ":") || (strings.HasPrefix(segment, "{") && strings.HasSuffix(segment, "}")) {
			segments[index] = "{}"
		}
	}
	return strings.Join(segments, "/")
}

func rustTokenTypes(expression string) []string {
	seen := make(map[string]struct{})
	for _, match := range tokenPattern.FindAllStringSubmatch(expression, -1) {
		if match[1] != "None" {
			seen[match[1]] = struct{}{}
		}
	}
	tokens := make([]string, 0, len(seen))
	for token := range seen {
		tokens = append(tokens, token)
	}
	sort.Strings(tokens)
	return tokens
}

func goV397Contracts(rustSDK string) ([]rustEndpoint, error) {
	path := filepath.Join(rustSDK, "src", "service", "go_v397_metadata.rs")
	source, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("read GoV397 metadata: %w", err)
	}
	blocks := regexp.MustCompile(`(?s)Self::[A-Za-z0-9_]+ => GoV397EndpointMeta \{(.*?)\n            \},`).FindAllStringSubmatch(string(source), -1)
	contracts := make([]rustEndpoint, 0, len(blocks))
	for _, block := range blocks {
		methodMatch := methodPattern.FindStringSubmatch(block[1])
		pathMatch := regexp.MustCompile(`path: "([^"]+)"`).FindStringSubmatch(block[1])
		if len(methodMatch) != 2 || len(pathMatch) != 2 {
			return nil, errors.New("incomplete GoV397 endpoint metadata")
		}
		contracts = append(contracts, rustEndpoint{
			SourceFile: "src/service/go_v397_metadata.rs",
			Function:   "GoV397Endpoint",
			Method:     strings.ToUpper(methodMatch[1]),
			Path:       normalizePath(pathMatch[1]),
			TokenTypes: rustTokenTypes(block[1]),
			FileUpload: strings.Contains(block[1], "file_upload: true"),
		})
	}
	if len(contracts) == 0 {
		return nil, errors.New("no GoV397 endpoint metadata found")
	}
	return contracts, nil
}

func contractsByRoute(contracts []rustEndpoint) map[string][]rustEndpoint {
	byRoute := make(map[string][]rustEndpoint)
	for _, contract := range contracts {
		key := routeKey(contract.Method, contract.Path)
		byRoute[key] = append(byRoute[key], contract)
	}
	return byRoute
}

func routeKey(method, path string) string {
	return strings.ToUpper(method) + " " + normalizePath(path)
}

func anyMetadataMatches(endpoint goEndpoint, candidates []rustEndpoint) bool {
	for _, candidate := range candidates {
		if endpoint.FileUpload == candidate.FileUpload && equalTokenTypes(endpoint.TokenTypes, candidate.TokenTypes) {
			return true
		}
	}
	return false
}

func equalTokenTypes(left, right []string) bool {
	left = append([]string(nil), left...)
	right = append([]string(nil), right...)
	sort.Strings(left)
	sort.Strings(right)
	return strings.Join(left, ",") == strings.Join(right, ",")
}

func splitTopLevel(input string) []string {
	var parts []string
	start := 0
	depth := 0
	inString := false
	escaped := false
	for index, character := range input {
		if inString {
			if escaped {
				escaped = false
				continue
			}
			if character == '\\' {
				escaped = true
			} else if character == '"' {
				inString = false
			}
			continue
		}
		switch character {
		case '"':
			inString = true
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
		case ',':
			if depth == 0 {
				parts = append(parts, strings.TrimSpace(input[start:index]))
				start = index + 1
			}
		}
	}
	parts = append(parts, strings.TrimSpace(input[start:]))
	return parts
}

func matchingDelimiter(input string, start int, open, close byte) (int, bool) {
	if start >= len(input) || input[start] != open {
		return 0, false
	}
	depth := 0
	inString := false
	escaped := false
	for index := start; index < len(input); index++ {
		character := input[index]
		if inString {
			if escaped {
				escaped = false
				continue
			}
			if character == '\\' {
				escaped = true
			} else if character == '"' {
				inString = false
			}
			continue
		}
		if character == '"' {
			inString = true
			continue
		}
		if character == open {
			depth++
		} else if character == close {
			depth--
			if depth == 0 {
				return index, true
			}
		}
	}
	return 0, false
}

func nextStatement(input string) string {
	if index := strings.Index(input, ";"); index >= 0 {
		return input[:index+1]
	}
	return input
}

func lineNumber(input string, offset int) int {
	return strings.Count(input[:offset], "\n") + 1
}

func sha256Hex(data []byte) string {
	sum := sha256.Sum256(data)
	return hex.EncodeToString(sum[:])
}
