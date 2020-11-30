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

pub mod misc;
pub mod jump;
pub mod operator;
pub mod load;
pub mod cmp;
pub mod branch;
pub mod store;
pub mod logic;

#[cfg(test)]
mod load_test;
mod jump_test;
mod operator_test;
mod cmp_test;