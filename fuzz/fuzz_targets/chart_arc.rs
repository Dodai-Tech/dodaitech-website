//! Property: an arc between any two finite angles is path data without NaN or inf.
#![no_main]
use dodaitech_website::charts::arc;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|d: (i16, i16, u16, i16, i16)| {
    let (cx, cy, r, a0, a1) = d;
    let text = arc(
        f32::from(cx),
        f32::from(cy),
        f32::from(r),
        f32::from(a0),
        f32::from(a1),
    );
    assert!(
        text.starts_with('M') && !text.contains("NaN") && !text.contains("inf"),
        "{text}"
    );
});
