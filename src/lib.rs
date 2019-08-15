#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
pub mod ioctl;
pub mod videodev2;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
