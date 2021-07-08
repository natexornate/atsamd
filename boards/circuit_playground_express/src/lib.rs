#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use embedded_hal as ehal;
pub use hal::pac;

// use hal::clock::GenericClockController;
// //use hal::sercom::v2::{spi, Sercom3};
// use hal::sercom::I2CMaster5;
// use hal::time::Hertz;

hal::bsp_pins!(
    /// Pin 0, rx. Also analog input (A6)
    PB09 {
        name: a6
        aliases: {
            AlternateC: UartRx
        }
    }
    /// Pin 1, tx. Also analog input (A7)
    PB08 {
        name: a7
        aliases: {
            AlternateC: UartTx
        }
    }
    /// Pin 4, button A.
    PA28 {
        name: d4
    }
    /// Pin 5, button B.
    PA14 {
        name: d5
    }
    /// Pin 7, slide switch.
    PA15 {
        name: d7
    }
    /// Pin 11, speaker enable.
    PA30 {
        name: d11
    }
    /// Digital pin number 13, which is also attached to the red LED. PWM capable.
    PA17 {
        name: d13
        aliases: {
            PushPullOutput: RedLed
        }
    }
    /// The I2C SDA. Also D2 and A5.
    PB02 {
        name: sda
        aliases: {
            AlternateC: Sda
        }
    }
    /// The I2C SCL. Also D3 and A4
    PB03 {
        name: scl
        aliases: {
            AlternateC: Scl
        }
    }

    /// The data line attached to the neopixel. Also D8.
    PB23 {
        name: neopixel
    }

    /// The line attached to the speaker. Also D12 and A0.
    PA02 {
        name: speaker
    }

    /// The SPI SCK. Also D6 and A1
    PA05 {
        name: sclk
        aliases: {
            AlternateD: Sclk
        }
    }
    /// The SPI MOSI. Also D10 and A3
    PA07 {
        name: mosi
        aliases: {
            AlternateD: Mosi
        }
    }
    /// The SPI MISO. Also D9 and A2
    PA06 {
        name: miso
        aliases: {
            AlternateD: Miso
        }
    }

    /// The SCK pin attached to the on-board SPI flash
    PA21 {
        name: flash_sclk
    }
    /// The MOSI pin attached to the on-board SPI flash
    PA20 {
        name: flash_mosi
    }
    /// The MISO pin attached to the on-board SPI flash
    PA16 {
        name: flash_miso
    }
    /// The CS pin attached to the on-board SPI flash
    PB22 {
        name: flash_cs
    }

    PA00 {
        name: accel_sda
    }
    PA01 {
        name: accel_scl
    }
);

// /// I2C master for the labelled SDA & SCL pins
// pub type I2C = I2CMaster5<Sda, Scl>;

// /// Convenience for setting up the labelled SDA, SCL pins to
// /// operate as an I2C master running at the specified frequency.
// pub fn i2c_master(
//     clocks: &mut GenericClockController,
//     baud: impl Into<Hertz>,
//     sercom5: pac::SERCOM5,
//     pm: &mut pac::PM,
//     sda: impl Into<Sda>,
//     scl: impl Into<Scl>,
// ) -> I2C {
//     let gclk0 = clocks.gclk0();
//     let clock = &clocks.sercom5_core(&gclk0).unwrap();
//     let baud = baud.into();
//     let sda = sda.into();
//     let scl = scl.into();
//     I2CMaster5::new(clock, baud, sercom5, pm, sda, scl);
// }
