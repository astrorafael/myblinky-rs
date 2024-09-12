#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_time::{Duration, Timer};
use fmt::{info, unwrap};

#[embassy_executor::task(pool_size = 5)]
async fn blinker(mut led: Output<'static>, interval: Duration, _i: usize) {
    loop {
        //info!("Task {:?}", i);
        led.toggle();
        Timer::after(interval).await;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn button_task(mut pin: Input<'static>, tag: char) {
    loop {
        pin.wait_for_low().await;
        info!("Button {:?} pressed!", tag);
        pin.wait_for_high().await;
        info!("Button {:?} released!", tag);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let _row1 = Output::new(p.P0_21, Level::High, OutputDrive::Standard);

    let col1 = Output::new(p.P0_28, Level::Low, OutputDrive::Standard);
    let col2 = Output::new(p.P0_11, Level::Low, OutputDrive::Standard);
    let col3 = Output::new(p.P0_31, Level::Low, OutputDrive::Standard);
    let col4 = Output::new(p.P1_05, Level::Low, OutputDrive::Standard);
    let col5 = Output::new(p.P0_30, Level::Low, OutputDrive::Standard);

    // These two are externally pulled up by 10k resistors (R4 & R5)
    let btn_a = Input::new(p.P0_14, Pull::None);
    let btn_b = Input::new(p.P0_23, Pull::None);

    info!("Starting LED tasks");
    unwrap!(spawner.spawn(blinker(col1, Duration::from_millis(900), 1)));
    unwrap!(spawner.spawn(blinker(col2, Duration::from_millis(800), 2)));
    unwrap!(spawner.spawn(blinker(col3, Duration::from_millis(700), 3)));
    unwrap!(spawner.spawn(blinker(col4, Duration::from_millis(600), 4)));
    unwrap!(spawner.spawn(blinker(col5, Duration::from_millis(500), 5)));
    unwrap!(spawner.spawn(button_task(btn_a, 'A')));
    unwrap!(spawner.spawn(button_task(btn_b, 'B')));

    loop {
        Timer::after_millis(1000).await;
    }
}
