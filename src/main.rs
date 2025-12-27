#![allow(dead_code)]

use std::{env::args, fs, process};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

static K8: usize = 0x2000;
static K16: usize = 0x4000;
static K32: usize = 0x8000;
static K64: usize = 0x10000;

struct Emulator {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    data: [u8; K64],
    sr: u8
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
enum Instruction {
    BRK         = 0x00,
    BPL         = 0x10,
    JSR         = 0x20,
    BMI         = 0x30,
    RTI         = 0x40,
    BVC         = 0x50,
    RTS         = 0x60,
    BVS         = 0x70,
    BCC         = 0x90,
    LDY_IMM     = 0xA0,
    BCS         = 0xB0,
    CPY_IMM     = 0xC0,
    BNE         = 0xD0,
    CPX_IMM     = 0xE0,
    BEQ         = 0xF0,

    ORA_X_IND   = 0x01,
    ORA_IND_Y   = 0x11,
    AND_X_IND   = 0x21,
    AND_IND_Y   = 0x31,
    EOR_X_IND   = 0x41,
    EOR_IND_Y   = 0x51,
    ADC_X_IND   = 0x61,
    ADC_IND_Y   = 0x71,
    STA_X_IND   = 0x81,
    STA_IND_Y   = 0x91,
    LDA_X_IND   = 0xA1,
    LDA_IND_Y   = 0xB1,
    CMP_X_IND   = 0xC1,
    CMP_IND_Y   = 0xD1,
    SBC_X_IND   = 0xE1,
    SBC_IND_Y   = 0xF1,

    LDX_IMM     = 0xA2,

    BIT_ZPG     = 0x24,
    STY_ZPG     = 0x84,
    STY_ZPG_X   = 0x94,
    LDY_ZPG     = 0xA4,
    LDY_ZPG_X   = 0xB4,
    CPY_ZPG     = 0xC4,
    CPX_ZPG     = 0xE4,

    ORA_ZPG     = 0x05,
    ORA_ZPG_X   = 0x15,
    AND_ZPG     = 0x25,
    AND_ZPG_X   = 0x35,
    EOR_ZPG     = 0x45,
    EOR_ZPG_X   = 0x55,
    ADC_ZPG     = 0x65,
    ADC_ZPG_X   = 0x75,
    STA_ZPG     = 0x85,
    STA_ZPG_X   = 0x95,
    LDA_ZPG     = 0xA5,
    LDA_ZPG_X   = 0xB5,
    CMP_ZPG     = 0xC5,
    CMP_ZPG_X   = 0xD5,
    SBC_ZPG     = 0xE5,
    SBC_ZPG_X   = 0xF5,

    ASL_ZPG     = 0x06,
    ASL_ZPG_X   = 0x16,
    ROL_ZPG     = 0x26,
    ROL_ZPG_X   = 0x36,
    LSR_ZPG     = 0x46,
    LSR_ZPG_X   = 0x56,
    ROR_ZPG     = 0x66,
    ROR_ZPG_X   = 0x76,
    STX_ZPG     = 0x86,
    STX_ZPG_Y   = 0x96,
    LDX_ZPG     = 0xA6,
    LDX_ZPG_Y   = 0xB6,
    DEC_ZPG     = 0xC6,
    DEC_ZPG_X   = 0xD6,
    INC_ZPG     = 0xE6,
    INC_ZPG_X   = 0xF6,

    PHP         = 0x08,
    CLC         = 0x18,
    PLP         = 0x28,
    SEC         = 0x38,
    PHA         = 0x48,
    CLI         = 0x58,
    PLA         = 0x68,
    SEI         = 0x78,
    DEY         = 0x88,
    TYA         = 0x98,
    TAY         = 0xA8,
    CLV         = 0xB8,
    INY         = 0xC8,
    CLD         = 0xD8,
    INX         = 0xE8,
    SED         = 0xF8,

    ORA_IMM     = 0x09,
    ORA_ABS_Y   = 0x19,
    AND_IMM     = 0x29,
    AND_ABS_Y   = 0x39,
    EOR_IMM     = 0x49,
    EOR_ABS_Y   = 0x59,
    ADC_IMM     = 0x69,
    ADC_ABS_Y   = 0x79,
    STA_ABS_Y   = 0x99,
    LDA_IMM     = 0xA9,
    LDA_ABS_Y   = 0xB9,
    CMP_IMM     = 0xC9,
    CMP_ABS_Y   = 0xD9,
    SBC_IMM     = 0xE9,
    SBC_ABS_Y   = 0xF9,

    ASL_A       = 0x0A,
    ROL_A       = 0x2A,
    LSR_A       = 0x4A,
    ROR_A       = 0x6A,
    TXA         = 0x8A,
    TXS         = 0x9A,
    TAX         = 0xAA,
    TSX         = 0xBA,
    DEX         = 0xCA,
    NOP         = 0xEA,

    BIT_ABS     = 0x2C,
    JMP_ABS     = 0x4C,
    JMP_IND     = 0x6C,
    STY_ABS     = 0x8C,
    LDY_ABS     = 0xAC,
    LDY_ABS_X   = 0xBC,
    CPY_ABS     = 0xCC,
    CPX_ABS     = 0xEC,

    ORA_ABS     = 0x0D,
    ORA_ABS_X   = 0x1D,
    AND_ABS     = 0x2D,
    AND_ABS_X   = 0x3D,
    EOR_ABS     = 0x4D,
    EOR_ABS_X   = 0x5D,
    ADC_ABS     = 0x6D,
    ADC_ABS_X   = 0x7D,
    STA_ABS     = 0x8D,
    STA_ABS_X   = 0x9D,
    LDA_ABS     = 0xAD,
    LDA_ABS_X   = 0xBD,
    CMP_ABS     = 0xCD,
    CMP_ABS_X   = 0xDD,
    SBC_ABS     = 0xED,
    SBC_ABS_X   = 0xFD,

    ASL_ABS     = 0x0E,
    ASL_ABS_X   = 0x1E,
    ROL_ABS     = 0x2E,
    ROL_ABS_X   = 0x3E,
    LSR_ABS     = 0x4E,
    LSR_ABS_X   = 0x5E,
    ROR_ABS     = 0x6E,
    ROR_ABS_X   = 0x7E,
    STX_ABS     = 0x8E,
    LDX_ABS     = 0xAE,
    LDX_ABS_Y   = 0xBE,
    DEC_ABS     = 0xCE,
    DEC_ABS_X   = 0xDE,
    INC_ABS     = 0xEE,
    INC_ABS_X   = 0xFE
}

#[repr(u8)]
enum SRMask {
    Carry       = 0b00000001,
    Zero        = 0b00000010,
    Interrupt   = 0b00000100,
    Decimal     = 0b00001000,
    Break       = 0b00010000,
    Reserved    = 0b00100000,
    Overflow    = 0b01000000,
    Negative    = 0b10000000
}

static B_R_MASK: u8 = 0b00110000;

#[derive(Debug)]
enum EErr {
    IllegalInstruction(u8),

    Break,
}

impl From<u8> for EErr {
    fn from(b: u8) -> Self {
        EErr::IllegalInstruction(b)
    }
}

impl Emulator {
    fn init(code: [u8; K32]) -> Emulator {
        let mut data: [u8; K64] = [0; K64];

        let mut i: usize = K32;
        for b in code {
            data[i] = b;
            i += 1;
        }

        let rv_low = data[0xfffc] as u16;
        let rv_high = data[0xfffd] as u16;
        let rv: u16 = rv_high << 8 | rv_low;

        Emulator {
            a: 0,
            x: 0,
            y: 0,
            pc: rv,
            sp: 0xff,
            data: data.clone(),
            sr: SRMask::Reserved as u8
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.data[self.pc as usize]
    }

    fn read_byte_at(&mut self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write_byte_at(&mut self, addr: u16, v: u8) {
        if addr > 0x8000 {
            return
        }
        self.data[addr as usize] = v;
    }

    fn read_addr(&mut self) -> u16 {
        let low = self.read_byte() as u16;
        let addr = self.pc;
        let high = self.read_byte_at(addr + 1) as u16;
        high << 8 | low
    }

    fn read_addr_at(&mut self, pos: u16) -> u16 {
        let low = self.read_byte_at(pos) as u16;
        let high = self.read_byte_at(pos.wrapping_add(1)) as u16;
        high << 8 | low
    }

    fn read_addr_from_stack(&mut self) -> u16 {
        let addr_low = self.pop_from_stack() as u16;
        let addr_high = self.pop_from_stack() as u16;
        addr_high << 8 | addr_low
    }

    fn read_instruction(&mut self) -> Result<Instruction, u8> {
        let b = self.read_byte();
        Instruction::from_u8(b).ok_or(b)
    }

    fn set_sr_bit(&mut self, b: SRMask, v: bool) {
        if v {
            self.sr |= b as u8;
        } else {
            self.sr &= !(b as u8);
        }
    }

    fn get_psr_bit(&mut self, b: SRMask) -> bool {
        self.sr & b as u8 != 0
    }

    fn push_to_stack(&mut self, b: u8) {
        self.data[0x100 | self.sp as usize] = b;
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop_from_stack(&mut self) -> u8 {
        let b = self.data[0x100 | (self.sp) as usize];
        self.sp = self.sp.wrapping_add(1);
        b
    }

    fn x_ind(&mut self) -> u16 {
        let ind = self.data[self.pc as usize].wrapping_add(self.x);
        self.read_addr_at(ind as u16)
    }

    fn ind_y(&mut self) -> u16 {
        let ind = self.data[self.pc as usize];
        self.read_addr_at(ind as u16).wrapping_add(self.y as u16)
    }

    fn set_nz(&mut self, v: u8) {
        self.set_sr_bit(SRMask::Zero, v == 0);
        self.set_sr_bit(SRMask::Negative, v != 0);
    }

    fn adc(&mut self, v: u8) {
        let c: u16 = if self.get_psr_bit(SRMask::Carry) {1} else {0};
        let true_sum = self.a as u16 + v as u16 + c;
        self.set_sr_bit(SRMask::Carry, true_sum > 0xff);
        let res: u8 = (true_sum & 0xff) as u8;
        self.set_sr_bit(SRMask::Overflow, false);
        // overflow check 
        if ((self.a & 0x80) == 0) & ((v & 0x80) == 0) { // both positive
            // set overflow bit if res is negative (2 positives cant make a negative)
            self.set_sr_bit(SRMask::Overflow, (res & 0x80) != 0); 
        }
        if ((self.a & 0x80) != 0) & ((v & 0x80) != 0) { // both negative
            // set overflow bit if res is positive (2 negatives cant make a positive)
            self.set_sr_bit(SRMask::Overflow, (res & 0x80) == 0)
        }
        self.a = res;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    fn branch(&mut self, cond: bool) {
        if cond {
            self.pc = self.pc.wrapping_add(1);
            return;
        }
        let offset = self.read_byte() as i8;
        self.pc = self.pc.wrapping_add(1);
        let idk = offset as i16;
        self.pc = self.pc.wrapping_add(idk as u16);
    }

    fn sbc(&mut self, v: u8) {
        let c: u16 = if self.get_psr_bit(SRMask::Carry) { 1 } else { 0 };
        let true_sum = self.a as u16 + (!v) as u16 + c;
        self.set_sr_bit(SRMask::Carry, true_sum > 0xff);
        let res = (true_sum & 0xff) as u8;
        let is_a_neg = (self.a & 0x80) != 0;
        let is_v_neg = (v & 0x80) != 0;
        let is_res_neg = (res & 0x80) != 0;
        self.set_sr_bit(SRMask::Overflow, false);
        if is_a_neg != is_v_neg { // fuck this shit man
            self.set_sr_bit(SRMask::Overflow, is_a_neg != is_res_neg);
        }
        self.a = res;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    fn cmp(&mut self, v: u8) {
        let res = self.a.wrapping_sub(v);
        self.set_nz(res);
        self.set_sr_bit(SRMask::Carry, self.a >= v);
        self.pc = self.pc.wrapping_add(1);
    }

    fn cpy(&mut self, v: u8) {
        let res = self.y.wrapping_sub(v);
        self.set_nz(res);
        self.set_sr_bit(SRMask::Carry, self.y >= v);
        self.pc = self.pc.wrapping_add(1);
    }

    fn cpx(&mut self, v: u8) {
        let res = self.x.wrapping_sub(v);
        self.set_nz(res);
        self.set_sr_bit(SRMask::Carry, self.x >= v);
        self.pc = self.pc.wrapping_add(1);
    }

    fn ora(&mut self, v: u8) {
        self.a = self.a | v;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    fn and(&mut self, v: u8) {
        self.a = self.a & v;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    fn eor(&mut self, v: u8) {
        self.a = self.a ^ v;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    fn lda(&mut self, v: u8) {
        self.a = v;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    fn ldx(&mut self, v: u8) {
        self.x = v;   
        self.set_nz(self.x);
        self.pc = self.pc.wrapping_add(1);
    }

    fn ldy(&mut self, v: u8) {
        self.y = v;   
        self.set_nz(self.y);
        self.pc = self.pc.wrapping_add(1);
    }

    fn bit(&mut self, v: u8) {
        self.set_sr_bit(SRMask::Zero, (self.a & v) == 0);
        self.set_sr_bit(SRMask::Negative, (v & SRMask::Negative as u8) != 0);
        self.set_sr_bit(SRMask::Overflow, (v & SRMask::Overflow as u8) != 0);
        self.pc = self.pc.wrapping_add(1);
    }

    fn sty(&mut self, addr: u16) {
        self.write_byte_at(addr, self.y);
        self.pc = self.pc.wrapping_add(1);
    }

    fn sta(&mut self, addr: u16) {
        self.write_byte_at(addr, self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    fn stx(&mut self, addr: u16) {
        self.write_byte_at(addr, self.x);
        self.pc = self.pc.wrapping_add(1);
    }

    fn rol(&mut self, v: u8) -> u8 {
        let c = if self.get_psr_bit(SRMask::Carry) {1} else {0};
        let v_shifted = (v as u16) << 1;
        let v_rotated = (v_shifted & 0xff) as u8 | c;
        self.set_sr_bit(SRMask::Carry, v_shifted > 0xff);
        self.set_nz(v_rotated);
        v_rotated
    }

    fn rol_addr(&mut self, addr: u16) {
        let v = self.read_byte_at(addr);
        let v_rotated = self.rol(v);
        self.write_byte_at(addr, v_rotated);
        self.pc = self.pc.wrapping_add(1);
    }

    fn ror(&mut self, v: u8) -> u8 {
        let c = if self.get_psr_bit(SRMask::Carry) {1} else {0};
        self.set_sr_bit(SRMask::Carry, (v & 0x01) != 0);
        let v_shifted = v >> 1;
        let v_rotated = v_shifted | c << 7;
        self.set_nz(v_rotated);
        v_rotated
    }

    fn ror_addr(&mut self, addr: u16) {
        let v = self.read_byte_at(addr);
        let v_rotated = self.ror(v);
        self.write_byte_at(addr, v_rotated);
        self.pc = self.pc.wrapping_add(1);
    }

    fn lsr(&mut self, v: u8) -> u8 {
        self.set_sr_bit(SRMask::Carry, (v & 0x01) != 0);
        let new_v = v >> 1;
        self.set_nz(new_v);
        new_v
    }

    fn lsr_addr(&mut self, addr: u16) {
        let v = self.read_byte_at(addr);
        let new_v = self.lsr(v);
        self.write_byte_at(addr, new_v);
        self.pc = self.pc.wrapping_add(1);
    }

    fn asl(&mut self, v: u8) -> u8 {
        let v_shifted = (v as u16) << 1;
        self.set_sr_bit(SRMask::Carry, v_shifted > 0xff);
        let new_v = (v_shifted & 0xff) as u8;
        self.set_nz(new_v);
        new_v
    }

    fn asl_addr(&mut self, addr: u16) {
        let v = self.read_byte_at(addr);
        let new_v = self.asl(v);
        self.write_byte_at(addr, new_v);
        self.pc = self.pc.wrapping_add(1);
    }

    fn dec(&mut self, addr: u16) {
        let v = self.read_byte_at(addr);
        let new_v = v.wrapping_sub(1);
        self.write_byte_at(addr, new_v);
        self.set_nz(new_v);
        self.pc = self.pc.wrapping_add(1);
    }

    fn inc(&mut self, addr: u16) {
        let v = self.read_byte_at(addr);
        let new_v = v.wrapping_add(1);
        self.write_byte_at(addr, new_v);
        self.set_nz(new_v);
        self.pc = self.pc.wrapping_add(1);
    }

    fn exec_instruction(&mut self) -> Result<(), EErr> {
        let inst = self.read_instruction()?;
        print!("0x{:04X}: {:?} ", self.pc, inst);
        self.pc = self.pc.wrapping_add(1);

        match inst {
            Instruction::BRK => {
                println!();
                Err(EErr::Break)
            }
            Instruction::BPL => {
                let cond = !self.get_psr_bit(SRMask::Negative);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::JSR => {
                let addr = self.read_addr();

                // keeping the logic as in the actual 6502
                // cause for some reason the other increment is done in RTS
                // as this is PC+2 which the original 6502 apparently did
                self.pc = self.pc.wrapping_add(1);
                let ret_high: u8 = (self.pc >> 8) as u8;
                let ret_low: u8 = (self.pc & 0xff) as u8;
                self.push_to_stack(ret_high);
                self.push_to_stack(ret_low);
                self.pc = addr;
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::BMI => {
                let cond = self.get_psr_bit(SRMask::Negative);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::RTI => {
                let new_sr = self.pop_from_stack();
                self.sr = new_sr;
                self.pc = self.read_addr_from_stack();
                println!("-> ${:04X}", self.pc);
                Ok(())
            }
            Instruction::BVC => {
                let cond = self.get_psr_bit(SRMask::Overflow);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::RTS => {
                // pc being incremented before the match doesnt matter here since we jump anyway
                let new_pc = self.read_addr_from_stack();

                // we increment pc by 1 so we dont execute the last byte of the address
                // that the JSR read
                self.pc = new_pc.wrapping_add(1);
                println!("-> ${:04X}", self.pc);
                Ok(())
            }
            Instruction::BVS => {
                let cond = !self.get_psr_bit(SRMask::Overflow);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::BCC => {
                let cond = self.get_psr_bit(SRMask::Carry);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::LDY_IMM => {
                let v = self.read_byte();
                self.ldy(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::BCS => {
                let cond = !self.get_psr_bit(SRMask::Carry);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::CPY_IMM => {
                let v: u8 = self.read_byte();
                self.cpy(v);
                println!("{:02X}", v);
                Ok(())
            }
            Instruction::BNE => {
                let cond = self.get_psr_bit(SRMask::Zero);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::CPX_IMM => {
                let v: u8 = self.read_byte();
                self.cpx(v);
                println!("{:02X}", v);
                Ok(())
            }
            Instruction::BEQ => {
                let cond = !self.get_psr_bit(SRMask::Zero);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::ORA_X_IND => {
                let addr = self.x_ind();
                let v = self.read_byte_at(addr);
                self.ora(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ORA_IND_Y => {
                let addr = self.ind_y();
                let v = self.read_byte_at(addr);
                self.ora(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::AND_X_IND => {
                let addr = self.x_ind();
                let v = self.read_byte_at(addr);
                self.and(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::AND_IND_Y => {
                let addr = self.ind_y();
                let v = self.read_byte_at(addr);
                self.and(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_X_IND => {
                let addr = self.x_ind();
                let v = self.read_byte_at(addr);
                self.eor(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_IND_Y => {
                let addr = self.ind_y();
                let v = self.read_byte_at(addr);
                self.eor(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_X_IND => {
                let addr = self.x_ind();
                let v = self.read_byte_at(addr);
                self.adc(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_IND_Y => {
                let addr = self.ind_y();
                let v = self.read_byte_at(addr);
                self.adc(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::STA_X_IND => {
                let addr = self.x_ind();
                self.sta(addr);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::STA_IND_Y => {
                let addr = self.ind_y();
                self.sta(addr);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_X_IND => {
                let addr = self.x_ind();
                let v = self.read_byte_at(addr);
                self.lda(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_IND_Y => {
                let addr = self.ind_y();
                let v = self.read_byte_at(addr);
                self.lda(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_X_IND => {
                let addr = self.x_ind();
                let v: u8 = self.read_byte_at(addr);
                self.cmp(v);
                println!("{:04X}", addr);
                Ok(())
            }
            Instruction::CMP_IND_Y => {
                let addr = self.ind_y();
                let v: u8 = self.read_byte_at(addr);
                self.cmp(v);
                println!("{:04X}", addr);
                Ok(())
            }
            Instruction::SBC_X_IND => {
                let addr = self.x_ind();
                let v = self.read_byte_at(addr);
                self.sbc(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::SBC_IND_Y => {
                let addr = self.ind_y();
                let v = self.read_byte_at(addr);
                self.sbc(v);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDX_IMM => {
                let v = self.read_byte();
                self.ldx(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::BIT_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.bit(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STY_ZPG => {
                let addr = self.read_byte();
                self.sty(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STY_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                self.sty(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDY_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.ldy(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDY_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                let v = self.read_byte_at(addr as u16);
                self.ldy(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::CPY_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.cpy(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::CPX_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.cpx(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ORA_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.ora(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ORA_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                let v = self.read_byte_at(addr as u16);
                self.ora(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::AND_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.and(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::AND_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                let v = self.read_byte_at(addr as u16);
                self.and(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::EOR_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.eor(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::EOR_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                let v = self.read_byte_at(addr as u16);
                self.eor(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ADC_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.adc(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ADC_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                let v = self.read_byte_at(addr as u16);
                self.adc(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STA_ZPG => {
                let addr = self.read_byte();
                self.sta(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STA_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                self.sta(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDA_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.lda(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDA_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                let v = self.read_byte_at(addr as u16);
                self.lda(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::CMP_ZPG => {
                let addr = self.read_byte();
                let v: u8 = self.read_byte_at(addr as u16);
                self.cmp(v);
                println!("{:02X}", addr);
                Ok(())
            }
            Instruction::CMP_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                let v: u8 = self.read_byte_at(addr as u16);
                self.cmp(v);
                println!("{:02X}", addr);
                Ok(())
            }
            Instruction::SBC_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.sbc(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::SBC_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                let v = self.read_byte_at(addr as u16);
                self.sbc(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ASL_ZPG => {
                let addr = self.read_byte();
                self.asl_addr(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ASL_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                self.asl_addr(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ROL_ZPG => {
                let addr = self.read_byte();
                self.rol_addr(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ROL_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                self.rol_addr(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LSR_ZPG => {
                let addr = self.read_byte();
                self.lsr_addr(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LSR_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                self.lsr_addr(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ROR_ZPG => {
                let addr = self.read_byte();
                self.ror_addr(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ROR_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                self.ror_addr(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STX_ZPG => {
                let addr = self.read_byte();
                self.stx(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STX_ZPG_Y => {
                let addr = self.read_byte().wrapping_add(self.y);
                self.stx(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDX_ZPG => {
                let addr = self.read_byte();
                let v = self.read_byte_at(addr as u16);
                self.ldx(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDX_ZPG_Y => {
                let addr = self.read_byte().wrapping_add(self.y);
                let v = self.read_byte_at(addr as u16);
                self.ldx(v);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::DEC_ZPG => {
                let addr = self.read_byte();
                self.dec(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::DEC_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                self.dec(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::INC_ZPG => {
                let addr = self.read_byte();
                self.inc(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::INC_ZPG_X => {
                let addr = self.read_byte().wrapping_add(self.x);
                self.inc(addr as u16);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::PHP => {
                self.push_to_stack(self.sr | SRMask::Break as u8);
                println!();
                Ok(())
            }
            Instruction::CLC => {
                self.sr = self.sr & !(SRMask::Carry as u8);
                println!();
                Ok(())
            }
            Instruction::PLP => {
                let stack_sr = self.pop_from_stack();
                self.sr = stack_sr & !B_R_MASK | self.sr & B_R_MASK;
                println!();
                Ok(())
            }
            Instruction::SEC => {
                self.sr = self.sr | SRMask::Carry as u8;
                println!();
                Ok(())
            }
            Instruction::PHA => {
                self.push_to_stack(self.a);
                println!();
                Ok(())
            }
            Instruction::CLI => {
                self.sr = self.sr & !(SRMask::Interrupt as u8);
                println!();
                Ok(())
            }
            Instruction::PLA => {
                self.a = self.pop_from_stack();
                self.set_nz(self.a);
                println!();
                Ok(())
            }
            Instruction::SEI => {
                self.sr = self.sr | SRMask::Interrupt as u8;
                println!();
                Ok(())
            }
            Instruction::DEY => {
                self.y = self.y.wrapping_sub(1);
                self.set_nz(self.y);
                println!();
                Ok(())
            }
            Instruction::TYA => {
                self.a = self.y;
                self.set_nz(self.a);
                println!();
                Ok(())
            }
            Instruction::TAY => {
                self.y = self.a;
                self.set_nz(self.y);
                println!();
                Ok(())
            }
            Instruction::CLV => {
                self.sr = self.sr & !(SRMask::Overflow as u8);
                println!();
                Ok(())
            }
            Instruction::INY => {
                self.y = self.y.wrapping_add(1);
                self.set_nz(self.y);
                println!();
                Ok(())
            }
            Instruction::CLD => {
                self.sr = self.sr & !(SRMask::Decimal as u8);
                println!();
                Ok(())
            }
            Instruction::INX => {
                self.x = self.x.wrapping_add(1);
                self.set_nz(self.x);
                println!();
                Ok(())
            }
            Instruction::SED => {
                self.sr = self.sr | SRMask::Decimal as u8;
                println!();
                Ok(())
            }
            Instruction::ORA_IMM => {
                let v = self.read_byte();
                self.ora(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::ORA_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                let v = self.read_byte_at(addr);
                self.ora(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::AND_IMM => {
                let v = self.read_byte();
                self.and(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::AND_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                let v = self.read_byte_at(addr);
                self.and(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_IMM => {
                let v = self.read_byte();
                self.eor(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::EOR_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                let v = self.read_byte_at(addr);
                self.eor(v);
                self.pc = self.pc.wrapping_add(1); 
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_IMM => {
                let v = self.read_byte();
                self.adc(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::ADC_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                let v = self.read_byte_at(addr);
                self.adc(v);
                self.pc = self.pc.wrapping_add(1); 
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::STA_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                self.sta(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_IMM => {
                let v = self.read_byte();
                self.lda(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::LDA_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                let v = self.read_byte_at(addr);
                self.lda(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_IMM => {
                let v = self.read_byte();
                self.cmp(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::CMP_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                let v = self.read_byte_at(addr);
                self.cmp(v);
                self.pc = self.pc.wrapping_add(1);
                println!("#${:04X}", v);
                Ok(())
            }
            Instruction::SBC_IMM => {
                let v = self.read_byte();
                self.sbc(v);
                println!("#${:02X}", v);
                Ok(())
            }
            Instruction::SBC_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                let v = self.read_byte_at(addr);
                self.sbc(v);
                self.pc = self.pc.wrapping_add(1); 
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ASL_A => {
                self.a = self.asl(self.a);
                println!("A");
                Ok(())
            }
            Instruction::ROL_A => {
                self.a = self.rol(self.a);
                println!("A");
                Ok(())
            }
            Instruction::LSR_A => {
                self.a = self.lsr(self.a);
                println!("A");
                Ok(())
            }
            Instruction::ROR_A => {
                self.a = self.ror(self.a);
                println!("A");
                Ok(())
            }
            Instruction::TXA => {
                self.a = self.x;
                self.set_nz(self.a);
                println!();
                Ok(())
            }
            Instruction::TXS => {
                self.sp = self.x;
                println!();
                Ok(())
            }
            Instruction::TAX => {
                self.x = self.a;
                self.set_nz(self.x);
                println!();
                Ok(())
            }
            Instruction::TSX => {
                self.x = self.sp;
                self.set_nz(self.x);
                println!();
                Ok(())
            }
            Instruction::DEX => {
                self.x = self.x.wrapping_sub(1);
                self.set_nz(self.x);
                println!();
                Ok(())
            }
            Instruction::NOP => {
                // literally do nothing, since the PC is incremented
                // before the match starts
                println!();
                Ok(())
            }
            Instruction::BIT_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.bit(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::JMP_ABS => {
                let addr = self.read_addr();
                self.pc = addr;
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::JMP_IND => {
                let addr = self.read_addr();
                let dest = self.read_addr_at(addr);
                self.pc = dest;
                println!("${:04X}", dest);
                Ok(())
            }
            Instruction::STY_ABS => {
                let addr = self.read_addr();
                self.sty(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDY_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.ldy(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDY_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                let v = self.read_byte_at(addr);
                self.ldy(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::CPY_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.cpy(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::CPX_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.cpx(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ORA_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.ora(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ORA_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                let v = self.read_byte_at(addr);
                self.ora(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::AND_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.and(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::AND_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                let v = self.read_byte_at(addr);
                self.and(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.eor(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                let v = self.read_byte_at(addr);
                self.eor(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.adc(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                let v = self.read_byte_at(addr);
                self.adc(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::STA_ABS => {
                let addr = self.read_addr();
                self.sta(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::STA_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                self.sta(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.lda(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                let v = self.read_byte_at(addr);
                self.lda(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.cmp(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                let v = self.read_byte_at(addr);
                self.cmp(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::SBC_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.sbc(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::SBC_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                let v = self.read_byte_at(addr);
                self.sbc(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ASL_ABS => {
                let addr = self.read_addr();
                self.asl_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ASL_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                self.asl_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ROL_ABS => {
                let addr = self.read_addr();
                self.rol_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ROL_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                self.rol_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LSR_ABS => {
                let addr = self.read_addr();
                self.lsr_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LSR_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                self.lsr_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ROR_ABS => {
                let addr = self.read_addr();
                self.ror_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ROR_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                self.ror_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::STX_ABS => {
                let addr = self.read_addr();
                self.stx(addr);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDX_ABS => {
                let addr = self.read_addr();
                let v = self.read_byte_at(addr);
                self.ldx(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDX_ABS_Y => {
                let addr = self.read_addr().wrapping_add(self.y as u16);
                let v = self.read_byte_at(addr);
                self.ldx(v);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::DEC_ABS => {
                let addr = self.read_addr();
                self.dec(addr);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::DEC_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                self.dec(addr);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::INC_ABS => {
                let addr = self.read_addr();
                self.inc(addr);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::INC_ABS_X => {
                let addr = self.read_addr().wrapping_add(self.x as u16);
                self.inc(addr);
                println!("${:04X}", addr);
                Ok(())
            }
        }
    }

    fn run(&mut self) {
        'end: loop {
            if let Some(e) = self.exec_instruction().err() {
                match e {
                    EErr::IllegalInstruction(opcode) => {
                        eprintln!("Illegal Instruction: 0x{:02X}", opcode);
                        break 'end;
                    }
                    EErr::Break => {
                        break 'end;
                    }
                }
            }
        }
    }
}

fn load_rom(path: &str, rom: &mut [u8; K32]) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|e| format!("IO Error: {}", e))?;

    for (i, byte) in bytes.iter().enumerate() {
        if i < K32 {
            rom[i as usize] = *byte;
        } else {
            return Err("ROM is larger than 32KiB".to_string())
        }
    }
    Ok(())
    
}

fn main() {
    let mut a = args().into_iter();
    let _program_name = a.next();
    let rom_name = a.next().expect("You need to supply a 32KiB ROM");
    let mut code: [u8; K32] = [0x00; K32];
    
    load_rom(&rom_name, &mut code).unwrap_or_else(|e| {
        eprintln!("[ERROR]: {}", e);
        process::exit(1);
    });
    
    let mut e = Emulator::init(code);
    e.run();
}

// TODO:    maybe add WDC extensions as an option? (are conditional enum fields a thing)
//          actually probably just make it a flag to the compiler and check upon encounter
// TODO:    add an option to replicate the JMP_IND page boundary bug