#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![no_std]
#![allow(warnings)]

pub mod common;
pub mod define;
pub mod mint;
pub mod registry;
pub mod smt;
pub mod transfer;
pub mod transfer_update;
pub mod update;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        pub use ckb_types::{self, molecule};
        pub use ckb_jsonrpc_types;
    } else  if #[cfg(feature = "no-std")] {
        pub use ckb_std::ckb_types;
        pub use ckb_jsonrpc_types;
        pub use molecule;
    }
}
