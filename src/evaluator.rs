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

use crate::cpu::*;
use crate::instr::*;
use std::process::exit;

fn log_instr(cpu: Cpu, msg: String)
{
    if cpu.debug
    {
        println!("\x1b[90m{}\x1b[39m", msg);
    }
}

pub fn eval(cpu: &mut Cpu)
{
    loop
    {
        let opcode: u8 = cpu.fetch_next();
        match opcode
        {
            0xa2 => {
                let byte: u8 = cpu.fetch_next();
                log_instr(*cpu, format!("LDX #${:02x}", byte));

                load::ldx_imm(cpu, byte);
            }

            0xa0 => {
                let byte: u8 = cpu.fetch_next();
                log_instr(*cpu, format!("LDY #${:02x}", byte));

                load::ldy_imm(cpu, byte);
            }

            0x84 => {
                let addr: u16 = cpu.fetch_next() as u16;

                log_instr(*cpu, format!("LDY ${:02x}", addr));
                load::ldy_abs(cpu, addr);

            }

            0xbd => {
                let addr_l: u8 = cpu.fetch_next();
                let addr_u: u8 = cpu.fetch_next();
                let addr: u16 = translate!(addr_u, addr_l);

                log_instr(*cpu, format!("LDA ${:04x}, X", addr));

                load::lda_abs(cpu, addr + cpu.x as u16);
            }

            0x78 => {
                log_instr(*cpu, format!("SEI"));
                misc::sei(cpu);
            }

            0x9a => {
                log_instr(*cpu, format!("TXS"));
                misc::txs(cpu);
            }

            0xa8 => {
                log_instr(*cpu, format!("TAY"));
                misc::tay(cpu);
            }

            0xd8 => {
                log_instr(*cpu, format!("CLD"));
                misc::cld(cpu);
            }

            0x60 => {
                log_instr(*cpu, format!("RTS"));
                misc::rts(cpu);
            }

            0xca => {
                log_instr(*cpu, format!("DEX"));
                misc::dex(cpu);
            }

            0xc8 => {
                log_instr(*cpu, format!("INY"));
                misc::iny(cpu);
            }

            0xe6 => {
                let addr: u16 = cpu.fetch_next() as u16;
                log_instr(*cpu, format!("INC ${:02x}", addr));

                mem::inc(cpu, addr);
            }

            0x20 => {
                let addr_l: u8 = cpu.fetch_next();
                let addr_u: u8 = cpu.fetch_next();
                let addr: u16 = translate!(addr_u, addr_l);

                log_instr(*cpu, format!("JSR ${:04x}\t\tFROM: ${:04x}", addr, cpu.pc-3));

                jump::jsr(cpu, addr);
            }

            0x29 => {
                let byte = cpu.fetch_next();

                log_instr(*cpu, format!("AND ${:02x}", byte));
                logic::and(cpu, byte);
            }

            0x09 => {
                let byte = cpu.fetch_next();

                log_instr(*cpu, format!("ORA ${:02x}", byte));
                logic::ora(cpu, byte);
            }

            0x4c => {
                let addr_l: u8 = cpu.fetch_next();
                let addr_u: u8 = cpu.fetch_next();
                let addr: u16 = translate!(addr_u, addr_l);

                log_instr(*cpu, format!("JMP ${:04x}\t\tFROM: ${:04x}", addr, cpu.pc-3));
                jump::jmp(cpu, addr);
            }

            0x05 => {
                let addr: u16  = (cpu.fetch_next() as u16) << 8;

                log_instr(*cpu, format!("ORA ${:04x}", addr));
                operator::ora_abs(cpu, addr);
            }

            0xdd => {
                let addr_l = cpu.fetch_next();
                let addr_u = cpu.fetch_next();
                let addr: u16 = translate!(addr_u, addr_l);
                
                log_instr(*cpu, format!("CMP ${:04x}, X", addr));
                cmp::cmp_abs(cpu, addr + cpu.x as u16);
            }

            0xa9 => {
                let byte = cpu.fetch_next();

                log_instr(*cpu, format!("LDA #${:02x}", byte));
                load::lda_imm(cpu, byte);
            }

            0xad => {
                let addr_l = cpu.fetch_next();
                let addr_u = cpu.fetch_next();
                let addr: u16 = translate!(addr_u, addr_l);

                log_instr(*cpu, format!("LDA ${:04x}", addr));
                load::lda_abs(cpu, addr);
            }

            0x85 => {
                let addr = cpu.fetch_next() as u16;

                log_instr(*cpu, format!("STA ${:02x}", addr));
                store::sta(cpu, addr);
            }

            0x99 => {
                let addr_l = cpu.fetch_next();
                let addr_u = cpu.fetch_next();
                let mut addr: u16 = translate!(addr_u, addr_l);

                log_instr(*cpu, format!("STA ${:04x}, Y", addr));
                addr = cpu.sum_addr(addr, cpu.y);

                store::sta(cpu, addr);
            }

            0x8e => {
                let addr_l = cpu.fetch_next();
                let addr_u = cpu.fetch_next();
                let addr: u16 = translate!(addr_u, addr_l);

                log_instr(*cpu, format!("STX ${:04x}", addr));
                store::stx(cpu, addr);
            }

            0x86 => {
                let addr: u16 = cpu.fetch_next() as u16;

                log_instr(*cpu, format!("STX ${:02x}", addr));
                store::stx(cpu, addr);
            }

            0x8d => {
                let addr_l = cpu.fetch_next();
                let addr_u = cpu.fetch_next();
                let addr: u16 = translate!(addr_u, addr_l);

                log_instr(*cpu, format!("STA ${:04x}", addr));
				store::sta(cpu, addr);
            }

            0xd0 => {
                let addr_add = cpu.fetch_next();
                let addr = cpu.sum_addr(cpu.pc, addr_add); 
                
                log_instr(*cpu, format!("BNE ${:04x}\t\tFROM: ${:04x}", addr, cpu.pc-2));
                
                branch::bne(cpu, addr);
            }

            0xf0 => {
                let addr_add = cpu.fetch_next();
                let addr = cpu.sum_addr(cpu.pc, addr_add);
                
                log_instr(*cpu, format!("BEQ ${:04x}\t\tFROM: ${:04x}", addr, cpu.pc-2));
                branch::beq(cpu, addr);
            }

            _ => {
                eprintln!("Unknown opcode 0x{:02x} at offset 0x{:04x}", opcode, cpu.pc-1);
                exit(1);
            }
        }
    }
}