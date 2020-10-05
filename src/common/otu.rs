use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::Gas;
use core::intrinsics::powf64;

/// Returns the Oxygen Toxicity Units (OTU) accumulated during a segment with a specified gas.
/// # Arguments
/// * `segment` - Segment to calculate OTUs for
/// * `gas` - Gas used in that segment
pub fn otu(segment: &DiveSegment, gas: &Gas) -> f64 {
    match segment.segment_type() {
        SegmentType::AscDesc => ascent_descent_constant(
            segment.time().as_secs() as usize,
            gas.pp_o2(segment.start_depth(), 10.0),
            gas.pp_o2(segment.end_depth(), 10.0),
        ),
        _ => constant_depth(
            segment.time().as_secs() as usize,
            gas.pp_o2(segment.start_depth(), 10.0),
        ),
    }
}

/// OTU in constant depth
pub fn constant_depth(time: usize, p_o2: f64) -> f64 {
    // (time as f64) * (0.5 / (p_o2 - 0.5)).powf(-5.0 / 6.0)
    unsafe { powf64(time as f64 * (0.5 / (p_o2 - 0.5)), -5.0 / 6.0) }
}

/// OTU in changing depth (constant a/descent rate)
fn ascent_descent_constant(time: usize, p_o2i: f64, p_o2f: f64) -> f64 {
    unsafe {
        ((3.0 / 11.0) * (time as f64) / (p_o2f - p_o2i)) * powf64((p_o2f - 0.5) / 0.5, 11.0 / 6.0)
            - powf64((p_o2i - 0.5) / 0.5, 11.0 / 6.0)
    }
}
