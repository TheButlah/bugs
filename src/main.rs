#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use defmt::error;

#[riscv_rt::entry]
fn main() -> ! {
	error!("Hello world");
	loop {}
}
