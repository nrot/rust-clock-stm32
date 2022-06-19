use cortex_m::prelude::_embedded_hal_blocking_i2c_Write;
use stm32f1xx_hal::i2c::{BlockingI2c, Instance};

pub const VK16K33: u8 = 0b1110000;

const SS_REG: u8 = 0b00100000;
const SS_NORMAL: u8 = 0b00000001;
const RIS: u8 = 0b10100000;
const RIS_OUT: u8 = 0b00000000;
const DSP: u8 = 0b10000000;
const DSP_ON: u8 = 0b00000001;
const DIM: u8 = 0b11100000;
const DDAP: u8 = 0b0;

// pub static mut DISPLAY: [u8; 15*4] = [0; 15*4];
pub const DISP_SIZE: u8 = 16;

pub type DisplayBuff = [u8; (DISP_SIZE + 1) as usize];

#[inline(always)]
pub fn init<I: Instance, P>(i2c: &mut BlockingI2c<I, P>) {
    let data = [SS_REG | SS_NORMAL, DSP | DSP_ON, RIS | RIS_OUT, DIM | 15];
    for d in data {
        let _ = i2c.write(VK16K33, &[d]);
    }
}

#[inline(always)]
pub fn disp_off<I: Instance, P>(i2c: &mut BlockingI2c<I, P>){
    let _ = i2c.write(VK16K33, &[DSP]);
}

#[inline(always)]
pub fn disp_on<I: Instance, P>(i2c: &mut BlockingI2c<I, P>){
    let _ = i2c.write(VK16K33, &[DSP | DSP_ON]);
}

#[inline(always)]
pub fn clear<I: Instance, P>(i2c: &mut BlockingI2c<I, P>) {
    for i in 0..4{
        let _ = draw_glyph(i2c, 0b0, i);
    }
    
}

#[inline(always)]
pub fn draw_glyph<I: Instance, P>(i2c: &mut BlockingI2c<I, P>, glyph: u16, index: u8) {
    let mut c :[u8; 3] = [0; 3];
    c[0] = index * 2;
    c[1] = (glyph & 0xFF) as u8;
    c[2] = (glyph >> 8) as u8;
    let _ = i2c.write(VK16K33, &c);
}