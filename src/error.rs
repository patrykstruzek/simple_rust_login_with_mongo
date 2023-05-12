use alloc::{
    alloc::{AllocError, LayoutError},
    collections::TryReserveError,
};
use core::convert::From;
use core::num::TryFromIntError;
use core::str::Utf8Error;

declare_err!(EPERM, "Operation not permitted.");

pub type Result<T = ()> = core::result::Result<T, Error>;
