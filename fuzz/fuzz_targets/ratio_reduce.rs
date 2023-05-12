#![no_main]
use libfuzzer_sys::fuzz_target;
use num_rational::Ratio;

fuzz_target!(|input: (i32, i32)| {
    if input.1 > 0 {
        Ratio::new(input.0, input.1).reduced();
    }
});