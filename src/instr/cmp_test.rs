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
use crate::instr::cmp::*;

#[cfg(test)]
#[test]
fn cmp_abs_test()
{
	let mut cpu_test = Cpu::new();

	cpu_test.a = 0x2C;
	cpu_test.write_byte(0x4269, 0x2C);

	cmp_abs(&mut cpu_test, 0x4269);

	assert_eq!(cpu_test.zero, true);
	assert_eq!(cpu_test.negative, false);
	assert_eq!(cpu_test.carry, true);

	cpu_test.a = 0x1C;
	cmp_abs(&mut cpu_test, 0x4269);

	assert_eq!(cpu_test.zero, false);
	assert_eq!(cpu_test.negative, true);
	assert_eq!(cpu_test.carry, false);

	cpu_test.a = 0x3C;
	cmp_abs(&mut cpu_test, 0x4269);

	assert_eq!(cpu_test.zero, false);
	assert_eq!(cpu_test.carry, true);
	assert_eq!(cpu_test.negative, false);
}
