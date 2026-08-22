// generate_go_event_catalog extracts typed webhook registrations from the Go SDK.
package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

const (
	defaultEventRevision = "v3.10.0"
	defaultEventOutput   = "tools/go_event_catalog.json"
)

type eventCatalogSource struct {
	SDK      string `json:"sdk"`
	Revision string `json:"revision"`
	Commit   string `json:"commit"`
	Glob     string `json:"glob"`
}

type goEventRegistration struct {
	DispatcherFile string `json:"dispatcher_file"`
	Line           int    `json:"line"`
	Method         string `json:"method"`
	Package        string `json:"package"`
	Payload        string `json:"payload"`
}

type goEvent struct {
	Protocol  string              `json:"protocol"`
	EventKeys []string            `json:"event_keys"`
	Family    string              `json:"family"`
	Version   string              `json:"version"`
	Go        goEventRegistration `json:"go"`
}

func (event goEvent) identity() string {
	return event.Go.DispatcherFile + ":" + event.Go.Method
}

type goEventCatalog struct {
	SchemaVersion     int                `json:"schema_version"`
	Source            eventCatalogSource `json:"source"`
	RegistrationCount int                `json:"registration_count"`
	EventKeyCount     int                `json:"event_key_count"`
	Events            []goEvent          `json:"events"`
}

func main() {
	goSDK := flag.String("go-sdk", "", "path to a larksuite-oapi-sdk-go checkout")
	revision := flag.String("revision", defaultEventRevision, "Go SDK revision or tag")
	output := flag.String("output", defaultEventOutput, "generated catalog path")
	check := flag.Bool("check", false, "fail when the generated output is stale")
	flag.Parse()

	if *goSDK == "" {
		fail(errors.New("--go-sdk is required"))
	}

	generated, err := generateEventCatalog(*goSDK, *revision)
	if err != nil {
		fail(err)
	}
	if *check {
		existing, err := os.ReadFile(*output)
		if err != nil {
			fail(fmt.Errorf("read %s: %w", *output, err))
		}
		if !bytes.Equal(existing, generated) {
			fail(fmt.Errorf("%s is stale; rerun this command without --check", *output))
		}
		return
	}
	if existing, err := os.ReadFile(*output); err == nil && bytes.Equal(existing, generated) {
		return
	}
	if err := os.WriteFile(*output, generated, 0o644); err != nil {
		fail(fmt.Errorf("write %s: %w", *output, err))
	}
}

func fail(err error) {
	fmt.Fprintln(os.Stderr, "generate_go_event_catalog:", err)
	os.Exit(1)
}

func generateEventCatalog(goSDK, revision string) ([]byte, error) {
	commit, err := eventGitLine(goSDK, "rev-parse", revision+"^{commit}")
	if err != nil {
		return nil, fmt.Errorf("resolve %s: %w", revision, err)
	}
	files, err := eventDispatcherFilesAt(goSDK, revision)
	if err != nil {
		return nil, err
	}

	var events []goEvent
	for _, name := range files {
		source, err := eventGitShow(goSDK, revision+":"+name)
		if err != nil {
			return nil, fmt.Errorf("read %s at %s: %w", name, revision, err)
		}
		parsed, err := parseGoEventDispatcher(name, source)
		if err != nil {
			return nil, err
		}
		events = append(events, parsed...)
	}
	if err := validateAndSortGoEvents(events); err != nil {
		return nil, err
	}

	keys := make(map[string]struct{})
	for _, event := range events {
		for _, key := range event.EventKeys {
			keys[key] = struct{}{}
		}
	}
	result := goEventCatalog{
		SchemaVersion: 1,
		Source: eventCatalogSource{
			SDK: "larksuite-oapi-sdk-go", Revision: revision, Commit: commit,
			Glob: "event/dispatcher/*_event_dispatch.go",
		},
		RegistrationCount: len(events), EventKeyCount: len(keys), Events: events,
	}
	var output bytes.Buffer
	encoder := json.NewEncoder(&output)
	encoder.SetEscapeHTML(false)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(result); err != nil {
		return nil, fmt.Errorf("encode catalog: %w", err)
	}
	return output.Bytes(), nil
}

func eventDispatcherFilesAt(goSDK, revision string) ([]string, error) {
	files, err := eventGitLines(goSDK, "ls-tree", "-r", "--name-only", revision, "--", "event/dispatcher")
	if err != nil {
		return nil, fmt.Errorf("list dispatcher files at %s: %w", revision, err)
	}
	var result []string
	for _, name := range files {
		if strings.HasPrefix(name, "event/dispatcher/") && strings.HasSuffix(name, "_event_dispatch.go") {
			result = append(result, name)
		}
	}
	if len(result) == 0 {
		return nil, fmt.Errorf("no typed dispatcher files at %s", revision)
	}
	sort.Strings(result)
	return result, nil
}

func parseGoEventDispatcher(dispatcherFile string, source []byte) ([]goEvent, error) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, dispatcherFile, source, 0)
	if err != nil {
		return nil, fmt.Errorf("parse %s: %w", dispatcherFile, err)
	}
	imports := eventImports(file)
	var events []goEvent
	for _, declaration := range file.Decls {
		function, ok := declaration.(*ast.FuncDecl)
		if !ok || function.Body == nil {
			continue
		}
		protocol := eventProtocol(function.Name.Name)
		if protocol == "" {
			continue
		}
		packageName, payload, ok := typedPayload(function)
		if !ok {
			continue
		}
		packagePath, ok := imports[packageName]
		if !ok {
			packagePath, ok = imports[""]
		}
		if !ok {
			continue
		}
		family, version, ok := eventFamilyVersion(packagePath)
		if !ok {
			continue
		}
		keys := assignedEventKeys(function.Body)
		if len(keys) == 0 {
			return nil, fmt.Errorf("no event keys in %s function %s", dispatcherFile, function.Name.Name)
		}
		events = append(events, goEvent{
			Protocol: protocol, EventKeys: keys, Family: family, Version: version,
			Go: goEventRegistration{
				DispatcherFile: dispatcherFile,
				Line:           fset.Position(function.Pos()).Line,
				Method:         function.Name.Name,
				Package:        packagePath,
				Payload:        payload,
			},
		})
	}
	return events, nil
}

func eventImports(file *ast.File) map[string]string {
	imports := make(map[string]string)
	var fallback string
	for _, spec := range file.Imports {
		path, err := strconv.Unquote(spec.Path.Value)
		if err != nil || !strings.Contains(path, "/service/") {
			continue
		}
		if spec.Name != nil {
			imports[spec.Name.Name] = path
		}
		if fallback == "" {
			fallback = path
		}
	}
	if fallback != "" {
		imports[""] = fallback
	}
	return imports
}

func eventProtocol(method string) string {
	if strings.HasPrefix(method, "OnP1") {
		return "p1"
	}
	if strings.HasPrefix(method, "OnP2") {
		return "p2"
	}
	return ""
}

func typedPayload(function *ast.FuncDecl) (string, string, bool) {
	if function.Type.Params == nil || len(function.Type.Params.List) != 1 {
		return "", "", false
	}
	handler, ok := function.Type.Params.List[0].Type.(*ast.FuncType)
	if !ok || handler.Params == nil || len(handler.Params.List) != 2 {
		return "", "", false
	}
	pointer, ok := handler.Params.List[1].Type.(*ast.StarExpr)
	if !ok {
		return "", "", false
	}
	selector, ok := pointer.X.(*ast.SelectorExpr)
	if !ok {
		return "", "", false
	}
	packageName, ok := selector.X.(*ast.Ident)
	if !ok {
		return "", "", false
	}
	return packageName.Name, selector.Sel.Name, true
}

func eventFamilyVersion(packagePath string) (string, string, bool) {
	parts := strings.Split(packagePath, "/")
	if len(parts) < 3 || parts[len(parts)-3] != "service" {
		return "", "", false
	}
	return parts[len(parts)-2], parts[len(parts)-1], true
}

func assignedEventKeys(body *ast.BlockStmt) []string {
	keys := make(map[string]struct{})
	ast.Inspect(body, func(node ast.Node) bool {
		assignment, ok := node.(*ast.AssignStmt)
		if !ok {
			return true
		}
		for _, left := range assignment.Lhs {
			index, ok := left.(*ast.IndexExpr)
			if !ok || !isEventHandlerMap(index.X) {
				continue
			}
			if key, ok := astStringLiteral(index.Index); ok {
				keys[key] = struct{}{}
			}
		}
		return true
	})
	result := make([]string, 0, len(keys))
	for key := range keys {
		result = append(result, key)
	}
	sort.Strings(result)
	return result
}

func isEventHandlerMap(expression ast.Expr) bool {
	selector, ok := expression.(*ast.SelectorExpr)
	return ok && selector.Sel.Name == "eventType2EventHandler"
}

func astStringLiteral(expression ast.Expr) (string, bool) {
	literal, ok := expression.(*ast.BasicLit)
	if !ok || literal.Kind != token.STRING {
		return "", false
	}
	value, err := strconv.Unquote(literal.Value)
	return value, err == nil
}

func validateAndSortGoEvents(events []goEvent) error {
	if len(events) == 0 {
		return errors.New("no typed webhook registrations found")
	}
	seenMethods := make(map[string]struct{}, len(events))
	seenKeys := make(map[string]struct{})
	for _, event := range events {
		if _, exists := seenMethods[event.identity()]; exists {
			return fmt.Errorf("duplicate typed webhook registration %s", event.identity())
		}
		seenMethods[event.identity()] = struct{}{}
		for _, key := range event.EventKeys {
			if _, exists := seenKeys[key]; exists {
				return fmt.Errorf("duplicate Go webhook key %s", key)
			}
			seenKeys[key] = struct{}{}
		}
	}
	sort.Slice(events, func(i, j int) bool { return events[i].identity() < events[j].identity() })
	return nil
}

func eventGitLines(dir string, args ...string) ([]string, error) {
	output, err := eventGit(dir, args...)
	if err != nil {
		return nil, err
	}
	return strings.Fields(string(output)), nil
}

func eventGitLine(dir string, args ...string) (string, error) {
	output, err := eventGit(dir, args...)
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(output)), nil
}

func eventGitShow(dir, object string) ([]byte, error) { return eventGit(dir, "show", object) }

func eventGit(dir string, args ...string) ([]byte, error) {
	command := exec.Command("git", args...)
	command.Dir = filepath.Clean(dir)
	output, err := command.CombinedOutput()
	if err != nil {
		return nil, fmt.Errorf("git %s: %w\n%s", strings.Join(args, " "), err, output)
	}
	return output, nil
}
