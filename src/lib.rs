#![feature(ip_addr)]

extern crate libc;
extern crate nix;

mod ffi;

pub mod interface;

pub use interface::{Interface, Kind};
