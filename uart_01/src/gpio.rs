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

use tock_registers::registers::ReadWrite;
use tock_registers::register_bitfields;
use super::PERIPHERAL_BASE;

// Descriptions taken from
// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
register_bitfields! {
    u32,

    /// GPIO Function Select 1
    pub GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD1 = 0b010  // Mini UART - Alternate function 5
        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD1 = 0b010  // Mini UART - Alternate function 5
        ]
    ],

    /// GPIO_PUP_PDN_CNTRL_REG0 Register
    pub GPPUPPDN0 [
        /// Pin 15
        GPPCNTRL15 OFFSET(30) NUMBITS(2) [
            None     = 0b00,
            PullUp   = 0b01,
            PullDown = 0b10,
            Reserved = 0b11
        ],

        /// Pin 14
        GPPCNTRL14 OFFSET(28) NUMBITS(2) [
            None     = 0b00,
            PullUp   = 0b01,
            PullDown = 0b10,
            Reserved = 0b11
        ]
    ]
}

pub const GPFSEL1: *const ReadWrite<u32, GPFSEL1::Register> =
    (PERIPHERAL_BASE + 0x0020_0004) as *const ReadWrite<u32, GPFSEL1::Register>;

pub const GPPUPPDN0: *const ReadWrite<u32, GPPUPPDN0::Register> = 
    (PERIPHERAL_BASE + 0x0020_00e4) as *const ReadWrite<u32, GPPUPPDN0::Register>;

