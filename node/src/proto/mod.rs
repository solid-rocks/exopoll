//! Module of the rust-protobuf generated files.

// For protobuf generated files.
#![allow(bare_trait_objects)]
#![allow(renamed_and_removed_lints)]

pub use self::exopoll::{TxRegisterVoter, Voter};

include!(concat!(env!("OUT_DIR"), "/protobuf_mod.rs"));

use exonum::proto::schema::*;
