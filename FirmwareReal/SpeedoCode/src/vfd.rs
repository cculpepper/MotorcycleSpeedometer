#![no_std]

//! This crate provides a VFD driver to connect to TFT displays.

// pub mod instruction;
mod VFDSegmentFont;
use crate::vfd::VFDSegmentFont::VFDFont;

use heapless::String;
use heapless::consts::*;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::spi;
use embedded_hal::blocking::delay::DelayMs;

/// VFD driver to connect to TFT displays.
pub struct VFD <SPI, BLANK, STROBE>
where
    SPI: spi::Write<u8>,
    BLANK: OutputPin,
    STROBE: OutputPin,
{
    /// SPI
    spi: SPI,

    blank: BLANK,

    /// Reset pin.
    strobe: STROBE,

    fb : [u32;12],
    currentGate : u8,
}

impl<SPI, BLANK, STROBE> VFD<SPI, BLANK, STROBE>
where
    SPI: spi::Write<u8>,
    BLANK: OutputPin,
    STROBE: OutputPin,
{
    /// Creates a new driver instance that uses hardware SPI.
    pub fn new(
        spi: SPI,
        blank: BLANK,
        strobe: STROBE,
    ) -> Self
    {
        let display = VFD {
            spi,
            blank,
            strobe,
            fb : [0;12],
            currentGate : 0,
        };

        display
    }

    /// Runs commands to initialize the display.
    pub fn init<DELAY>(&mut self, delay: &mut DELAY)
        where DELAY: DelayMs<u8>
    {
        // We will probably just clock out a bunch of zeros
        // self.spi.write(0 as u8);
    }


    fn start_data(&mut self) {
        self.strobe.set_high().map_err(|_| ());
    }

    fn write_data(&mut self, data: &[u8]){
        self.spi.write(data).map_err(|_| ());
    }

    /// Writes a data word to the display.
    fn write_word(&mut self, value: u16){
        self.write_data(&value.to_be_bytes())
    }
    pub fn write_char_fb(&mut self, row: u8, col: u8, c: char){
        if (row > 1) || (col > 9) {
            return;
        }
        let gate = (col + 1) as usize;
        let fontIndex = (c as u8 - ' ' as u8 ) as usize;
        let upperDigitMask : u32 = 0b0000_0000_0000_0011_1111_1111_1111;
        if row == 0{
            // Then we have an upper digit
            self.fb[gate] &= !upperDigitMask;
            self.fb[gate] |= upperDigitMask & (
                (VFDFont[fontIndex] as u32)
                );
        } else if row == 1{
            // Then we have an upper digit
            self.fb[gate] &= upperDigitMask;
            self.fb[gate] |= !upperDigitMask & (
                (VFDFont[fontIndex] as u32) << 14
                );
        }
    }
    pub fn write_u32(&mut self, row: u8, col: u8, n: u16){
        let mut buf : [u8;4] = [0x00;4];
        let mut w = n;

        w %= 9999;
        // w should now be in the right range
        buf[3] = (((w % 10) & 0xff) as u8) + (48);
        w /= 10;
        buf[2] = (((w % 10) & 0xff) as u8) + (48);
        w /= 10;
        buf[1] = (((w % 10) & 0xff) as u8) + (48);
        w /= 10;
        buf[0] = (((w % 10) & 0xff) as u8) + (48);
        w /= 10;

        self.write_char_fb(row, col, buf[3].into());
        self.write_char_fb(row, col+1, buf[2].into());
        self.write_char_fb(row, col+2, buf[1].into());
        self.write_char_fb(row, col+3, buf[0].into());

    }


    pub fn write_str(&mut self, row: u8, inputCol: u8, s: &str){
        let mut col = inputCol;
        if (row > 1) || (col > 9) {
            return;
        }
        for &byte in s.as_bytes(){
            if col > 9{
                return;
            }
            self.write_char_fb(row, col, byte.into());
            col += 1;
        }
    }

    pub fn update_display(&mut self){
        self.write_gate_fb(self.currentGate);
        self.currentGate += 1;
        if self.currentGate >= 12{
            self.currentGate = 0;
        }
    }

    pub fn write_gate_fb(&mut self, gate: u8) {
        self.clock_out_data(self.fb[gate as usize], gate);
    }

    pub fn clock_out_data(&mut self, filaments: u32, gate: u8){
        // The upper bytes in this array will be sent first, so the lower 1.5 bytes are the gate.
        // also assuming the bits will be sent out lower first.
        self.blank.set_high();
        // self.strobe.set_low();
        let mut data_out : u64 = 0;
        data_out |= (0x001 << gate);
        data_out |= ((filaments & 0x0fffffff) as u64) << 12;
        self.clock_out_data_direct(data_out);
    }
    pub fn clock_out_data_direct(&mut self, filaments: u64){
        // The upper bytes in this array will be sent first,.
        self.blank.set_high();
        let mut data : [u8; 5] = [0;5];
        data[4] |= (filaments & 0xff) as u8;
        data[3] = ((filaments >> 8) & 0xff) as u8;
        data[2] = ((filaments >> 16) & 0xff) as u8;
        data[1] |= ((filaments >> 24) & 0xff) as u8;
        data[0] |= ((filaments >> 32) & 0xff) as u8;
        unsafe{
        self.spi.write(&data).map_err(|_| ());
        self.strobe.set_low();
        self.strobe.set_high();
        self.blank.set_low();
        }
        
    }

}
