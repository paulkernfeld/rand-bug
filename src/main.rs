#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic;
use core::sync::atomic::Ordering;
use cortex_m_rt::entry;
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

#[entry]
fn main() -> ! {
    let mut rng = StdRng::seed_from_u64(0);
    rng.next_u32();

    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
