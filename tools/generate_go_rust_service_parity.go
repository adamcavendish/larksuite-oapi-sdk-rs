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
	defaultCatalog   = "tools/go_service_catalog.json"
	defaultOmissions = "tools/go_rust_service_parity_omissions.json"
	defaultOutput    = "tools/go_rust_service_parity.json"
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

type omissionCatalog struct {
	SchemaVersion int               `json:"schema_version"`
	Omissions     []serviceOmission `json:"omissions"`
}

type serviceOmission struct {
	ResourceFile string `json:"resource_file"`
	Receiver     string `json:"receiver"`
	Operation    string `json:"operation"`
	Reason       string `json:"reason"`
	Source       string `json:"source"`
}

func (omission serviceOmission) identity() string {
	return omission.ResourceFile + ":" + omission.Receiver + "." + omission.Operation
}

type omittedContract struct {
	Contract goEndpoint `json:"contract"`
	Reason   string     `json:"reason"`
	Source   string     `json:"source"`
}

type paritySummary struct {
	GoContracts                    int `json:"go_contracts"`
	RustDirectContracts            int `json:"rust_direct_contracts"`
	GoCompatibilityBridgeContracts int `json:"go_compatibility_bridge_contracts"`
	TypedMatches                   int `json:"typed_matches"`
	BridgeMatches                  int `json:"bridge_matches"`
	MetadataMismatches             int `json:"metadata_mismatches"`
	MissingContracts               int `json:"missing_contracts"`
	IntentionalOmissions           int `json:"intentional_omissions"`
	UnparsedRustRequests           int `json:"unparsed_rust_requests"`
}

type parityReport struct {
	SchemaVersion        int                   `json:"schema_version"`
	GoCatalogSHA256      string                `json:"go_catalog_sha256"`
	Summary              paritySummary         `json:"summary"`
	MetadataMismatches   []metadataMismatch    `json:"metadata_mismatches"`
	MissingContracts     []goEndpoint          `json:"missing_contracts"`
	IntentionalOmissions []omittedContract     `json:"intentional_omissions"`
	UnparsedRustRequests []unparsedRustRequest `json:"unparsed_rust_requests"`
}

var functionStart = regexp.MustCompile(`(?m)^\s*(pub(\([^)]*\))?\s+)?(async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*`)
var methodPattern = regexp.MustCompile(`http::Method::([A-Za-z]+)`)
var literalMethod = regexp.MustCompile(`^http::Method::([A-Za-z]+)$`)
var tokenPattern = regexp.MustCompile(`AccessTokenType::([A-Za-z]+)`)
var pathVariablePattern = regexp.MustCompile(`(?s)let\s+(mut\s+)?path\s*=\s*(.*?);`)
var macroInvocationStart = regexp.MustCompile(`(?m)^\s*(post_method|post_method_with_tokens|get_method|post_query|external_crud_resource)!\s*\(`)

type sourceRange struct {
	start int
	end   int
}

func main() {
	goCatalogPath := flag.String("go-catalog", defaultCatalog, "path to the generated Go service contract catalog")
	omissionsPath := flag.String("omissions", defaultOmissions, "path to documented intentional Go contract omissions")
	rustSDK := flag.String("rust-sdk", ".", "path to the Rust SDK checkout")
	output := flag.String("output", defaultOutput, "generated parity report path")
	check := flag.Bool("check", false, "fail when the generated report is stale")
	flag.Parse()

	generated, err := generate(*goCatalogPath, *omissionsPath, *rustSDK)
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

func generate(goCatalogPath, omissionsPath, rustSDK string) ([]byte, error) {
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
	omissions, err := loadOmissions(omissionsPath)
	if err != nil {
		return nil, err
	}

	direct, unparsed, err := rustServiceContracts(rustSDK)
	if err != nil {
		return nil, err
	}
	bridge, err := goCompatibilityContracts(rustSDK)
	if err != nil {
		return nil, err
	}

	directByRoute := contractsByRoute(direct)
	bridgeByRoute := contractsByRoute(bridge)
	report := parityReport{
		SchemaVersion:   3,
		GoCatalogSHA256: sha256Hex(goCatalogBytes),
		Summary: paritySummary{
			GoContracts:                    len(catalog.Endpoints),
			RustDirectContracts:            len(direct),
			GoCompatibilityBridgeContracts: len(bridge),
		},
		UnparsedRustRequests: unparsed,
	}

	matchedOmissions := make(map[string]bool, len(omissions))
	for _, endpoint := range catalog.Endpoints {
		if omission, ok := omissions[endpoint.identity()]; ok {
			report.IntentionalOmissions = append(report.IntentionalOmissions, omittedContract{
				Contract: endpoint,
				Reason:   omission.Reason,
				Source:   omission.Source,
			})
			matchedOmissions[endpoint.identity()] = true
			continue
		}
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
	for identity := range omissions {
		if !matchedOmissions[identity] {
			return nil, fmt.Errorf("intentional omission %s is not in the Go catalog", identity)
		}
	}

	report.Summary.MetadataMismatches = len(report.MetadataMismatches)
	report.Summary.MissingContracts = len(report.MissingContracts)
	report.Summary.IntentionalOmissions = len(report.IntentionalOmissions)
	report.Summary.UnparsedRustRequests = len(report.UnparsedRustRequests)
	sort.Slice(report.MetadataMismatches, func(i, j int) bool {
		return report.MetadataMismatches[i].Contract.identity() < report.MetadataMismatches[j].Contract.identity()
	})
	sort.Slice(report.MissingContracts, func(i, j int) bool {
		return report.MissingContracts[i].identity() < report.MissingContracts[j].identity()
	})
	sort.Slice(report.IntentionalOmissions, func(i, j int) bool {
		return report.IntentionalOmissions[i].Contract.identity() < report.IntentionalOmissions[j].Contract.identity()
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

func loadOmissions(path string) (map[string]serviceOmission, error) {
	contents, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("read intentional omissions: %w", err)
	}
	var catalog omissionCatalog
	if err := json.Unmarshal(contents, &catalog); err != nil {
		return nil, fmt.Errorf("decode intentional omissions: %w", err)
	}
	if catalog.SchemaVersion != 1 {
		return nil, fmt.Errorf("intentional omissions schema version %d is unsupported", catalog.SchemaVersion)
	}
	omissions := make(map[string]serviceOmission, len(catalog.Omissions))
	for _, omission := range catalog.Omissions {
		if omission.ResourceFile == "" || omission.Receiver == "" || omission.Operation == "" || omission.Reason == "" || omission.Source == "" {
			return nil, errors.New("intentional omissions require resource_file, receiver, operation, reason, and source")
		}
		identity := omission.identity()
		if _, exists := omissions[identity]; exists {
			return nil, fmt.Errorf("duplicate intentional omission %s", identity)
		}
		omissions[identity] = omission
	}
	return omissions, nil
}

func rustServiceContracts(rustSDK string) ([]rustEndpoint, []unparsedRustRequest, error) {
	serviceRoot := filepath.Join(rustSDK, "src", "service")
	var contracts []rustEndpoint
	var unparsed []unparsedRustRequest
	sources := make(map[string]string)
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
		sources[filepath.ToSlash(relative)] = string(source)
		return nil
	})
	if err != nil {
		return nil, nil, fmt.Errorf("read Rust service contracts: %w", err)
	}
	helpers := make(map[string]rustRequestHelper)
	files := make([]string, 0, len(sources))
	for file := range sources {
		files = append(files, file)
	}
	sort.Strings(files)
	for _, file := range files {
		source := sources[file]
		functions := rustFunctions(source, macroRuleRanges(source))
		tokenHelpers := rustTokenHelpers(source, functions)
		for _, function := range functions {
			if function.owner == "" {
				if helper, ok := requestHelper(source, function, tokenHelpers); ok {
					helpers[rustModulePath(file)+"::"+function.name] = helper
				}
			}
		}
	}
	for _, file := range files {
		parsed, unresolved := extractRustContractsWithHelpers(file, sources[file], helpers)
		contracts = append(contracts, parsed...)
		unparsed = append(unparsed, unresolved...)
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
	return name == "common.rs" || name == "go_compatibility.rs" || name == "go_compatibility_metadata.rs"
}

type rustFunction struct {
	name       string
	owner      string
	parameters []string
	bodyStart  int
	bodyEnd    int
}

type rustRequestHelper struct {
	function rustFunction
	args     []string
	bindings map[int]int // RestRequest argument -> helper parameter.
	upload   bool
}

var implStart = regexp.MustCompile(`\bimpl(?:<[^>]*>)?\s+([A-Za-z_][A-Za-z0-9_]*)[^{};]*\{`)
var rustCallStart = regexp.MustCompile(`\b[A-Za-z_][A-Za-z0-9_]*(?:(?:::|\.)[A-Za-z_][A-Za-z0-9_]*)*\s*\(`)
var literalTokenTypes = regexp.MustCompile(`(?s)^vec!\[\s*(?:AccessTokenType::[A-Za-z]+\s*,\s*)*(?:AccessTokenType::[A-Za-z]+\s*)?\]$`)

func rustFunctions(source string, macroDefinitions []sourceRange) []rustFunction {
	var functions []rustFunction
	type implScope struct {
		owner      string
		start, end int
	}
	var scopes []implScope
	for _, match := range implStart.FindAllStringSubmatchIndex(source, -1) {
		if inSourceRanges(match[0], macroDefinitions) {
			continue
		}
		if end, ok := matchingDelimiter(source, match[1]-1, '{', '}'); ok {
			scopes = append(scopes, implScope{source[match[2]:match[3]], match[1] - 1, end})
		}
	}
	for _, match := range functionStart.FindAllStringSubmatchIndex(source, -1) {
		if inSourceRanges(match[0], macroDefinitions) {
			continue
		}
		parametersStart := strings.Index(source[match[1]:], "(") + match[1]
		parametersEnd, ok := matchingDelimiter(source, parametersStart, '(', ')')
		if !ok {
			continue
		}
		bodyStart := strings.Index(source[parametersEnd+1:], "{")
		if bodyStart < 0 {
			continue
		}
		bodyStart += parametersEnd + 1
		bodyEnd, ok := matchingDelimiter(source, bodyStart, '{', '}')
		if !ok {
			continue
		}
		function := rustFunction{name: source[match[8]:match[9]], bodyStart: bodyStart, bodyEnd: bodyEnd}
		for _, parameter := range splitTopLevel(source[parametersStart+1 : parametersEnd]) {
			if colon := strings.Index(parameter, ":"); colon >= 0 {
				function.parameters = append(function.parameters, strings.TrimSpace(strings.TrimPrefix(parameter[:colon], "mut ")))
			}
		}
		for _, scope := range scopes {
			if scope.start < bodyStart && bodyStart < scope.end {
				function.owner = scope.owner
			}
		}
		functions = append(functions, function)
	}
	return functions
}

func (function rustFunction) key() string {
	if function.owner != "" {
		return function.owner + "::" + function.name
	}
	return function.name
}

// Token-list helpers are statically known only when their entire body is a
// literal vector. Other computations remain unresolved.
func rustTokenHelpers(source string, functions []rustFunction) map[string]string {
	helpers := make(map[string]string)
	for _, function := range functions {
		body := strings.TrimSpace(source[function.bodyStart+1 : function.bodyEnd])
		if function.owner == "" && len(function.parameters) == 0 && literalTokenTypes.MatchString(body) {
			helpers[function.name+"()"] = body
		}
	}
	return helpers
}

// A helper must forward a method, path or token parameter to one constructor.
// Metadata literals come from that constructor, never from a service name.
func requestHelper(source string, function rustFunction, tokenHelpers map[string]string) (rustRequestHelper, bool) {
	body := source[function.bodyStart : function.bodyEnd+1]
	const constructor = "RestRequest::new("
	start := strings.Index(body, constructor)
	if start < 0 || strings.Count(body, constructor) != 1 {
		return rustRequestHelper{}, false
	}
	open := start + len(constructor) - 1
	close, ok := matchingDelimiter(body, open, '(', ')')
	if !ok {
		return rustRequestHelper{}, false
	}
	args := splitTopLevel(body[open+1 : close])
	if len(args) < 4 {
		return rustRequestHelper{}, false
	}
	if tokens, ok := tokenHelpers[args[3]]; ok {
		args[3] = tokens
	}
	helper := rustRequestHelper{function: function, args: args, bindings: make(map[int]int), upload: rustFileUpload(nextStatement(body[close:]))}
	for _, argument := range []int{1, 2, 3} {
		bound := false
		for index, parameter := range function.parameters {
			if args[argument] == parameter {
				helper.bindings[argument] = index
				bound = true
				break
			}
		}
		if bound {
			continue
		}
		switch argument {
		case 1:
			if !literalMethod.MatchString(args[argument]) {
				return rustRequestHelper{}, false
			}
		case 2:
			path, ok := rustPath(args[argument], body[:start])
			if !ok {
				return rustRequestHelper{}, false
			}
			helper.args[argument] = strconv.Quote(path)
		case 3:
			if !literalTokenTypes.MatchString(args[argument]) {
				return rustRequestHelper{}, false
			}
		}
	}
	return helper, len(helper.bindings) > 0
}

// Module paths are used only for Rust name resolution, not service-specific rules.
func rustModulePath(sourceFile string) string {
	path := strings.TrimSuffix(strings.TrimPrefix(filepath.ToSlash(sourceFile), "src/"), ".rs")
	path = strings.TrimSuffix(path, "/mod")
	return strings.ReplaceAll(path, "/", "::")
}

func resolveRustPath(module, path string) string {
	parts := strings.Split(path, "::")
	var prefix []string
	switch parts[0] {
	case "crate":
		parts = parts[1:]
	case "self":
		prefix = strings.Split(module, "::")
		parts = parts[1:]
	default:
		prefix = strings.Split(module, "::")
	}
	for len(parts) > 0 && parts[0] == "super" {
		if len(prefix) == 0 {
			return ""
		}
		prefix = prefix[:len(prefix)-1]
		parts = parts[1:]
	}
	return strings.Join(append(prefix, parts...), "::")
}

var rustUse = regexp.MustCompile(`\buse\s+([^;]+);`)

func rustImports(sourceFile, source string) (map[string]string, []string) {
	imports := make(map[string]string)
	var globs []string
	var add func(string)
	add = func(path string) {
		path = strings.TrimSpace(path)
		if open := strings.Index(path, "{"); open >= 0 {
			close, ok := matchingDelimiter(path, open, '{', '}')
			if ok {
				for _, child := range splitTopLevel(path[open+1 : close]) {
					if strings.TrimSpace(child) != "" {
						add(path[:open] + strings.TrimSpace(child))
					}
				}
			}
			return
		}
		alias := ""
		if parts := strings.Split(path, " as "); len(parts) == 2 {
			path, alias = strings.TrimSpace(parts[0]), strings.TrimSpace(parts[1])
		}
		qualified := resolveRustPath(rustModulePath(sourceFile), path)
		if strings.HasSuffix(qualified, "::*") {
			globs = append(globs, strings.TrimSuffix(qualified, "::*"))
			return
		}
		if alias == "" {
			parts := strings.Split(qualified, "::")
			alias = parts[len(parts)-1]
		}
		imports[alias] = qualified
	}
	for _, match := range rustUse.FindAllStringSubmatch(source, -1) {
		add(match[1])
	}
	return imports, globs
}

func importedRequestHelper(sourceFile, name string, imports map[string]string, globs []string, helpers map[string]rustRequestHelper) (rustRequestHelper, bool) {
	parts := strings.Split(name, "::")
	if path, ok := imports[parts[0]]; ok {
		if len(parts) > 1 {
			path += "::" + strings.Join(parts[1:], "::")
		}
		helper, found := helpers[path]
		return helper, found
	}
	if helper, ok := helpers[resolveRustPath(rustModulePath(sourceFile), name)]; ok {
		return helper, true
	}
	for _, module := range globs {
		if helper, ok := helpers[module+"::"+name]; ok {
			return helper, true
		}
	}
	return rustRequestHelper{}, false
}

func extractRustContracts(sourceFile, source string) ([]rustEndpoint, []unparsedRustRequest) {
	return extractRustContractsWithHelpers(sourceFile, source, nil)
}

func extractRustContractsWithHelpers(sourceFile, source string, importedHelpers map[string]rustRequestHelper) ([]rustEndpoint, []unparsedRustRequest) {
	var contracts []rustEndpoint
	var unparsed []unparsedRustRequest
	macroDefinitions := macroRuleRanges(source)
	functions := rustFunctions(source, macroDefinitions)
	tokenHelpers := rustTokenHelpers(source, functions)
	imports, globs := rustImports(sourceFile, source)
	helpers := make(map[string]rustRequestHelper)
	for _, function := range functions {
		if helper, ok := requestHelper(source, function, tokenHelpers); ok {
			helpers[function.key()] = helper
		}
	}
	for _, function := range functions {
		if helper, ok := helpers[function.key()]; ok && helper.function.bodyStart == function.bodyStart {
			continue
		}
		body := source[function.bodyStart : function.bodyEnd+1]
		for _, call := range rustCallStart.FindAllStringIndex(body, -1) {
			open := call[1] - 1
			name := strings.TrimSpace(body[call[0]:open])
			key := name
			if strings.HasPrefix(name, "self.") {
				key = function.owner + "::" + strings.TrimPrefix(name, "self.")
			} else if strings.HasPrefix(name, "Self::") {
				key = function.owner + "::" + strings.TrimPrefix(name, "Self::")
			}
			helper, isHelper := helpers[key]
			if !isHelper {
				helper, isHelper = importedRequestHelper(sourceFile, name, imports, globs, importedHelpers)
			}
			if name != "RestRequest::new" && !isHelper {
				continue
			}
			line := lineNumber(source, function.bodyStart+call[0])
			close, ok := matchingDelimiter(body, open, '(', ')')
			if !ok {
				unparsed = append(unparsed, unparsedRustRequest{sourceFile, function.name, line, "unclosed request call"})
				continue
			}
			args := splitTopLevel(body[open+1 : close])
			upload := false
			reason := ""
			if isHelper {
				resolved := append([]string(nil), helper.args...)
				for argument, parameter := range helper.bindings {
					if parameter >= len(args) || strings.TrimSpace(args[parameter]) == "" {
						reason = "request helper call has too few arguments"
						break
					}
					resolved[argument] = args[parameter]
				}
				args = resolved
				if tokens, ok := tokenHelpers[args[3]]; ok {
					args[3] = tokens
				}
				if reason == "" && !literalTokenTypes.MatchString(args[3]) {
					reason = "non-literal request helper access token types"
				}
				upload = helper.upload
			}
			var endpoint rustEndpoint
			if len(args) >= 4 {
				if tokens, ok := tokenHelpers[args[3]]; ok {
					args[3] = tokens
				}
			}
			if reason == "" {
				endpoint, reason = endpointFromRustCall(sourceFile, function.name, line, body[:call[0]], args, body[close:])
			}
			if reason != "" {
				unparsed = append(unparsed, unparsedRustRequest{sourceFile, function.name, line, reason})
			} else {
				endpoint.FileUpload = endpoint.FileUpload || upload
				contracts = append(contracts, endpoint)
			}
		}
	}
	macroContracts, macroUnparsed := extractSupportedMacroContracts(sourceFile, source, macroDefinitions)
	contracts = append(contracts, macroContracts...)
	unparsed = append(unparsed, macroUnparsed...)
	return contracts, unparsed
}

func macroRuleRanges(source string) []sourceRange {
	starts := regexp.MustCompile(`macro_rules!\s+[A-Za-z_][A-Za-z0-9_]*\s*`).FindAllStringIndex(source, -1)
	ranges := make([]sourceRange, 0, len(starts))
	for _, match := range starts {
		open := strings.Index(source[match[1]:], "{")
		if open < 0 {
			continue
		}
		open += match[1]
		close, ok := matchingDelimiter(source, open, '{', '}')
		if ok {
			ranges = append(ranges, sourceRange{start: match[0], end: close + 1})
		}
	}
	return ranges
}

func inSourceRanges(offset int, ranges []sourceRange) bool {
	for _, sourceRange := range ranges {
		if sourceRange.start <= offset && offset < sourceRange.end {
			return true
		}
	}
	return false
}

func extractSupportedMacroContracts(sourceFile, source string, macroDefinitions []sourceRange) ([]rustEndpoint, []unparsedRustRequest) {
	var contracts []rustEndpoint
	var unparsed []unparsedRustRequest
	for _, match := range macroInvocationStart.FindAllStringSubmatchIndex(source, -1) {
		if inSourceRanges(match[0], macroDefinitions) {
			continue
		}
		name := source[match[2]:match[3]]
		open := strings.LastIndex(source[match[0]:match[1]], "(") + match[0]
		close, ok := matchingDelimiter(source, open, '(', ')')
		line := lineNumber(source, match[0])
		if !ok {
			unparsed = append(unparsed, unparsedRustRequest{sourceFile, name, line, "unclosed supported macro invocation"})
			continue
		}
		args := splitTopLevel(source[open+1 : close])
		parsed, reason := contractsFromSupportedMacro(sourceFile, name, line, args)
		if reason != "" {
			unparsed = append(unparsed, unparsedRustRequest{sourceFile, name, line, reason})
			continue
		}
		contracts = append(contracts, parsed...)
	}
	return contracts, unparsed
}

func contractsFromSupportedMacro(sourceFile, name string, line int, args []string) ([]rustEndpoint, string) {
	contract := func(function, method, path string, tokenTypes []string) rustEndpoint {
		return rustEndpoint{
			SourceFile: sourceFile,
			Function:   function,
			Line:       line,
			Method:     method,
			Path:       normalizePath(path),
			TokenTypes: tokenTypes,
		}
	}

	switch name {
	case "post_method", "get_method", "post_query":
		if len(args) == 0 {
			return nil, "supported macro invocation has no arguments"
		}
		path, ok := rustString(args[len(args)-1])
		if !ok {
			return nil, "non-literal supported macro request path"
		}
		method := "POST"
		if name == "get_method" {
			method = "GET"
		}
		return []rustEndpoint{contract(strings.TrimSpace(args[0]), method, path, []string{"Tenant"})}, ""
	case "post_method_with_tokens":
		if len(args) != 5 {
			return nil, "token-aware post macro invocation has an unexpected argument count"
		}
		path, ok := rustString(args[3])
		if !ok {
			return nil, "non-literal token-aware post macro request path"
		}
		return []rustEndpoint{contract(strings.TrimSpace(args[0]), "POST", path, rustTokenTypes(args[4]))}, ""
	case "external_crud_resource":
		if len(args) < 2 {
			return nil, "external CRUD macro invocation has fewer than two arguments"
		}
		basePath, ok := rustString(args[1])
		if !ok {
			return nil, "non-literal external CRUD macro base path"
		}
		return []rustEndpoint{
			contract("create", "POST", basePath, []string{"Tenant"}),
			contract("update", "PUT", basePath+"/{}", []string{"Tenant"}),
			contract("delete", "DELETE", basePath+"/{}", []string{"Tenant"}),
		}, ""
	default:
		return nil, "unsupported macro invocation"
	}
}

func endpointFromRustCall(sourceFile, function string, line int, prefix string, args []string, suffix string) (rustEndpoint, string) {
	if len(args) < 4 {
		return rustEndpoint{}, "RestRequest::new has fewer than four arguments"
	}
	methodMatch := literalMethod.FindStringSubmatch(args[1])
	if len(methodMatch) != 2 {
		return rustEndpoint{}, "non-literal HTTP method"
	}
	path, ok := rustPath(args[2], prefix)
	if !ok {
		return rustEndpoint{}, "non-literal request path"
	}
	if !literalTokenTypes.MatchString(args[3]) {
		return rustEndpoint{}, "non-literal access token types"
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

func goCompatibilityContracts(rustSDK string) ([]rustEndpoint, error) {
	path := filepath.Join(rustSDK, "src", "service", "go_compatibility_metadata.rs")
	source, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("read GoCompatibility metadata: %w", err)
	}
	blocks := regexp.MustCompile(`(?s)Self::[A-Za-z0-9_]+ => GoCompatibilityEndpointMeta \{(.*?)\n            \},`).FindAllStringSubmatch(string(source), -1)
	contracts := make([]rustEndpoint, 0, len(blocks))
	for _, block := range blocks {
		methodMatch := methodPattern.FindStringSubmatch(block[1])
		pathMatch := regexp.MustCompile(`path: "([^"]+)"`).FindStringSubmatch(block[1])
		if len(methodMatch) != 2 || len(pathMatch) != 2 {
			return nil, errors.New("incomplete GoCompatibility endpoint metadata")
		}
		contracts = append(contracts, rustEndpoint{
			SourceFile: "src/service/go_compatibility_metadata.rs",
			Function:   "GoCompatibilityEndpoint",
			Method:     strings.ToUpper(methodMatch[1]),
			Path:       normalizePath(pathMatch[1]),
			TokenTypes: rustTokenTypes(block[1]),
			FileUpload: strings.Contains(block[1], "file_upload: true"),
		})
	}
	if len(contracts) == 0 {
		return nil, errors.New("no GoCompatibility endpoint metadata found")
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
