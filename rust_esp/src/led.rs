use anyhow::Error;
use esp_idf_svc::hal::gpio::OutputPin;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::rmt::RmtChannel;
use ws2812_esp32_rmt_driver::driver::Ws2812Esp32RmtDriver;
use ws2812_esp32_rmt_driver::driver::color::{LedPixelColor, LedPixelColorGrb24};


pub struct RGBLed<'a> {
    driver: Ws2812Esp32RmtDriver<'a>,
}

impl <'a> RGBLed<'a> {
    pub fn new(pin: impl Peripheral<P = impl OutputPin> + 'a, channel: impl Peripheral<P = impl RmtChannel> + 'a) -> Result<RGBLed<'a>, Error> {

        let mut driver = Ws2812Esp32RmtDriver::new(channel, pin)?;
        Ok(RGBLed { driver: driver })
    }

    pub fn write(&mut self, r: u8, g: u8, b: u8) {
    
        self.driver.write_blocking([r,g,b].into_iter()).unwrap();
    }
}