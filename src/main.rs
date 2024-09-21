#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::{Input, Level, Output, OutputDrive, Pull},
    Peripherals,
};
use embassy_time::{Duration, Timer};
use fmt::{info, unwrap};

// task functions cannot use generics,
// so it can't use a generic 'a lifetime specifier
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

// Micro:bit hardware resources to use for LEDS
pub struct LEDHwResources<'a> {
    _row1: embassy_nrf::gpio::Output<'a>,
    col1: embassy_nrf::gpio::Output<'a>,
    col2: embassy_nrf::gpio::Output<'a>,
    col3: embassy_nrf::gpio::Output<'a>,
    col4: embassy_nrf::gpio::Output<'a>,
    col5: embassy_nrf::gpio::Output<'a>,
}

// Micro:bit hardware resources to use for Buttons
pub struct ButtonHwResources<'a> {
    btn_a: embassy_nrf::gpio::Input<'a>,
    btn_b: embassy_nrf::gpio::Input<'a>,
}

fn split_resources<'a>(p: Peripherals) -> (LEDHwResources<'a>, ButtonHwResources<'a>) {
    (
        LEDHwResources {
            // LED Matrix, columns 1 to 5
            col1: Output::new(p.P0_28, Level::Low, OutputDrive::Standard),
            col2: Output::new(p.P0_11, Level::Low, OutputDrive::Standard),
            col3: Output::new(p.P0_31, Level::Low, OutputDrive::Standard),
            col4: Output::new(p.P1_05, Level::Low, OutputDrive::Standard),
            col5: Output::new(p.P0_30, Level::Low, OutputDrive::Standard),
            // LED Matrix Row 1
            _row1: Output::new(p.P0_21, Level::High, OutputDrive::Standard),
        },
        ButtonHwResources {
            btn_a: Input::new(p.P0_14, Pull::None),
            btn_b: Input::new(p.P0_23, Pull::None),
        },
    )
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p: Peripherals = embassy_nrf::init(Default::default());
    let (hw1, hw2) = split_resources(p);

    info!("Starting LED tasks");
    unwrap!(spawner.spawn(blinker(hw1.col1, Duration::from_millis(900), 1)));
    unwrap!(spawner.spawn(blinker(hw1.col2, Duration::from_millis(800), 2)));
    unwrap!(spawner.spawn(blinker(hw1.col3, Duration::from_millis(700), 3)));
    unwrap!(spawner.spawn(blinker(hw1.col4, Duration::from_millis(600), 4)));
    unwrap!(spawner.spawn(blinker(hw1.col5, Duration::from_millis(500), 5)));
    unwrap!(spawner.spawn(button_task(hw2.btn_a, 'A')));
    unwrap!(spawner.spawn(button_task(hw2.btn_b, 'B')));

    loop {
        Timer::after_millis(1000).await;
    }
}
