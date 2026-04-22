use larksuite_oapi_sdk_rs::req::{PathParams, QueryParams, ReqBody};

#[test]
fn query_params_encode_empty() {
    let qp = QueryParams::new();
    assert_eq!(qp.encode(), "");
}

#[test]
fn query_params_encode_single() {
    let mut qp = QueryParams::new();
    qp.set("page_size", "10");
    assert_eq!(qp.encode(), "page_size=10");
}

#[test]
fn query_params_encode_sorted() {
    let mut qp = QueryParams::new();
    qp.set("z_param", "last");
    qp.set("a_param", "first");
    qp.set("m_param", "middle");
    assert_eq!(qp.encode(), "a_param=first&m_param=middle&z_param=last");
}

#[test]
fn query_params_encode_special_chars() {
    let mut qp = QueryParams::new();
    qp.set("q", "hello world&foo=bar");
    assert_eq!(qp.encode(), "q=hello+world%26foo%3Dbar");
}

#[test]
fn query_params_multi_value() {
    let mut qp = QueryParams::new();
    qp.add("tag", "a");
    qp.add("tag", "b");
    let encoded = qp.encode();
    assert!(encoded.contains("tag=a"));
    assert!(encoded.contains("tag=b"));
}

#[test]
fn query_params_set_overwrites() {
    let mut qp = QueryParams::new();
    qp.set("key", "old");
    qp.set("key", "new");
    assert_eq!(qp.get("key"), Some("new"));
}

#[test]
fn path_params_set_and_get() {
    let mut pp = PathParams::new();
    assert_eq!(pp.get("id"), None);
    pp.set("id", "123");
    assert_eq!(pp.get("id"), Some("123"));
}

#[test]
fn path_params_overwrite() {
    let mut pp = PathParams::new();
    pp.set("id", "old");
    pp.set("id", "new");
    assert_eq!(pp.get("id"), Some("new"));
}

#[test]
fn query_params_get_existing_key() {
    let mut qp = QueryParams::new();
    qp.set("page_size", "20");
    assert_eq!(qp.get("page_size"), Some("20"));
}

#[test]
fn query_params_get_missing_key() {
    let qp = QueryParams::new();
    assert_eq!(qp.get("nonexistent"), None);
}

#[test]
fn req_body_json_from_struct() {
    #[derive(serde::Serialize)]
    struct Payload {
        name: String,
        count: u32,
    }
    let body = ReqBody::json(&Payload {
        name: "test".to_string(),
        count: 42,
    })
    .unwrap();
    match body {
        ReqBody::Json(v) => {
            assert_eq!(v["name"], "test");
            assert_eq!(v["count"], 42);
        }
        _ => panic!("expected Json variant"),
    }
}
