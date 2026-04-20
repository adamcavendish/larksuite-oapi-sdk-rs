fn main() {
    prost_build::compile_protos(&["proto/ws.proto"], &["proto/"]).unwrap();
}
