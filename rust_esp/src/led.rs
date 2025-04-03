use anyhow::Result;
use core::time::Duration;

use esp_idf_svc::hal::{
    rmt::{config::TransmitConfig, FixedLengthSignal, PinState, Pulse, RmtChannel, TxRmtDriver},
    peripheral::Peripheral,
    gpio::OutputPin,
};


pub struct RGBLed<'a> {
    driver: TxRmtDriver<'a>,
    p_one: (Pulse, Pulse),
    p_zero: (Pulse, Pulse),
}

impl <'a> RGBLed<'a> {
    pub fn new(led: impl Peripheral<P = impl OutputPin> + 'a, channel: impl Peripheral<P = impl RmtChannel> + 'a) -> Result<Self> {
        let config = TransmitConfig::new().clock_divider(2);
        let tx = TxRmtDriver::new(channel, led, &config)?;

        let ticks_hz = tx.counter_clock()?;

        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(800))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(700))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(600))?;

        Ok(Self { driver: tx, p_one: (t1h, t1l), p_zero: (t0h, t0l) })
    }

    pub fn write(&mut self, r:u8, g:u8, b:u8) -> Result<()> {

        let color: u32 = ((g as u32) << 16) | ((r as u32) << 8) | b as u32;
        
        let mut signal = FixedLengthSignal::<24>::new();
        for i in (0..24).rev() {
            let bit = (color >> i) & 0x01 != 0;
            let p: &(Pulse,Pulse)= if bit { &self.p_one } else { &self.p_zero };
            signal.set(23 - i as usize, p)?;
        }
        
        self.driver.start_blocking(&signal)?;

        Ok(())
    }
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}