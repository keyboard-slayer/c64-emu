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

#[macro_use]
pub mod cpu;
mod evaluator;
pub mod instr;

use crate::cpu::Cpu;
use crate::evaluator::eval;
use std::process::exit;

fn main()
{
    let mut cpu = Cpu::new();

    if let Err(err) = cpu.load_kernal("kernal", 0xe000)
    {
        eprintln!("{}", err);
        exit(1);
    }

    cpu.load_reset_point();
    cpu.enable_debug();

    eval(&mut cpu);
}
