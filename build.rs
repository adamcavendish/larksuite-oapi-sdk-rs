fn main() {
    if std::env::var("CARGO_FEATURE_WS").is_ok() {
        prost_build::compile_protos(&["proto/ws.proto"], &["proto/"]).unwrap();
    }
}
