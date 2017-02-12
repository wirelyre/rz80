use RegT;

/// CPU carry flag
pub const CF: RegT = 1 << 0;
/// CPU add/subtract flag
pub const NF: RegT = 1 << 1;
/// CPU overflow flag (same as parity)
pub const VF: RegT = 1 << 2;
/// CPU parity flag (same as overflow)
pub const PF: RegT = 1 << 2;
/// CPU undocumented 'X' flag
pub const XF: RegT = 1 << 3;
/// CPU half carry flag
pub const HF: RegT = 1 << 4;
/// CPU undocumented 'Y' flag
pub const YF: RegT = 1 << 5;
/// CPU zero flag
pub const ZF: RegT = 1 << 6;
/// CPU sign flag
pub const SF: RegT = 1 << 7;

#[derive(Copy, Clone)]
pub enum Register8 {
    B, C, D, E, H, L, A, F,
    IXH, IXL, IYH, IYL,
    SPH, SPL, WZH, WZL,
    B_, C_, D_, E_, H_, L_, A_, F_,
    WZH_, WZL_,
}
const NUM_REGS: usize = 26;

#[derive(Copy, Clone)]
pub enum Register16 {
    BC = 0, DE = 2, HL = 4, AF = 6,
    IX = 8, IY = 10, SP = 12, WZ = 14,
    BC_ = 16, DE_ = 18, HL_ = 20, AF_ = 22,
    WZ_ = 24,
}

/// CPU register access
///
/// # Examples
///
/// set the PC and SP registers:
///
/// ```
/// use rz80::CPU;
/// use rz80::Register16::SP;
///
/// let mut cpu = CPU::new();
/// cpu.reg.set_pc(0x0200);
/// cpu.reg.set16(SP, 0x01C0);
/// ```
///
/// get the B, C and BC registers
///
/// ```
/// use rz80::CPU;
/// use rz80::Register8::{B, C};
/// use rz80::Register16::BC;
///
/// let cpu = CPU::new();
/// let b = cpu.reg.get8(B);
/// let c = cpu.reg.get8(C);
/// let bc = cpu.reg.get16(BC);
/// println!("B: {}, C: {}, BC: {}", b, c, bc);
/// ```
/// 8- or 16-bit wraparound happens during the set operation:
///
/// ```
/// use rz80::CPU;
/// use rz80::Register8::A;
/// use rz80::Register16::HL;
///
/// let mut cpu = CPU::new();
///
/// cpu.reg.set8(A, 0xFF);
/// let a = cpu.reg.get8(A) + 1;
/// cpu.reg.set8(A, a);
/// assert_eq!(cpu.reg.get8(A), 0x00);
///
/// cpu.reg.set16(HL, 0x0000);
/// let hl = cpu.reg.get16(HL) - 1;
/// cpu.reg.set16(HL, hl);
/// assert_eq!(cpu.reg.get16(HL), 0xFFFF);
/// ```
pub struct Registers {
    reg: [u8; NUM_REGS],
    r_pc: u16,

    pub i: RegT,
    pub r: RegT,
    pub im: RegT,

    m_r: [Register8; 8],
    m_r2: [Register8; 8],
    m_sp: [Register16; 4],
    m_af: [Register16; 4],
}

use self::Register8::*;
use self::Register16::*;

impl Registers {
    /// initialize a new Registers object
    pub fn new() -> Registers {
        Registers {
            reg: [0; NUM_REGS],
            r_pc: 0,
            i: 0,
            r: 0,
            im: 0,
            m_r: [B, C, D, E, H, L, F, A],
            m_r2: [B, C, D, E, H, L, F, A],
            m_sp: [BC, DE, HL, SP],
            m_af: [BC, DE, HL, AF],
        }
    }

    /// perform a CPU reset (clears some, but not all registers)
    pub fn reset(&mut self) {
        self.r_pc = 0;
        self.set16(WZ, 0);
        self.im = 0;
        self.i = 0;
        self.r = 0;
    }

    /// get content of 8-bit register
    #[inline(always)]
    pub fn get8(&self, reg: Register8) -> RegT {
        self.reg[reg as usize] as RegT
    }

    /// set content of 8-bit register
    #[inline(always)]
    pub fn set8(&mut self, reg: Register8, v: RegT) {
        self.reg[reg as usize] = v as u8;
    }

    /// get content of register pair or 16-bit register
    #[inline(always)]
    pub fn get16(&self, reg: Register16) -> RegT {
        let h = reg as usize;
        let l = h + 1;

        (self.reg[h] as RegT) << 8 | self.reg[l] as RegT
    }

    /// set content of register pair or 16-bit register
    #[inline(always)]
    pub fn set16(&mut self, reg: Register16, v: RegT) {
        let h = reg as usize;
        let l = h + 1;

        self.reg[h] = (v >> 8) as u8;
        self.reg[l] = v as u8;
    }

    /// get content of PC register
    #[inline(always)]
    pub fn pc(&self) -> RegT {
        self.r_pc as RegT
    }

    /// set content of PC register
    #[inline(always)]
    pub fn set_pc(&mut self, v: RegT) {
        self.r_pc = v as u16;
    }

    /// increment the PC register by some value
    #[inline(always)]
    pub fn inc_pc(&mut self, inc: u16) {
        self.r_pc = self.r_pc.wrapping_add(inc);
    }

    /// decrement the PC register by some value
    #[inline(always)]
    pub fn dec_pc(&mut self, dec: u16) {
        self.r_pc = self.r_pc.wrapping_sub(dec);
    }

    /// get 8-bit register by index (where index is 3-bit register id from Z80 instruction)
    #[inline(always)]
    pub fn r8(&self, i: usize) -> RegT {
        let r = self.m_r[i];
        self.get8(r)
    }

    /// set 8-bit register by index (where index is 3-bit register id from Z80 instruction)
    #[inline(always)]
    pub fn set_r8(&mut self, i: usize, v: RegT) {
        let r = self.m_r[i];
        self.set8(r, v);
    }

    /// get 8-bit register by index, H,L never patched to IXH,IXL,IYH,IYL
    #[inline(always)]
    pub fn r8i(&self, i: usize) -> RegT {
        let r = self.m_r2[i];
        self.get8(r)
    }

    /// set 8-bit register by index, H,L never patched to IXH,IXL,IYH,IYL
    #[inline(always)]
    pub fn set_r8i(&mut self, i: usize, v: RegT) {
        let r = self.m_r2[i];
        self.set8(r, v);
    }

    /// get 16-bit register by 2-bit index with mapping through SP-table
    #[inline(always)]
    pub fn r16sp(&self, i: usize) -> RegT {
        let r = self.m_sp[i];
        self.get16(r)
    }

    /// set 16-bit register by 2-bit index with mapping through SP-table
    #[inline(always)]
    pub fn set_r16sp(&mut self, i: usize, v: RegT) {
        let r = self.m_sp[i];
        self.set16(r, v);
    }

    /// get 16-bit register by 2-bit index with mapping through AF-table
    #[inline(always)]
    pub fn r16af(&self, i: usize) -> RegT {
        let r = self.m_af[i];
        self.get16(r)
    }

    /// set 16-bit register by 2-bit index with mapping through AF-table
    #[inline(always)]
    pub fn set_r16af(&mut self, i: usize, v: RegT) {
        let r = self.m_af[i];
        self.set16(r, v);
    }

    /// swap two 16-bit registers
    pub fn swap(&mut self, r1: Register16, r2: Register16) {
        let v1 = self.get16(r1);
        let v2 = self.get16(r2);
        self.set16(r1, v2);
        self.set16(r2, v1);
    }

    /// patch register mapping tables for use of IX instead of HL
    pub fn patch_ix(&mut self) {
        self.m_r[H as usize] = IXH;
        self.m_r[L as usize] = IXL;
        self.m_sp[2] = IX;
        self.m_af[2] = IX;
    }

    /// patch register mapping tables for use of IY instead of HL
    pub fn patch_iy(&mut self) {
        self.m_r[H as usize] = IYH;
        self.m_r[L as usize] = IYL;
        self.m_sp[2] = IY;
        self.m_af[2] = IY;
    }

    /// unpatch register mapping tables to use HL instead of IX/IY
    pub fn unpatch(&mut self) {
        self.m_r[H as usize] = H;
        self.m_r[L as usize] = L;
        self.m_sp[2] = HL;
        self.m_af[2] = HL;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Register8::*;
    use super::Register16::*;

    #[test]
    fn new() {
        let reg = Registers::new();
        assert_eq!(reg.get8(A), 0);
        assert_eq!(reg.get8(F), 0);
        assert_eq!(reg.get8(B), 0);
        assert_eq!(reg.get8(C), 0);
        assert_eq!(reg.get8(D), 0);
        assert_eq!(reg.get8(E), 0);
        assert_eq!(reg.get8(H), 0);
        assert_eq!(reg.get8(L), 0);
        assert_eq!(reg.get16(AF), 0);
        assert_eq!(reg.get16(AF_), 0);
        assert_eq!(reg.get16(BC), 0);
        assert_eq!(reg.get16(BC_), 0);
        assert_eq!(reg.get16(DE), 0);
        assert_eq!(reg.get16(DE_), 0);
        assert_eq!(reg.get16(HL), 0);
        assert_eq!(reg.get16(HL_), 0);
        assert_eq!(reg.get16(WZ), 0);
        assert_eq!(reg.get16(WZ_), 0);
        assert_eq!(reg.get16(IX), 0);
        assert_eq!(reg.get16(IY), 0);
        assert_eq!(reg.pc(), 0);
        assert_eq!(reg.get16(SP), 0);
        assert_eq!(reg.r, 0);
        assert_eq!(reg.i, 0);
        assert_eq!(reg.im, 0);
    }

    #[test]
    fn set_get() {
        let mut reg = Registers::new();
        reg.set8(A, 0x12);
        reg.set8(F, 0x34);
        assert_eq!(reg.get8(A), 0x12);
        assert_eq!(reg.get8(F), 0x34);
        assert_eq!(reg.get16(AF), 0x1234);
        reg.set16(AF, 0x2345);
        assert_eq!(reg.get16(AF), 0x2345);
        assert_eq!(reg.get8(A), 0x23);
        assert_eq!(reg.get8(F), 0x45);
        reg.set8(B, 0x34);
        reg.set8(C, 0x56);
        assert_eq!(reg.get8(B), 0x34);
        assert_eq!(reg.get8(C), 0x56);
        assert_eq!(reg.get16(BC), 0x3456);
        reg.set8(D, 0x78);
        reg.set8(E, 0x9A);
        assert_eq!(reg.get16(DE), 0x789A);
        assert_eq!(reg.get8(D), 0x78);
        assert_eq!(reg.get8(E), 0x9A);
        reg.set8(H, 0xAB);
        reg.set8(L, 0xCD);
        assert_eq!(reg.get16(HL), 0xABCD);
        assert_eq!(reg.get8(H), 0xAB);
        assert_eq!(reg.get8(L), 0xCD);
        reg.set16(IX, 0x0102);
        assert_eq!(reg.get16(IX), 0x0102);
        reg.set16(IY, 0x0304);
        assert_eq!(reg.get16(IY), 0x0304);
        reg.set_pc(0x1122);
        assert_eq!(reg.pc(), 0x1122);
        reg.set16(SP, 0x3344);
        assert_eq!(reg.get16(SP), 0x3344);
    }
}
