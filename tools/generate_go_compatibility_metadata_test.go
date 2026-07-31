package main

import (
	"strings"
	"testing"
)

func TestParseResourceSkipsTokenlessEndpoints(t *testing.T) {
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
	if len(endpoints) != 0 {
		t.Fatalf("endpoints = %#v, want no bridge endpoints", endpoints)
	}
}

func TestParseResourceRetainsAppTokenEndpoints(t *testing.T) {
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
	if len(endpoints) != 1 || strings.Join(endpoints[0].tokens, ",") != "App" {
		t.Fatalf("endpoints = %#v, want one App-token endpoint", endpoints)
	}
}
