//! Alloy serde helpers for primitive types.

#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

/// Helpers for dealing with booleans.
mod bool;
pub use self::bool::*;

/// Helpers for dealing with numbers.
#[cfg_attr(not(test), deprecated = "use `quantity::{self, opt, vec}` instead")]
pub mod num;
#[allow(deprecated)]
pub use num::*;

pub mod quantity;

/// Storage related helpers.
pub mod storage;
pub use storage::JsonStorageKey;

pub mod ttd;
pub use ttd::*;

use alloc::format;
use serde::Serializer;

use alloy_primitives::B256;

/// Serialize a byte vec as a hex string _without_ the "0x" prefix.
///
/// This behaves the same as [`hex::encode`](alloy_primitives::hex::encode).
pub fn serialize_hex_string_no_prefix<S, T>(x: T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    s.serialize_str(&alloy_primitives::hex::encode(x.as_ref()))
}

/// Serialize a [B256] as a hex string _without_ the "0x" prefix.
pub fn serialize_b256_hex_string_no_prefix<S>(x: &B256, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{x:x}"))
}
