#![cfg_attr(feature = "uefi", no_std)]
#![feature(prelude_import)]

#[cfg(feature = "uefi")]
#[macro_use]
extern crate uefi_std as std;

#[cfg(feature = "uefi")]
#[allow(unused_imports)]
#[prelude_import]
use std::prelude::*;

pub mod chromium_ec;
pub mod commandline;
pub mod ec_binary;
mod os_specific;
pub mod pd_binary;
pub mod power;
mod util;
