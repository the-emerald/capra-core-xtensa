//! Diver decompression library. Includes ZHL-16 (B/C)
#![no_std]
#![feature(core_intrinsics)]
#![cfg_attr(not(feature = "std"), allow(unused_imports), allow(dead_code))]

use core::intrinsics;
use core::panic::PanicInfo;

use crate::common::{DiveSegment, Gas, SegmentType};
use crate::deco::zhl16::util::{ZHL16C_N2_A, ZHL16C_N2_B, ZHL16C_N2_HALFLIFE, ZHL16C_HE_A, ZHL16C_HE_B, ZHL16C_HE_HALFLIFE};
use crate::deco::Tissue;
use crate::deco::zhl16::variant::Variant::C;
use core::time::Duration;
use crate::common::dive_segment::SegmentType::DecoStop;
use crate::deco::zhl16::ZHL16;

pub mod common;
pub mod deco;

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

#[panic_handler]
#[allow(unused_unsafe)]
fn panic(_: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub extern "C" fn initialise(deco: &mut ZHL16) {
    deco.tissue = Tissue {
        p_n2: [0.7405; 16],
        p_he: [0.0; 16],
        p_t: [0.7405; 16]
    };

    deco.n2_a = ZHL16C_N2_A;
    deco.n2_b = ZHL16C_N2_B;
    deco.n2_hl = ZHL16C_N2_HALFLIFE;

    deco.he_a = ZHL16C_HE_A;
    deco.he_b = ZHL16C_HE_B;
    deco.he_hl = ZHL16C_HE_HALFLIFE;

    deco.gf_low = 1.0;
    deco.gf_high = 1.0;

    deco.first_deco_depth = usize::MAX;
}

#[no_mangle]
pub extern "C" fn tick_segment(deco: &mut ZHL16, gas: &Gas, depth: usize, tick: u64) {
    let segment = DiveSegment::new(
        SegmentType::DiveSegment,
        depth,
        depth,
        Duration::from_secs(tick),
        -5, 5       // Placeholder value - this is a constant segment
    ).unwrap();

    deco.add_segment(&segment, &gas, 10.0);
}

#[no_mangle]
pub extern "C" fn get_next_stop(deco: &ZHL16, gas: &Gas, ascent_rate: isize, descent_rate: isize) -> CDiveSegment {
    if deco.find_ascent_ceiling(Some(deco.gfh())) < 1.0 {
        match deco.ndl(gas, 1000.0) {
            None => { unreachable!(); }
            Some(t) => { return t.into(); }
        }
    }
    deco.next_stop(ascent_rate, descent_rate, gas, 10.0).into()
}