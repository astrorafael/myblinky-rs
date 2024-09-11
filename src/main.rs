#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::{Duration, Timer};
use fmt::info;

#[embassy_executor::task(pool_size = 5)]
async fn blinker(mut led: Output<'static>, interval: Duration, i: i32) {
    loop {
        info!("Task {}", i);
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let _row1 = Output::new(p.P0_21, Level::High, OutputDrive::Standard);
    let col1 = Output::new(p.P0_28, Level::High, OutputDrive::Standard);
    /*let col2 = Output::new(p.P0_11, Level::Low, OutputDrive::Standard);
    let col3 = Output::new(p.P0_31, Level::Low, OutputDrive::Standard);
    let col4 = Output::new(p.P1_05, Level::Low, OutputDrive::Standard);
    let col5 = Output::new(p.P0_30, Level::Low, OutputDrive::Standard);*/
    info!("Starting LED tasks");
    crate::fmt::unwrap!(spawner.spawn(blinker(col1, Duration::from_millis(900), 1)));
    /*crate::fmt::unwrap!(spawner.spawn(blinker(col2, Duration::from_millis(900), 2)));
    crate::fmt::unwrap!(spawner.spawn(blinker(col3, Duration::from_millis(900), 3)));
    crate::fmt::unwrap!(spawner.spawn(blinker(col4, Duration::from_millis(900), 4)));
    crate::fmt::unwrap!(spawner.spawn(blinker(col5, Duration::from_millis(900), 5)));*/
}
