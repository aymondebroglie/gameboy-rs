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
            0x01 => 3,
            other => panic!("Instruction {:2X} is not implemented", other),
        }
    }
    fn fetchword(&mut self) -> u16 {
        self.reg.inc_pc(2);
        self.mmu.rw()
    }

    fn ld<T, U>(&mut self, r1: T, r2: U) -> u32
    where
        Self: LD<T, U>,
    {
        self.overloaded_ld(r1, r2)
    }

}

impl LD<R8, R8> for CPU {
    fn overloaded_ld(&mut self, to: R8, from: R8) -> u32 {
        self.reg.set_8(to, self.reg.get_8(from));
        return 1;
    }
}

trait LD<T, U> {
    fn overloaded_ld(&mut self, t: T, u: U) -> u32 ;
}
