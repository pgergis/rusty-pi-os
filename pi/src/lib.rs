#![feature(allow_internal_unstable)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(extern_prelude)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;
extern crate volatile;

#[macro_export]
#[allow_internal_unstable]
macro_rules! panic {
    () => (
        panic!("explicit panic")
    );
    ($msg:expr) => ({
        ::core::panicking::panic(&($msg, file!(), line!(), __rust_unstable_column!()))
    });
    ($msg:expr,) => (
        panic!($msg)
    );
    ($fmt:expr, $($arg:tt)+) => ({
        ::core::panicking::panic_fmt(format_args!($fmt, $($arg)*),
                                     &(file!(), line!(), __rust_unstable_column!()))
    });
}

#[macro_export]
macro_rules! unimplemented {
    (  ) => (panic!("not yet implemented"));
    ( $ ( $ arg : tt ) + ) => (panic!("not yet implemented: {}", format_args!($($arg)*)));
}

pub mod timer;
pub mod uart;
pub mod gpio;
pub mod common;
