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
use crate::instr::operator::*;

#[cfg(test)]
#[test]
fn ora_abs_test()
{
	let mut cpu_test = Cpu::new();
	cpu_test.write_byte(0x0012, 0x10);
	cpu_test.a = 0x42;

	ora_abs(&mut cpu_test, 0x0012);

	assert_eq!(cpu_test.a, 0x52);
	assert_eq!(cpu_test.zero, false);
	assert_eq!(cpu_test.negative, false);

	cpu_test.write_byte(0x1200, 0xff);
	ora_abs(&mut cpu_test, 0x1200);

	assert_eq!(cpu_test.a, 0xff);
	assert_eq!(cpu_test.zero, false);
	assert_eq!(cpu_test.negative, true);
}