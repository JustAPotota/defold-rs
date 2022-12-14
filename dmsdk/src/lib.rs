#![warn(missing_docs)]

//! # dmsdk
//!
//! Rust-friendly wrappers for interacting with the [Defold](https://defold.com) extension SDK.

pub mod dmconfigfile;
pub mod dmengine;
pub mod dmextension;
pub mod dmgameobject;
mod dmhash;
pub mod dmhid;
pub mod dmlog;
pub mod dmresource;
pub mod dmscript;
pub mod dmtime;
pub mod dmvmath;
pub mod dmwebserver;
pub mod lua;

pub use dmhash::*;

#[doc(inline)]
pub use dmsdk_ffi as ffi;

#[doc(hidden)]
pub use paste::paste;
