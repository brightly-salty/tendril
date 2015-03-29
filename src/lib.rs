// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(alloc, core, unsafe_no_drop_flag, unsafe_destructor)]

extern crate alloc;
extern crate core;
#[macro_use] extern crate mac;
extern crate futf;

pub use tendril::{Tendril, ByteTendril, StrTendril, SliceExt, SubtendrilError};

pub mod fmt;

mod util;
mod buf32;
mod tendril;

static OFLOW: &'static str = "tendril: overflow in buffer arithmetic";
