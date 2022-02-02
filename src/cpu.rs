use crate::mmu::MMU;
use crate::registers::*;

struct CPU {
    reg: Registers,
    mmu: MMU,
}

impl CPU {
    fn read() {}
    fn operation(&mut self, opcode: u16) -> u32 {
        match opcode {
            0x00 => 1,
            0x01 => {3}
            other => panic!("Instruction {:2X} is not implemented", other),
        }
    }
    fn fetchword(&mut self) -> u16 {
        self.reg.inc_pc( 2);
        self.mmu.rw()
    }
}
