#![no_main]
use libfuzzer_sys::fuzz_target;
use num_rational::Ratio;

fuzz_target!(|input: (i32, i32, i32, i32)| {
    let (a, b, c, d) = input;
    if b >> 16 > 0 && d >> 16 > 0 {
        let _ = Ratio::new(a >> 16, b >> 16) * Ratio::new(c >> 16, d >> 16);
    }
});