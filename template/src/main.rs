use rust_esp::led::*;
use rust_esp::timer::*;

use esp_idf_svc::{
    hal::peripherals::Peripherals,
};

fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    let perf = Peripherals::take().unwrap();

    log::info!("Hello, world!");

    let mut led = RGBLed::new(perf.pins.gpio48, perf.rmt.channel0).unwrap();

    led.write(0, 40, 40);

    let mut timer: i64 = 0;
    let mut state = false;

    unsafe {loop {
        if millis() - timer >= 1000 {
            timer = millis();
            if state {
                led.write(0, 40, 40);
            } else {
                led.write(0, 0, 0);
            }
            state = !state;
        }
    }}
}