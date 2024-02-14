#![no_std]
#![deny(unsafe_code)]

use core::fmt::Debug;
use embedded_storage::nor_flash::{ErrorType, NorFlashError, NorFlashErrorKind};

pub mod models;
mod w25q;
#[cfg(feature = "async")]
mod w25q_async;

use self::models::FlashModel;

/// Low level driver for the w25q flash memory chips.
pub struct W25Q<MODEL, SPI> {
    model: MODEL,
    spi: SPI,
}

impl<MODEL, SPI> W25Q<MODEL, SPI>
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

impl<MODEL, SPI, S: Debug> W25Q<MODEL, SPI>
where
    MODEL: FlashModel,
    SPI: embedded_hal::spi::ErrorType<Error = S>,
{
    pub fn new(model: MODEL, spi: SPI) -> Self {
        W25Q { model, spi }
    }
}

impl<MODEL, SPI, S: Debug> ErrorType for W25Q<MODEL, SPI>
where
    MODEL: FlashModel,
    SPI: embedded_hal::spi::ErrorType<Error = S>,
{
    type Error = Error<S>;
}

/// Custom error type for the various errors that can be thrown by W25q32jv.
/// Can be converted into a NorFlashError.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error<S: Debug> {
    SpiError(S),
    NotAligned,
    OutOfBounds,
    WriteEnableFail,
    ReadbackFail,
}

impl<S: Debug> NorFlashError for Error<S> {
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
        (address & 0x0000FF) as u8,
    ]
}
