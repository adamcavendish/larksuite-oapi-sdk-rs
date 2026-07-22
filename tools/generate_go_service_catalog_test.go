package main

import (
	"strings"
	"testing"
)

func TestParseResourceExtractsEndpointMetadata(t *testing.T) {
	source := []byte(`package larkexample

import "net/http"

type message struct{}

func (m *message) Upload() {
	apiReq := request{}
	apiReq.ApiPath = "/open-apis/example/v1/messages/:message_id/files"
	apiReq.HttpMethod = http.MethodPost
	apiReq.SupportedAccessTokenTypes = []AccessTokenType{AccessTokenTypeUser, AccessTokenTypeTenant}
	options = append(options, WithFileUpload())
}
`)

	endpoints, err := parseResource("service/example/v1/resource.go", source)
	if err != nil {
		t.Fatalf("parse resource: %v", err)
	}
	if len(endpoints) != 1 {
		t.Fatalf("endpoint count = %d, want 1", len(endpoints))
	}
	endpoint := endpoints[0]
	if endpoint.Receiver != "message" || endpoint.Operation != "Upload" {
		t.Fatalf("endpoint identity = %s, want message.Upload", endpoint.identity())
	}
	if endpoint.Method != "POST" || endpoint.Path != "/open-apis/example/v1/messages/:message_id/files" {
		t.Fatalf("endpoint route = %s %s", endpoint.Method, endpoint.Path)
	}
	if strings.Join(endpoint.TokenTypes, ",") != "User,Tenant" || !endpoint.FileUpload {
		t.Fatalf("endpoint metadata = %#v", endpoint)
	}
}

func TestParseResourceRejectsIncompleteEndpointMetadata(t *testing.T) {
	source := []byte(`package larkexample

import "net/http"

type message struct{}

func (m *message) List() {
	apiReq := request{}
	apiReq.ApiPath = "/open-apis/example/v1/messages"
	apiReq.HttpMethod = http.MethodGet
}
`)

	_, err := parseResource("service/example/v1/resource.go", source)
	if err == nil || !strings.Contains(err.Error(), "incomplete endpoint metadata") {
		t.Fatalf("parse error = %v, want incomplete endpoint metadata", err)
	}
}

func TestValidateAndSortRejectsDuplicateIdentity(t *testing.T) {
	endpoints := []endpoint{
		{ResourceFile: "service/example/v1/resource.go", Receiver: "message", Operation: "Get"},
		{ResourceFile: "service/example/v1/resource.go", Receiver: "message", Operation: "Get"},
	}

	err := validateAndSort(endpoints)
	if err == nil || !strings.Contains(err.Error(), "duplicate endpoint identity") {
		t.Fatalf("validate error = %v, want duplicate endpoint identity", err)
	}
}

func TestParseResourceAcceptsNoTokenEndpoint(t *testing.T) {
	source := []byte(`package larkexample

import "net/http"

type appToken struct{}

func (a *appToken) Create() {
	apiReq := request{}
	apiReq.ApiPath = "/open-apis/auth/v3/app_access_token"
	apiReq.HttpMethod = http.MethodPost
	apiReq.SupportedAccessTokenTypes = []AccessTokenType{}
}
`)

	endpoints, err := parseResource("service/auth/v3/resource.go", source)
	if err != nil {
		t.Fatalf("parse resource: %v", err)
	}
	if len(endpoints) != 1 || len(endpoints[0].TokenTypes) != 0 {
		t.Fatalf("endpoints = %#v, want one endpoint without token types", endpoints)
	}
}

func TestParseResourceRetainsAppToken(t *testing.T) {
	source := []byte(`package larkexample

import "net/http"

type accessToken struct{}

func (a *accessToken) Create() {
	apiReq := request{}
	apiReq.ApiPath = "/open-apis/authen/v1/access_token"
	apiReq.HttpMethod = http.MethodPost
	apiReq.SupportedAccessTokenTypes = []AccessTokenType{AccessTokenTypeApp}
}
`)

	endpoints, err := parseResource("service/authen/v1/resource.go", source)
	if err != nil {
		t.Fatalf("parse resource: %v", err)
	}
	if len(endpoints) != 1 || strings.Join(endpoints[0].TokenTypes, ",") != "App" {
		t.Fatalf("endpoints = %#v, want one App-token endpoint", endpoints)
	}
}
