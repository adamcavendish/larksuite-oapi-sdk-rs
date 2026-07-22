// generate_go_service_catalog extracts the full Go SDK service endpoint catalog.
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
	defaultRevision = "v3.9.9"
	defaultOutput   = "tools/go_service_catalog.json"
)

type endpoint struct {
	ResourceFile string   `json:"resource_file"`
	Receiver     string   `json:"receiver"`
	Operation    string   `json:"operation"`
	Method       string   `json:"method"`
	Path         string   `json:"path"`
	TokenTypes   []string `json:"token_types"`
	FileUpload   bool     `json:"file_upload"`
}

func (endpoint endpoint) identity() string {
	return endpoint.ResourceFile + ":" + endpoint.Receiver + "." + endpoint.Operation
}

type catalogSource struct {
	SDK      string `json:"sdk"`
	Revision string `json:"revision"`
	Commit   string `json:"commit"`
	Glob     string `json:"glob"`
}

type catalog struct {
	SchemaVersion int           `json:"schema_version"`
	Source        catalogSource `json:"source"`
	EndpointCount int           `json:"endpoint_count"`
	Endpoints     []endpoint    `json:"endpoints"`
}

func main() {
	goSDK := flag.String("go-sdk", "", "path to a larksuite-oapi-sdk-go checkout")
	revision := flag.String("revision", defaultRevision, "Go SDK revision or tag")
	output := flag.String("output", defaultOutput, "generated catalog path")
	check := flag.Bool("check", false, "fail when the generated output is stale")
	flag.Parse()

	if *goSDK == "" {
		fail(errors.New("--go-sdk is required"))
	}

	generated, err := generate(*goSDK, *revision)
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
	fmt.Fprintln(os.Stderr, "generate_go_service_catalog:", err)
	os.Exit(1)
}

func generate(goSDK, revision string) ([]byte, error) {
	commit, err := gitLine(goSDK, "rev-parse", revision+"^{commit}")
	if err != nil {
		return nil, fmt.Errorf("resolve %s: %w", revision, err)
	}
	resourceFiles, err := resourceFilesAt(goSDK, revision)
	if err != nil {
		return nil, err
	}

	var endpoints []endpoint
	for _, resourceFile := range resourceFiles {
		source, err := gitShow(goSDK, revision+":"+resourceFile)
		if err != nil {
			return nil, fmt.Errorf("read %s at %s: %w", resourceFile, revision, err)
		}
		parsed, err := parseResource(resourceFile, source)
		if err != nil {
			return nil, err
		}
		endpoints = append(endpoints, parsed...)
	}
	if err := validateAndSort(endpoints); err != nil {
		return nil, err
	}

	result := catalog{
		SchemaVersion: 1,
		Source: catalogSource{
			SDK:      "larksuite-oapi-sdk-go",
			Revision: revision,
			Commit:   commit,
			Glob:     "service/*/*/resource.go",
		},
		EndpointCount: len(endpoints),
		Endpoints:     endpoints,
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

func resourceFilesAt(goSDK, revision string) ([]string, error) {
	files, err := gitLines(goSDK, "ls-tree", "-r", "--name-only", revision, "--", "service")
	if err != nil {
		return nil, fmt.Errorf("list resource files at %s: %w", revision, err)
	}
	resourceFiles := make([]string, 0, len(files))
	for _, file := range files {
		if isResourceFile(file) {
			resourceFiles = append(resourceFiles, file)
		}
	}
	if len(resourceFiles) == 0 {
		return nil, fmt.Errorf("no service resource files at %s", revision)
	}
	sort.Strings(resourceFiles)
	return resourceFiles, nil
}

func isResourceFile(name string) bool {
	parts := strings.Split(name, "/")
	return len(parts) == 4 && parts[0] == "service" && parts[3] == "resource.go"
}

func parseResource(resourceFile string, source []byte) ([]endpoint, error) {
	file, err := parser.ParseFile(token.NewFileSet(), resourceFile, source, 0)
	if err != nil {
		return nil, fmt.Errorf("parse %s: %w", resourceFile, err)
	}

	var endpoints []endpoint
	for _, declaration := range file.Decls {
		function, ok := declaration.(*ast.FuncDecl)
		if !ok || function.Body == nil {
			continue
		}
		endpoint, found, err := endpointFromFunction(resourceFile, function)
		if err != nil {
			return nil, err
		}
		if found {
			endpoints = append(endpoints, endpoint)
		}
	}
	return endpoints, nil
}

func endpointFromFunction(resourceFile string, function *ast.FuncDecl) (endpoint, bool, error) {
	var result endpoint
	result.ResourceFile = resourceFile
	result.Operation = function.Name.Name
	result.Receiver = receiverName(function)
	var hasPath, hasMethod, hasTokens bool

	ast.Inspect(function.Body, func(node ast.Node) bool {
		assignment, ok := node.(*ast.AssignStmt)
		if !ok {
			return true
		}
		for index, left := range assignment.Lhs {
			field := selectorName(left)
			if field == "" || index >= len(assignment.Rhs) {
				continue
			}
			right := assignment.Rhs[index]
			switch field {
			case "ApiPath":
				path, ok := stringLiteral(right)
				if ok {
					result.Path = path
					hasPath = true
				}
			case "HttpMethod":
				method, ok := httpMethod(right)
				if ok {
					result.Method = method
					hasMethod = true
				}
			case "SupportedAccessTokenTypes":
				tokens, ok := accessTokenTypes(right)
				if ok {
					result.TokenTypes = tokens
					hasTokens = true
				}
			}
		}
		return true
	})

	ast.Inspect(function.Body, func(node ast.Node) bool {
		call, ok := node.(*ast.CallExpr)
		if ok && expressionName(call.Fun) == "WithFileUpload" {
			result.FileUpload = true
		}
		return true
	})

	if !hasPath && !hasMethod && !hasTokens {
		return endpoint{}, false, nil
	}
	if !hasPath || !hasMethod || !hasTokens || result.Receiver == "" {
		return endpoint{}, false, fmt.Errorf("incomplete endpoint metadata in %s function %s", resourceFile, function.Name.Name)
	}
	return result, true, nil
}

func receiverName(function *ast.FuncDecl) string {
	if function.Recv == nil || len(function.Recv.List) != 1 {
		return ""
	}
	return typeName(function.Recv.List[0].Type)
}

func typeName(expression ast.Expr) string {
	switch expression := expression.(type) {
	case *ast.Ident:
		return expression.Name
	case *ast.StarExpr:
		return typeName(expression.X)
	default:
		return ""
	}
}

func selectorName(expression ast.Expr) string {
	selector, ok := expression.(*ast.SelectorExpr)
	if !ok {
		return ""
	}
	return selector.Sel.Name
}

func stringLiteral(expression ast.Expr) (string, bool) {
	literal, ok := expression.(*ast.BasicLit)
	if !ok || literal.Kind != token.STRING {
		return "", false
	}
	value, err := strconv.Unquote(literal.Value)
	return value, err == nil
}

func httpMethod(expression ast.Expr) (string, bool) {
	name := selectorName(expression)
	if !strings.HasPrefix(name, "Method") {
		return "", false
	}
	method := strings.TrimPrefix(name, "Method")
	switch method {
	case "Get", "Post", "Put", "Patch", "Delete", "Head", "Options":
		return strings.ToUpper(method), true
	default:
		return "", false
	}
}

func accessTokenTypes(expression ast.Expr) ([]string, bool) {
	list, ok := expression.(*ast.CompositeLit)
	if !ok {
		return nil, false
	}
	tokens := make([]string, 0, len(list.Elts))
	for _, element := range list.Elts {
		switch expressionName(element) {
		case "AccessTokenTypeApp":
			tokens = append(tokens, "App")
		case "AccessTokenTypeTenant":
			tokens = append(tokens, "Tenant")
		case "AccessTokenTypeUser":
			tokens = append(tokens, "User")
		default:
			return nil, false
		}
	}
	return tokens, true
}

func expressionName(expression ast.Expr) string {
	if name := selectorName(expression); name != "" {
		return name
	}
	identifier, ok := expression.(*ast.Ident)
	if !ok {
		return ""
	}
	return identifier.Name
}

func validateAndSort(endpoints []endpoint) error {
	if len(endpoints) == 0 {
		return errors.New("no endpoints found")
	}
	identities := make(map[string]struct{}, len(endpoints))
	for _, endpoint := range endpoints {
		identity := endpoint.identity()
		if _, exists := identities[identity]; exists {
			return fmt.Errorf("duplicate endpoint identity %s", identity)
		}
		identities[identity] = struct{}{}
	}
	sort.Slice(endpoints, func(i, j int) bool {
		return endpoints[i].identity() < endpoints[j].identity()
	})
	return nil
}

func gitLines(dir string, args ...string) ([]string, error) {
	output, err := git(dir, args...)
	if err != nil {
		return nil, err
	}
	return strings.Fields(string(output)), nil
}

func gitLine(dir string, args ...string) (string, error) {
	output, err := git(dir, args...)
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(output)), nil
}

func gitShow(dir, object string) ([]byte, error) {
	return git(dir, "show", object)
}

func git(dir string, args ...string) ([]byte, error) {
	command := exec.Command("git", args...)
	command.Dir = filepath.Clean(dir)
	output, err := command.CombinedOutput()
	if err != nil {
		return nil, fmt.Errorf("git %s: %w\n%s", strings.Join(args, " "), err, output)
	}
	return output, nil
}
