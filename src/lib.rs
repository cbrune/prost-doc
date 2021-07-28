//! test library

#![warn(missing_docs)]

/// test protos module
pub mod test_protos {
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}
