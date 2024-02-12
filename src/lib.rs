#![no_std]
#![deny(unsafe_code)]

use core::fmt::Debug;
use embedded_hal::digital::{OutputPin, PinState};
use embedded_storage::nor_flash::{ErrorType, NorFlashError, NorFlashErrorKind};

pub mod models;
mod w25q;
#[cfg(feature = "async")]
mod w25q_async;

use self::models::FlashModel;

/// Low level driver for the w25q32jv flash memory chip.
pub struct W25Q<MODEL, SPI, HOLD, WP> {
    model: MODEL,
    spi: SPI,
    hold: HOLD,
    wp: WP,
}

impl<MODEL, SPI, HOLD, WP> W25Q<MODEL, SPI, HOLD, WP>
where
    MODEL: FlashModel,
{
    /// Get the model of the flash chip.
    pub fn model(&self) -> &MODEL {
        &self.model
    }

    /// Get the capacity of the flash chip in bytes.
    pub fn capacity() -> usize {
        MODEL::CAPACITY as usize
    }
}

impl<MODEL, SPI, S: Debug, P: Debug, HOLD, WP> W25Q<MODEL, SPI, HOLD, WP>
where
    MODEL: FlashModel,
    SPI: embedded_hal::spi::ErrorType<Error = S>,
    HOLD: OutputPin<Error = P>,
    WP: OutputPin<Error = P>,
{
    pub fn new(model: MODEL, spi: SPI, hold: HOLD, wp: WP) -> Result<Self, Error<S, P>> {
        let mut flash = W25Q {
            model,
            spi,
            hold,
            wp,
        };

        flash.hold.set_high().map_err(Error::PinError)?;
        flash.wp.set_high().map_err(Error::PinError)?;

        Ok(flash)
    }

    /// Set the hold pin state.
    ///
    /// The driver doesn't do anything with this pin. When using the chip, make sure the hold pin is not asserted.
    /// By default this means the pin needs to be high (true).
    ///
    /// This function sets the pin directly and can cause the chip to not work.
    pub fn set_hold(&mut self, value: PinState) -> Result<(), Error<S, P>> {
        self.hold.set_state(value).map_err(Error::PinError)?;
        Ok(())
    }

    /// Set the write protect pin state.
    ///
    /// The driver doesn't do anything with this pin. When using the chip, make sure the hold pin is not asserted.
    /// By default this means the pin needs to be high (true).
    ///
    /// This function sets the pin directly and can cause the chip to not work.
    pub fn set_wp(&mut self, value: PinState) -> Result<(), Error<S, P>> {
        self.wp.set_state(value).map_err(Error::PinError)?;
        Ok(())
    }
}

impl<MODEL, SPI, S: Debug, P: Debug, HOLD, WP> ErrorType for W25Q<MODEL, SPI, HOLD, WP>
where
    MODEL: FlashModel,
    SPI: embedded_hal::spi::ErrorType<Error = S>,
    HOLD: OutputPin<Error = P>,
    WP: OutputPin<Error = P>,
{
    type Error = Error<S, P>;
}

/// Custom error type for the various errors that can be thrown by W25q32jv.
/// Can be converted into a NorFlashError.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error<S: Debug, P: Debug> {
    SpiError(S),
    PinError(P),
    NotAligned,
    OutOfBounds,
    WriteEnableFail,
    ReadbackFail,
}

impl<S: Debug, P: Debug> NorFlashError for Error<S, P> {
    fn kind(&self) -> NorFlashErrorKind {
        match self {
            Error::NotAligned => NorFlashErrorKind::NotAligned,
            Error::OutOfBounds => NorFlashErrorKind::OutOfBounds,
            _ => NorFlashErrorKind::Other,
        }
    }
}

/// Easily readable representation of the command bytes used by the flash chip.
#[repr(u8)]
enum Command {
    PageProgram = 0x02,
    ReadData = 0x03,
    ReadStatusRegister1 = 0x05,
    WriteEnable = 0x06,
    SectorErase = 0x20,
    UniqueId = 0x4B,
    Block32Erase = 0x52,
    Block64Erase = 0xD8,
    ChipErase = 0xC7,
    EnableReset = 0x66,
    PowerDown = 0xB9,
    ReleasePowerDown = 0xAB,
    Reset = 0x99,
}

fn command_and_address(command: u8, address: u32) -> [u8; 4] {
    [
        command,
        // MSB, BE
        ((address & 0xFF0000) >> 16) as u8,
        ((address & 0x00FF00) >> 8) as u8,
        ((address & 0x0000FF) >> 0) as u8,
    ]
}
