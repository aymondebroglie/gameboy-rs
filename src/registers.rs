pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum CpuFlag {
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

    pub fn get_8(&self, r: R8) -> u8 {
        match r {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
        }
    }

    pub fn get_16(&self, r: R16) -> u16 {
        match r {
            R16::AF => self.join(self.a, self.f),
            R16::BC => self.join(self.b , self.c ),
            R16::DE => self.join(self.d,self.e),
            R16::HL => self.join(self.h, self.l ),
            R16::SP => self.sp,
            R16::PC => self.pc,
        }
    }

    pub fn set_8(&mut self, r :R8, v : u8 ){
       match r{
           R8::A => {self.a = v}
           R8::B => {self.b = v }
           R8::C => {self.c = v }
           R8::D => {self.d = v}
           R8::E => {self.e = v}
           R8::H => {self.h = v}
           R8::L => {self.l = v}
       }
    }

    pub fn set_16(&mut self , r : R16 , v : u16){
        match r {
            R16::AF => {let s = self.split(v); self.a = s.0; self.f = s.1}
            R16::BC => {let s = self.split(v); self.f = s.0; self.c = s.1}
            R16::DE => {let s = self.split(v); self.d = s.0; self.e = s.1}
            R16::HL => {let s = self.split(v); self.h = s.0; self.l = s.1}
            R16::SP => {self.sp = v}
            R16::PC => {self.pc = v}
        }
    }

    fn split(&self ,v : u16) -> (u8, u8){
       ( (v >> 8) as u8, (v & 0b11111111) as u8)
    }

    pub fn flag(&self, flag: CpuFlag) -> bool {
        self.f & (flag as u8) > 0
    }

    fn join(&self, high: u8, low: u8) -> u16 {
        return (high as u16) << 8 | (low as u16);
    }

    pub fn inc_pc(&mut self , i : u16){
        self.pc += i;
    }
}
