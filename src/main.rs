#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use defmt::error;
use esp32c3_hal::{
	clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Rtc,
};

#[riscv_rt::entry]
fn main() -> ! {
	let p = Peripherals::take().unwrap();
	let system = p.SYSTEM.split();
	let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

	// Disable watchdogs
	{
		let mut rtc = Rtc::new(p.RTC_CNTL);
		let timer_group0 = TimerGroup::new(p.TIMG0, &clocks);
		let mut wdt0 = timer_group0.wdt;
		let timer_group1 = TimerGroup::new(p.TIMG1, &clocks);
		let mut wdt1 = timer_group1.wdt;

		rtc.rwdt.disable();
		rtc.swd.disable();
		wdt0.disable();
		wdt1.disable();
	}

	let delay = esp32c3_hal::Delay::new(&clocks);

	let mut i = 0;
	loop {
		delay.delay(100_000);
		i += 1;
		error!("i was {}", i);
	}
}
