fn main() {
    #[cfg(feature = "ws")]
    prost_build::compile_protos(&["proto/ws.proto"], &["proto/"]).unwrap();
}
