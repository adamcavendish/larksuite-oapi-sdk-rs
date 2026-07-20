// generate_go_v397_metadata extracts the Go SDK endpoint delta used by GoV397.
package main

import (
	"bytes"
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
	defaultFrom   = "v3.6.1"
	defaultTo     = "v3.9.7"
	defaultOutput = "src/service/go_v397_metadata.rs"
)

type endpoint struct {
	method       string
	path         string
	tokens       []string
	fileUpload   bool
	resourceFile string
	order        int
}

func main() {
	goSDK := flag.String("go-sdk", "", "path to a larksuite-oapi-sdk-go checkout")
	from := flag.String("from", defaultFrom, "baseline Go SDK tag")
	to := flag.String("to", defaultTo, "target Go SDK tag")
	output := flag.String("output", defaultOutput, "generated Rust output path")
	check := flag.Bool("check", false, "fail when the generated output is stale")
	flag.Parse()

	if *goSDK == "" {
		fail(errors.New("--go-sdk is required"))
	}

	generated, err := generate(*goSDK, *from, *to)
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
	fmt.Fprintln(os.Stderr, "generate_go_v397_metadata:", err)
	os.Exit(1)
}

func generate(goSDK, from, to string) ([]byte, error) {
	resourceFiles, err := changedResourceFiles(goSDK, from, to)
	if err != nil {
		return nil, err
	}
	fromEndpoints, err := endpointsAt(goSDK, from, resourceFiles)
	if err != nil {
		return nil, err
	}
	toEndpoints, err := endpointsAt(goSDK, to, resourceFiles)
	if err != nil {
		return nil, err
	}

	var delta []endpoint
	for key, candidate := range toEndpoints {
		if _, exists := fromEndpoints[key]; !exists {
			delta = append(delta, candidate)
		}
	}
	if len(delta) == 0 {
		return nil, fmt.Errorf("no endpoints were added between %s and %s", from, to)
	}

	sort.Slice(delta, func(i, j int) bool { return delta[i].order < delta[j].order })
	return render(delta, from, to), nil
}

func changedResourceFiles(goSDK, from, to string) ([]string, error) {
	files, err := gitLines(goSDK, "diff", "--name-only", from, to, "--", "service")
	if err != nil {
		return nil, fmt.Errorf("list changed resource files between %s and %s: %w", from, to, err)
	}
	return filterResourceFiles(files), nil
}

func endpointsAt(goSDK, revision string, resourceFiles []string) (map[string]endpoint, error) {
	endpoints := make(map[string]endpoint)
	order := 0
	for _, resourceFile := range resourceFiles {
		exists, err := gitObjectExists(goSDK, revision+":"+resourceFile)
		if err != nil {
			return nil, fmt.Errorf("check %s at %s: %w", resourceFile, revision, err)
		}
		if !exists {
			continue
		}
		source, err := gitShow(goSDK, revision+":"+resourceFile)
		if err != nil {
			return nil, fmt.Errorf("read %s at %s: %w", resourceFile, revision, err)
		}
		parsed, err := parseResource(resourceFile, source)
		if err != nil {
			return nil, err
		}
		for _, endpoint := range parsed {
			endpoint.order = order
			order++
			key := endpoint.method + " " + endpoint.path
			if previous, exists := endpoints[key]; exists {
				return nil, fmt.Errorf("duplicate endpoint %s in %s and %s", key, previous.resourceFile, endpoint.resourceFile)
			}
			endpoints[key] = endpoint
		}
	}
	return endpoints, nil
}

func filterResourceFiles(files []string) []string {
	resourceFiles := make([]string, 0, len(files))
	for _, file := range files {
		if isResourceFile(file) {
			resourceFiles = append(resourceFiles, file)
		}
	}
	sort.Strings(resourceFiles)
	return resourceFiles
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
	result.resourceFile = resourceFile
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
					result.path = path
					hasPath = true
				}
			case "HttpMethod":
				method, ok := httpMethod(right)
				if ok {
					result.method = method
					hasMethod = true
				}
			case "SupportedAccessTokenTypes":
				tokens, ok := accessTokenTypes(right)
				if ok {
					result.tokens = tokens
					hasTokens = true
				}
			}
		}
		return true
	})

	ast.Inspect(function.Body, func(node ast.Node) bool {
		call, ok := node.(*ast.CallExpr)
		if ok && selectorName(call.Fun) == "WithFileUpload" {
			result.fileUpload = true
		}
		return true
	})

	if !hasPath && !hasMethod && !hasTokens {
		return endpoint{}, false, nil
	}
	if !hasPath || !hasMethod || !hasTokens {
		return endpoint{}, false, fmt.Errorf("incomplete endpoint metadata in %s function %s", resourceFile, function.Name.Name)
	}
	return result, true, nil
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
		switch selectorName(element) {
		case "AccessTokenTypeTenant":
			tokens = append(tokens, "Tenant")
		case "AccessTokenTypeUser":
			tokens = append(tokens, "User")
		default:
			return nil, false
		}
	}
	if len(tokens) == 0 {
		return nil, false
	}
	return tokens, true
}

func endpointName(endpoint endpoint) string {
	var name strings.Builder
	method := strings.ToLower(endpoint.method)
	name.WriteString(strings.ToUpper(method[:1]))
	name.WriteString(method[1:])
	for _, segment := range strings.Split(strings.TrimPrefix(endpoint.path, "/open-apis/"), "/") {
		if strings.HasPrefix(segment, ":") {
			name.WriteString("By")
			name.WriteString(pascalCase(strings.TrimPrefix(segment, ":")))
			continue
		}
		name.WriteString(pascalCase(segment))
	}
	return name.String()
}

func pascalCase(value string) string {
	parts := strings.FieldsFunc(value, func(r rune) bool {
		return r == '_' || r == '-'
	})
	for index, part := range parts {
		if part == "" {
			continue
		}
		parts[index] = strings.ToUpper(part[:1]) + part[1:]
	}
	return strings.Join(parts, "")
}

func render(endpoints []endpoint, from, to string) []byte {
	var output strings.Builder
	fmt.Fprintln(&output, "// Code generated by tools/generate_go_v397_metadata.go; DO NOT EDIT.")
	fmt.Fprintf(&output, "// Source: larksuite-oapi-sdk-go %s..%s service/*/*/resource.go.\n\n", from, to)
	fmt.Fprintln(&output, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
	fmt.Fprintln(&output, "#[non_exhaustive]")
	fmt.Fprintln(&output, "pub enum GoV397Endpoint {")
	for _, endpoint := range endpoints {
		fmt.Fprintf(&output, "    /// %s %s (%s)\n", endpoint.method, endpoint.path, endpoint.resourceFile)
		fmt.Fprintf(&output, "    %s,\n", endpointName(endpoint))
	}
	fmt.Fprintln(&output, "}")
	fmt.Fprintln(&output)
	fmt.Fprintln(&output, "#[derive(Debug, Clone)]")
	fmt.Fprintln(&output, "pub struct GoV397EndpointMeta {")
	fmt.Fprintln(&output, "    pub method: http::Method,")
	fmt.Fprintln(&output, "    pub path: &'static str,")
	fmt.Fprintln(&output, "    pub supported_access_token_types: &'static [AccessTokenType],")
	fmt.Fprintln(&output, "    pub file_upload: bool,")
	fmt.Fprintln(&output, "}")
	fmt.Fprintln(&output)
	fmt.Fprintln(&output, "impl GoV397Endpoint {")
	fmt.Fprintln(&output, "    pub const ALL: &'static [Self] = &[")
	for _, endpoint := range endpoints {
		fmt.Fprintf(&output, "        Self::%s,\n", endpointName(endpoint))
	}
	fmt.Fprintln(&output, "    ];")
	fmt.Fprintln(&output)
	fmt.Fprintln(&output, "    pub fn meta(self) -> GoV397EndpointMeta {")
	fmt.Fprintln(&output, "        match self {")
	for _, endpoint := range endpoints {
		fmt.Fprintf(&output, "            Self::%s => GoV397EndpointMeta {\n", endpointName(endpoint))
		fmt.Fprintf(&output, "                method: http::Method::%s,\n", endpoint.method)
		fmt.Fprintf(&output, "                path: \"%s\",\n", endpoint.path)
		fmt.Fprint(&output, "                supported_access_token_types: &[")
		for index, accessTokenType := range endpoint.tokens {
			if index > 0 {
				fmt.Fprint(&output, ", ")
			}
			fmt.Fprintf(&output, "AccessTokenType::%s", accessTokenType)
		}
		fmt.Fprintln(&output, "],")
		fmt.Fprintf(&output, "                file_upload: %t,\n", endpoint.fileUpload)
		fmt.Fprintln(&output, "            },")
	}
	fmt.Fprintln(&output, "        }")
	fmt.Fprintln(&output, "    }")
	fmt.Fprintln(&output, "}")
	return []byte(output.String())
}

func gitLines(dir string, args ...string) ([]string, error) {
	output, err := git(dir, args...)
	if err != nil {
		return nil, err
	}
	return strings.Fields(string(output)), nil
}

func gitShow(dir, object string) ([]byte, error) {
	return git(dir, "show", object)
}

func gitObjectExists(dir, object string) (bool, error) {
	command := exec.Command("git", "cat-file", "-e", object)
	command.Dir = filepath.Clean(dir)
	if err := command.Run(); err != nil {
		var exitError *exec.ExitError
		if errors.As(err, &exitError) {
			return false, nil
		}
		return false, err
	}
	return true, nil
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
