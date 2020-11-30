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

pub fn ora_abs(cpu: &mut Cpu, addr: u16)
{
	let byte: u8 = cpu.fetch_mem(addr);
	cpu.a |= byte;

	cpu.zero = cpu.a == 0;
	cpu.negative = is_negative!(cpu.a); 
}