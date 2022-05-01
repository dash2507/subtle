#![no_main]

#[macro_use]
extern crate libfuzzer_sys;
extern crate core;
extern crate subtle as parity_subtle;

use core::intrinsics::transmute;

use parity_subtle::ConditionallySelectable;

fuzz_target!(|data: &[u8]| {
    let chunk_size: usize = 2;

    if data.len() % chunk_size != 0 {
        return;
    }

    for bytes in data.chunks(chunk_size) {
        unsafe {
            let mut x: u16 = 0;
            let y: u16 = transmute::<[u8; 2], u16>([bytes[0], bytes[1]]);

            x.conditional_assign(&y, 0.into());
            assert_eq!(x, 0);

            x.conditional_assign(&y, 1.into());
            assert_eq!(x, y);
        }
    }
});
