//! Include gRPC definition files that have been generated by `build.rs`

// tonic does not derive `Eq` for the gRPC message types which causes a warning from Clippy. The
// current suggestion is to explicitly allow the lint in the module that imports the protos, see
// https://github.com/hyperium/tonic/issues/1056
#![allow(clippy::derive_partial_eq_without_eq)]

pub static FILE_DESCRIPTOR_SET_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));

pub mod csi {
    pub mod v1 {
        tonic::include_proto!("csi.v1");
    }
}
