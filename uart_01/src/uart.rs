/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use super::PERIPHERAL_BASE;
use core::ops;
use crate::gpio;
use tock_registers::registers::*;
use tock_registers::register_bitfields;
use tock_registers::interfaces::*;
use core::arch::asm;

// Auxilary mini UART registers
// See 2.1.1 AUX registers AUX_ENABLES Register
register_bitfields! {
    u32,

    /// Auxiliary enables
    AUX_ENABLES [
        MINI_UART_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Mini Uart Interrupt Identify
    AUX_MU_IIR [
        /// Writing with bit 1 set will clear the receive FIFO
        /// Writing with bit 2 set will clear the transmit FIFO
        FIFO_CLEAR OFFSET(1) NUMBITS(2) [
            Rx = 0b01,
            Tx = 0b10,
            All = 0b11
        ]
    ],

    /// Mini Uart Line Control
    AUX_MU_LCR [
        /// Mode the UART works in
        DATA_SIZE OFFSET(0) NUMBITS(2) [
            SevenBit = 0b00,
            EightBit = 0b11
        ]
    ],

    /// Mini Uart Line Status
    AUX_MU_LSR [
        /// This bit is set if the transmit FIFO can accept at least
        /// one byte.
        TX_EMPTY   OFFSET(5) NUMBITS(1) [],

        /// This bit is set if the receive FIFO holds at least 1
        /// symbol.
        DATA_READY OFFSET(0) NUMBITS(1) []
    ],

    AUX_MU_MSR [
        CTS_STATUS OFFSET(4) NUMBITS(1) []
    ],

    AUX_MU_SCRATCH [
        SCRATCH OFFSET(0) NUMBITS(8) []
    ],

    /// Mini Uart Extra Control
    AUX_MU_CNTL [
        /// If this bit is set the mini UART transmitter is enabled.
        /// If this bit is clear the mini UART transmitter is disabled.
        TX_EN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// If this bit is set the mini UART receiver is enabled.
        /// If this bit is clear the mini UART receiver is disabled.
        RX_EN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    AUX_MU_STAT [
        // Many status, only describe 1 bit for compile demo.
        RTS_STATUS OFFSET(6) NUMBITS(1) []
    ],

    /// Mini Uart Baudrate
    AUX_MU_BAUD [
        /// Mini UART baudrate counter
        RATE OFFSET(0) NUMBITS(16) []
    ]
}

const AUX_BASE: u32 = PERIPHERAL_BASE + 0x21_5000;
const AUX_UART_CLOCK: u32 = 500000000;

fn aux_baud(baud: u32) -> u32 {
    (AUX_UART_CLOCK / (baud * 8)) - 1
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    __reserved_0: [u8; 4],                              // AUX_BASE + 0 4Bytes for IRQ
    AUX_ENABLES: ReadWrite<u32, AUX_ENABLES::Register>, // AUX_BASE + 4  (0x04)
    __reserved_1: [u8; 56],                             // AUX_BASE + 8
    AUX_MU_IO: ReadWrite<u32>,                          // AUX_BASE + 64 (0x40)
    AUX_MU_IER: WriteOnly<u32>,                         // AUX_BASE + 68 (0x44)
    AUX_MU_IIR: WriteOnly<u32, AUX_MU_IIR::Register>,   // AUX_BASE + 72 (0x48)
    AUX_MU_LCR: WriteOnly<u32, AUX_MU_LCR::Register>,   // AUX_BASE + 76 (0x4C)
    AUX_MU_MCR: WriteOnly<u32>,                         // AUX_BASE + 80 (0x50)
    AUX_MU_LSR: ReadOnly<u32, AUX_MU_LSR::Register>,    // AUX_BASE + 84 (0x54)
    AUX_MU_MSR: ReadOnly<u32, AUX_MU_MSR::Register>,    // AUX_BASE + 88
    AUX_MU_SCRATCH: ReadWrite<u32, AUX_MU_SCRATCH::Register>, // AUX_BASE + 92
    AUX_MU_CNTL: WriteOnly<u32, AUX_MU_CNTL::Register>, // AUX_BASE + 96
    AUX_MU_STAT: ReadOnly<u32, AUX_MU_STAT::Register>,  // AUX_BASE + 100
    AUX_MU_BAUD: WriteOnly<u32, AUX_MU_BAUD::Register>, // AUX_BASE + 104
}

pub struct MiniUart;

/// Deref to RegisterBlock
///
/// Allows writing
/// ```
/// self.MU_IER.read()
/// ```
/// instead of something along the lines of
/// ```
/// unsafe { (*MiniUart::ptr()).MU_IER.read() }
/// ```
impl ops::Deref for MiniUart {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl MiniUart {
    pub fn new() -> MiniUart {
        MiniUart
    }

    /// Returns a pointer to the register block
    fn ptr() -> *const RegisterBlock {
        AUX_BASE as *const _
    }

    ///Set baud rate and characteristics (115200 8N1) and map to GPIO
    pub fn init(&self) {
        self.AUX_ENABLES.modify(AUX_ENABLES::MINI_UART_ENABLE::SET);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_CNTL.set(0);
        self.AUX_MU_LCR.write(AUX_MU_LCR::DATA_SIZE::EightBit);
        self.AUX_MU_MCR.set(0);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_IIR.write(AUX_MU_IIR::FIFO_CLEAR::All);
        //self.AUX_MU_IIR.set(0xC6);
        self.AUX_MU_BAUD.write(AUX_MU_BAUD::RATE.val(aux_baud(115200))); // 115200 baud

        unsafe {
            (*gpio::GPPUPPDN0).modify(
                gpio::GPPUPPDN0::GPPCNTRL15::None + gpio::GPPUPPDN0::GPPCNTRL14::None
            );

            (*gpio::GPFSEL1).modify(
                gpio::GPFSEL1::FSEL14::TXD1 + gpio::GPFSEL1::FSEL15::RXD1
            );
        }

        self.AUX_MU_CNTL
            .write(AUX_MU_CNTL::RX_EN::Enabled + AUX_MU_CNTL::TX_EN::Enabled);
    }

    /// Send a character
    pub fn send(&self, c: char) {
        // wait until we can send
        loop {
            if self.AUX_MU_LSR.is_set(AUX_MU_LSR::TX_EMPTY) {
                break;
            }

            unsafe { asm!("nop") /*asm!("nop" :::: "volatile")*/ };
        }

        // write the character to the buffer
        self.AUX_MU_IO.set(c as u32);
    }

    /// Receive a character
    pub fn getc(&self) -> char {
        // wait until something is in the buffer
        loop {
            if self.AUX_MU_LSR.is_set(AUX_MU_LSR::DATA_READY) {
                break;
            }

            unsafe { asm!("nop") /*asm!("nop" :::: "volatile")*/ };
        }

        // read it and return
        let mut ret = self.AUX_MU_IO.get() as u8 as char;

        // convert carrige return to newline
        if ret == '\r' {
            ret = '\n';
        }

        ret
    }

    /// Display a string
    pub fn puts(&self, string: &str) {
        for c in string.chars() {
            // convert newline to carrige return + newline
            if c == '\n' {
                self.send('\r')
            }

            self.send(c);
        }
    }
}
