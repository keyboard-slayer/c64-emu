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

pub fn sei(cpu: &mut Cpu)
{
	cpu.interrupt = true;
}

pub fn txs(cpu: &mut Cpu)
{
	cpu.sp = cpu.x;	
}

pub fn tay(cpu: &mut Cpu)
{
	cpu.y = cpu.a;

	cpu.zero = cpu.y == 0;
	cpu.negative = is_negative!(cpu.y);
}

pub fn iny(cpu: &mut Cpu)
{
    cpu.y = cpu.sum(cpu.y, 1);

	cpu.zero = cpu.y == 0;
	cpu.negative = is_negative!(cpu.y);
}

pub fn dex(cpu: &mut Cpu)
{
	cpu.x = cpu.subst(cpu.x as u16, 1) as u8;

	cpu.zero = cpu.x == 0;
	cpu.negative = is_negative!(cpu.x);
}

pub fn cld(cpu: &mut Cpu)
{
	cpu.decimal = false;
}

pub fn rts(cpu: &mut Cpu)
{
	let addr_l = cpu.pop_from_stack();
	let addr_u = cpu.pop_from_stack();

	let addr = translate!(addr_u, addr_l);
	cpu.pc = addr;
}