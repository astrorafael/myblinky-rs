#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use fmt::{debug, info};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let _row1 = Output::new(p.P0_21, Level::High, OutputDrive::Standard);
    let mut col1 = Output::new(p.P0_28, Level::High, OutputDrive::Standard);
    let mut col2 = Output::new(p.P0_11, Level::Low, OutputDrive::Standard);
    info!("Hello world");
    loop {
        debug!("Looping...");
        col2.set_high();
        col1.set_low();
        Timer::after_millis(300).await;
        col2.set_low();
        col1.set_high();
        Timer::after_millis(300).await;
    }
}
