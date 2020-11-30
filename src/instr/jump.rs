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

use crate::cpu::Cpu;

pub fn jsr(cpu: &mut Cpu, addr: u16)
{
	let addr_l: u8 = (cpu.pc & 0xff) as u8;
	let addr_u: u8 = ((cpu.pc & 0xff00) >> 8) as u8;

	cpu.push_to_stack(addr_u);
	cpu.push_to_stack(addr_l);

	cpu.pc = addr;
}

pub fn jmp(cpu: &mut Cpu, addr: u16)
{
	cpu.pc = addr;
}