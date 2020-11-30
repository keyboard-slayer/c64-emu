// Copyright (C) 2020 Jordan DALCQ
// 
// This file is part of c64-emu.
// 
// c64-emu is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// c64-emu is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with c64-emu.  If not, see <http://www.gnu.org/licenses/>.

use std::io;
use std::fs::read;

macro_rules! translate {
    ($upper:expr, $lower: expr) => {
        ((($upper as u16) << 8) + ($lower as u16))
    }
}

macro_rules! is_negative {
    ($numbr: expr) => {
        ($numbr & 0x80) >> 7 == 1
    }
}


#[derive(Copy, Clone)]
pub struct Cpu
{
    pub pc: u16,
    pub sp: u8,

    pub a: u8,
    pub x: u8,
    pub y: u8,

    mem: [u8; 65536],

    pub carry: bool,
    pub zero: bool,
    pub interrupt: bool,
    pub decimal: bool,
    pub brk: bool,
    pub overflow: bool,
    pub negative: bool,
    pub empty: bool,
    pub debug: bool,
}

impl Cpu
{
    pub fn new() -> Cpu
    {
        Cpu {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            mem: [0; 65536],
            carry: false,
            zero: false,
            interrupt: false,
            decimal: false,
            brk: false,
            overflow: false,
            negative: false,
            empty: false,
            debug: false
        }
    }

    pub fn load_reset_point(&mut self)
    {
        let addr: u16 = ((self.mem[0xfffd] as u16) << 8) + self.mem[0xfffc] as u16;
        self.pc = addr;
    }

    pub fn load_kernal(&mut self, path: &str, _addr: usize) -> io::Result<()>
    {
        let vec = read(path)?;

        for i in 0xe000 .. 0xffff
        {
            self.mem[i] = vec[i-0xe000];
        }

        Ok(())
    }

    pub fn enable_debug(&mut self)
    {
        self.debug = true;
    }

    pub fn dump_memory(self)
    {
        print!("          ");
        for i in 0..16
        {
            print!("{:02x}", i);
        }

        for i in 0..65536
        {
            if i % 16 == 0
            {
                print!("\n{:08x}", i);
            }

            print!("{:02x} ", self.mem[i]);
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8)
    {
        self.mem[addr as usize] = byte;
    }

    pub fn read_byte(&mut self, addr: u16) -> u8
    {
        self.mem[addr as usize]
    }

    pub fn fetch_next(&mut self) -> u8
    {
        let byte = self.mem[self.pc as usize];
        self.pc += 1;

        byte
    }

    pub fn fetch_mem(&mut self, addr: u16) -> u8
    {
        self.mem[addr as usize]
    }

    pub fn add_to_pc(&mut self, add: u16)
    {
        self.pc += add;
    }

    pub fn push_to_stack(&mut self, value: u8)
    {
        let index: u16 = (0x0100 + self.sp as u16) as u16;

        self.write_byte(index, value);
        self.sp -= 1;
    }

    pub fn pop_from_stack(&mut self) -> u8 
    {
        self.sp += 1;
        let index: u16 = (0x0100 + self.sp as u16) as u16;

        self.read_byte(index)
    }

    pub fn subst(&mut self, word1: u16, word2: u16) -> u16
    {
        if word2 > word1
        {
            0xff - (word2 - word1)
        }
        else 
        {
            word1 - word2
        }
    }

    pub fn sum_addr(&mut self, addr: u16, byte: u8) -> u16 
    {
        let mut sum: u16 = addr;

        if is_negative!(byte)
        {
            sum += (0xff - byte as u16) as u16;
        }
        else 
        {
            sum += byte as u16;
        }

        sum
    }
}