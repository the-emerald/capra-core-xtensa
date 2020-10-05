//! Diver decompression library. Includes ZHL-16 (B/C)
#![no_std]
#![feature(core_intrinsics)]
#![cfg_attr(not(feature = "std"), allow(unused_imports), allow(dead_code))]

use core::intrinsics;
use core::panic::PanicInfo;

use crate::common::DiveSegment;
use crate::deco::zhl16::ZHL16;

pub mod common;
pub mod deco;

#[panic_handler]
#[allow(unused_unsafe)]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub extern "C" fn initialise() -> &ZHL16 {
    unimplemented!()
}
