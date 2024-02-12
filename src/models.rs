pub trait FlashModel {
    const PAGE_SIZE: u32;
    const N_PAGES: u32;
    const CAPACITY: u32 = Self::PAGE_SIZE * Self::N_PAGES;

    const SECTOR_SIZE: u32 = Self::PAGE_SIZE * 16;
    const N_SECTORS: u32 = Self::N_PAGES / 16;
    const BLOCK_32K_SIZE: u32 = Self::SECTOR_SIZE * 8;
    const N_BLOCKS_32K: u32 = Self::N_SECTORS / 8;
    const BLOCK_64K_SIZE: u32 = Self::BLOCK_32K_SIZE * 2;
    const N_BLOCKS_64K: u32 = Self::N_BLOCKS_32K / 2;
}

pub struct W25Q32;

impl FlashModel for W25Q32 {
    const PAGE_SIZE: u32 = 256;
    const N_PAGES: u32 = 16384;
}

pub struct W25Q64;

impl FlashModel for W25Q64 {
    const PAGE_SIZE: u32 = 256;
    const N_PAGES: u32 = 32768;
}

pub struct W25Q128;

impl FlashModel for W25Q128 {
    const PAGE_SIZE: u32 = 256;
    const N_PAGES: u32 = 65536;
}
