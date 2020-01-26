use prost_build;

fn main() {
    // generate rust file from the protocol buffer
    prost_build::compile_protos(&["src/resources/BEP.proto"], &["src/"]).unwrap();
}
