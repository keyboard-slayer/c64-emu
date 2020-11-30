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

#[allow(unused_imports)]
use crate::cpu::Cpu;

#[allow(unused_imports)]
use crate::instr::jump::jsr;

#[cfg(test)]
#[test]
fn jsr_test()
{
	let mut cpu_test = Cpu::new();
	cpu_test.sp = 0xff;
	cpu_test.pc = 0x4269;
	cpu_test.write_byte(0x2171, 0xea);
	jsr(&mut cpu_test, 0x2171);

	assert_eq!(cpu_test.sp, 0xfd);
	assert_eq!(cpu_test.fetch_mem(0x01ff), 0x42);
	assert_eq!(cpu_test.fetch_mem(0x01fe), 0x69);
	assert_eq!(cpu_test.pc, 0x2171);
	assert_eq!(cpu_test.fetch_mem(cpu_test.pc), 0xea);
}