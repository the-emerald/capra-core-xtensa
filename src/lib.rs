//! Diver decompression library. Includes ZHL-16 (B/C)
#![no_std]
#![feature(core_intrinsics)]
#![cfg_attr(not(feature = "std"), allow(unused_imports), allow(dead_code))]

use core::intrinsics;
use core::panic::PanicInfo;

use crate::common::{DiveSegment, Gas, SegmentType};
use crate::deco::zhl16::ZHL16;
use crate::deco::Tissue;
use crate::deco::zhl16::variant::Variant::C;
use once_cell::unsync::Lazy;
use core::cell::RefCell;
use core::time::Duration;
use crate::common::dive_segment::SegmentType::DecoStop;
use core::hint::unreachable_unchecked;

pub mod common;
pub mod deco;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CDiveSegment {
    /// Type of this segment. See [`SegmentType`].
    segment_type: SegmentType,
    /// Depth at the beginning of segment.
    start_depth: usize,
    /// Depth at the end of segment.
    end_depth: usize,
    /// Duration of the segment (milliseconds)
    time: u64,
    /// Ascent rate (measured in m min^-1)
    ascent_rate: isize,
    /// Descent rate (measured in m min^-1)
    descent_rate: isize,
}

impl From<DiveSegment> for CDiveSegment {
    fn from(value: DiveSegment) -> Self {
        CDiveSegment {
            segment_type: value.segment_type(),
            start_depth: value.start_depth(),
            end_depth: value.end_depth(),
            time: value.time().as_millis() as u64,
            ascent_rate: value.ascent_rate(),
            descent_rate: value.descent_rate()
        }
    }
}

static mut DECO: Lazy<ZHL16> = Lazy::new(||
    ZHL16::new_by_variant(Tissue::default(), 100, 100, C)
);

#[panic_handler]
#[allow(unused_unsafe)]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub extern "C" fn initialise() {
    unsafe { Lazy::force(&DECO); }
}

#[no_mangle]
pub extern "C" fn set_gfl(low: usize) {
    unsafe { DECO.change_gfl(low) }
}

#[no_mangle]
pub extern "C" fn set_gfh(high: usize) {
    unsafe { DECO.change_gfh(high) }
}

#[no_mangle]
pub extern "C" fn tick_segment(gas: &Gas, depth: usize, tick: u64) {
    let segment = DiveSegment::new(
        SegmentType::DiveSegment,
        depth,
        depth,
        Duration::from_secs(tick),
        -5, 5       // Placeholder value - this is a constant segment
    ).unwrap();

    unsafe {
        DECO.add_segment(&segment, &gas, 1000.0);
    }
}

#[no_mangle]
pub extern "C" fn get_next_stop(gas: &Gas, ascent_rate: isize, descent_rate: isize) -> CDiveSegment {
    unsafe {
        if DECO.find_ascent_ceiling(Some(DECO.gfh())) < 1.0 {
            match DECO.ndl(gas, 1000.0) {
                None => { unreachable_unchecked(); }
                Some(t) => { return t.into(); }
            }
        }
        DECO.next_stop(ascent_rate, descent_rate, gas, 1000.0).into()
    }
}