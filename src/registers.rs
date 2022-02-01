pub struct Registers {
    pub a: u8,
    pub b: u8,
   pub  c: u8,
    pub d: u8,
    pub e: u8,
     f: u8,
    pub h: u8,
    pub l: u8,
    sp: u16,
    pc: u16,
}

pub enum CpuFlag
{
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}
impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn flag(&self, flag : CpuFlag) -> bool {
        self.f & (flag as u8) > 0
    }


}

