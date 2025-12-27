#![allow(dead_code)]

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
    IND_ABS_X   = 0xFE
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

        let rv_low: u16 = data[0xfffc] as u16;
        let rv_high: u16 = data[0xfffd] as u16;
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
        self.data[usize::from(self.pc)]
    }

    fn read_byte_at(&mut self, addr: u16) -> u8 {
        self.data[usize::from(addr)]
    }

    fn read_addr(&mut self) -> u16 {
        let low = u16::from(self.read_byte());
        let addr: u16 = self.pc;
        let high = u16::from(self.read_byte_at(addr + 1));
        high << 8 | low
    }

    fn read_addr_1b(&mut self, pos: u8) -> u16 {
        let low = u16::from(self.read_byte_at(pos as u16));
        let high = u16::from(self.read_byte_at(pos.wrapping_add(1) as u16));
        high << 8 | low
    }

    fn read_instruction(&mut self) -> Result<Instruction, u8> {
        let b: u8 = self.read_byte();
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

    fn branch(&mut self, cond: bool) {
        if cond {
            self.pc = self.pc.wrapping_add(1);
            return;
        }
        let offset = self.read_byte() as i8;
        self.pc = self.pc.wrapping_add(1);
        let idk = offset as i16;
        self.pc = self.pc.wrapping_add(idk as u16);
        println!("#${:04X}", self.pc);
    }

    fn x_ind(&mut self) -> u16 {
        let ind = self.data[self.pc as usize].wrapping_add(self.x);
        let addr = self.read_addr_1b(ind);
        addr
    }

    fn ind_y(&mut self) -> u16 {
        let ind = self.data[self.pc as usize];
        let addr = self.read_addr_1b(ind).wrapping_add(self.y as u16);
        addr
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
    }

    fn sbc(&mut self, v: u8) {
        let c: u16 = if self.get_psr_bit(SRMask::Carry) { 1 } else { 0 };
        let true_sum = self.a as u16 + (!v) as u16 + c;
        self.set_sr_bit(SRMask::Carry, true_sum > 0xff);
        let res = (true_sum & 0xff) as u8;
        let is_a_neg = (self.a & 0x80) != 0;
        let is_val_neg = (v & 0x80) != 0;
        let is_res_neg = (res & 0x80) != 0;
        self.set_sr_bit(SRMask::Overflow, false);
        if is_a_neg != is_val_neg { // fuck this shit man
            self.set_sr_bit(SRMask::Overflow, is_a_neg != is_res_neg);
        }
        self.a = res;
        self.set_nz(self.a);
    }

    fn exec_instruction(&mut self) -> Result<(), EErr> {
        let inst = self.read_instruction()?;
        print!("0x{:04X}: {:?} ", self.pc, inst);

        match inst {
            Instruction::BRK => {
                println!();
                return Err(EErr::Break);
            }
            Instruction::BPL => {
                self.pc = self.pc.wrapping_add(1);
                let cond = !self.get_psr_bit(SRMask::Negative);
                self.branch(cond);
                Ok(())
            }
            Instruction::JSR => {
                self.pc = self.pc.wrapping_add(1);
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
                self.pc = self.pc.wrapping_add(1);
                let cond = self.get_psr_bit(SRMask::Negative);
                self.branch(cond);
                Ok(())
            }
            Instruction::RTI => {
                self.pc = self.pc.wrapping_add(1);
                let new_sr = self.pop_from_stack();
                self.sr = new_sr;
                let new_pc_low = self.pop_from_stack() as u16;
                let new_pc_high = self.pop_from_stack() as u16;
                self.pc = new_pc_high << 8 | new_pc_low;
                println!();
                Ok(())
            }
            Instruction::BVC => {
                self.pc = self.pc.wrapping_add(1);
                let cond = self.get_psr_bit(SRMask::Overflow);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::RTS => {
                let new_pc_low = self.pop_from_stack() as u16;
                let new_pc_high = self.pop_from_stack() as u16;
                let new_pc = new_pc_high << 8 | new_pc_low;

                // we increment pc by 1 so we dont execute the last byte of the address
                // that the JSR read
                self.pc = new_pc.wrapping_add(1);
                Ok(())
            }
            Instruction::BVS => {
                self.pc = self.pc.wrapping_add(1);
                let cond =  !self.get_psr_bit(SRMask::Overflow);
                self.branch(cond);
                Ok(())
            }
            Instruction::BCC => {
                self.pc = self.pc.wrapping_add(1);
                let cond =  self.get_psr_bit(SRMask::Carry);
                self.branch(cond);
                Ok(())
            }
            Instruction::LDY_IMM => {
                self.pc = self.pc.wrapping_add(1);
                let val = self.read_byte();
                println!("#${:02X}", val);
                self.y = val;
                self.pc = self.pc.wrapping_add(1);
                Ok(())
            }
            Instruction::BCS => {
                self.pc = self.pc.wrapping_add(1);
                let cond = !self.get_psr_bit(SRMask::Carry);
                self.branch(cond);
                Ok(())
            }
            Instruction::CPY_IMM => {
                self.pc = self.pc.wrapping_add(1);
                let val: u8 = self.read_byte();
                let res = self.y.wrapping_sub(val);
                self.set_nz(res);
                self.set_sr_bit(SRMask::Carry, self.y >= val);
                self.pc = self.pc.wrapping_add(1);
                println!("{:02X}", val);
                Ok(())
            }
            Instruction::BNE => {
                self.pc = self.pc.wrapping_add(1);
                let cond = self.get_psr_bit(SRMask::Zero);
                self.branch(cond);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::CPX_IMM => {
                self.pc = self.pc.wrapping_add(1);
                let val: u8 = self.read_byte();
                let res = self.x.wrapping_sub(val);
                self.set_nz(res);
                self.set_sr_bit(SRMask::Carry, self.y >= val);
                self.pc = self.pc.wrapping_add(1);
                println!("{:02X}", val);
                Ok(())
            }
            Instruction::BEQ => {
                self.pc = self.pc.wrapping_add(1);
                let cond = !self.get_psr_bit(SRMask::Zero);
                self.branch(cond);
                Ok(())
            }
            Instruction::ORA_X_IND => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.x_ind();
                let val = self.data[addr as usize];
                self.a = self.a | val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ORA_IND_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.ind_y();
                let val = self.data[addr as usize];
                self.a = self.a | val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::AND_X_IND => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.x_ind();
                let val = self.data[addr as usize];
                self.a = self.a & val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::AND_IND_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.ind_y();
                let val = self.data[addr as usize];
                self.a = self.a & val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_X_IND => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.x_ind();
                let val = self.data[addr as usize];
                self.a = self.a ^ val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_IND_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.ind_y();
                let val = self.data[addr as usize];
                self.a = self.a ^ val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_X_IND => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.x_ind();
                let val = self.data[addr as usize];
                self.adc(val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_IND_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.ind_y();
                let val = self.data[addr as usize];
                self.adc(val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::STA_X_IND => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.x_ind();
                self.data[addr as usize] = self.a;
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::STA_IND_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.ind_y();
                self.data[addr as usize] = self.a;
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_X_IND => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.x_ind();
                let val = self.data[addr as usize];
                self.a = val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_IND_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.ind_y();
                let val = self.data[addr as usize];
                self.a = val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_X_IND => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.x_ind();
                let val: u8 = self.data[addr as usize];
                let res = self.a.wrapping_sub(val);
                self.set_nz(res);
                self.set_sr_bit(SRMask::Carry, self.a >= val);
                self.pc = self.pc.wrapping_add(1);
                println!("{:04X}", addr);
                Ok(())
            }
            Instruction::CMP_IND_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.ind_y();
                let val: u8 = self.data[addr as usize];
                let res = self.a.wrapping_sub(val);
                self.set_nz(res);
                self.set_sr_bit(SRMask::Carry, self.a >= val);
                self.pc = self.pc.wrapping_add(1);
                println!("{:04X}", addr);
                Ok(())
            }
            Instruction::SBC_X_IND => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.x_ind();
                let val = self.data[addr as usize];
                self.sbc(val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::SBC_IND_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.ind_y();
                let val = self.data[addr as usize];
                self.sbc(val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:04X}", addr);
                Ok(())
            }
            Instruction::LDX_IMM => {
                self.pc = self.pc.wrapping_add(1);
                let val = self.read_byte();
                self.x = val;   
                self.pc = self.pc.wrapping_add(1);
                println!("#${:02X}", val);
                Ok(())
            }
            Instruction::BIT_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let zpg_addr = self.read_byte();
                let val = self.data[zpg_addr as usize];
                self.set_sr_bit(SRMask::Zero, (self.a & val) == 0);
                self.set_sr_bit(SRMask::Negative, (val & SRMask::Negative as u8) != 0);
                self.set_sr_bit(SRMask::Overflow, (val & SRMask::Overflow as u8) != 0);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", zpg_addr);
                Ok(())
            }
            Instruction::STY_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                self.data[addr as usize] = self.y;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STY_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                self.data[addr as usize] = self.y;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDY_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                self.y = self.data[addr as usize];
                self.pc = self.pc.wrapping_add(1);
                self.set_nz(self.y);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDY_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                self.y = self.data[addr as usize];
                self.pc = self.pc.wrapping_add(1);
                self.set_nz(self.y);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::CPY_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                let res = self.y.wrapping_sub(val);
                self.set_nz(res);
                self.set_sr_bit(SRMask::Carry, self.y >= val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::CPX_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                let res = self.x.wrapping_sub(val);
                self.set_nz(res);
                self.set_sr_bit(SRMask::Carry, self.x >= val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ORA_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                self.a = self.a | val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ORA_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                self.a = self.a | val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::AND_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                self.a = self.a & val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::AND_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                self.a = self.a & val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::EOR_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                self.a = self.a ^ val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::EOR_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                self.a = self.a ^ val;
                self.set_nz(self.a);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ADC_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                self.adc(val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ADC_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                self.adc(val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STA_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                self.data[addr as usize] = self.a;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STA_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                self.data[addr as usize] = self.a;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDA_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                self.a = self.data[addr as usize];
                self.pc = self.pc.wrapping_add(1);
                self.set_nz(self.y);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDA_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                self.a = self.data[addr as usize];
                self.pc = self.pc.wrapping_add(1);
                self.set_nz(self.y);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::CMP_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val: u8 = self.data[addr as usize];
                let res = self.a.wrapping_sub(val);
                self.set_nz(res);
                self.set_sr_bit(SRMask::Carry, self.a >= val);
                self.pc = self.pc.wrapping_add(1);
                println!("{:02X}", addr);
                Ok(())
            }
            Instruction::CMP_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val: u8 = self.data[addr as usize];
                let res = self.a.wrapping_sub(val);
                self.set_nz(res);
                self.set_sr_bit(SRMask::Carry, self.a >= val);
                self.pc = self.pc.wrapping_add(1);
                println!("{:02X}", addr);
                Ok(())
            }
            Instruction::SBC_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                self.sbc(val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::SBC_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                self.sbc(val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ASL_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                let val_shifted = (val as u16) << 1;
                self.set_sr_bit(SRMask::Carry, val_shifted > 0xff);
                let new_val = (val_shifted & 0xff) as u8;
                self.set_nz(new_val);
                self.data[addr as usize] = new_val;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ASL_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                let val_shifted = (val as u16) << 1;
                self.set_sr_bit(SRMask::Carry, val_shifted > 0xff);
                let new_val = (val_shifted & 0xff) as u8;
                self.set_nz(new_val);
                self.data[addr as usize] = new_val;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ROL_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                let c = if self.get_psr_bit(SRMask::Carry) {1} else {0};
                let val_shifted = (val as u16) << 1;
                let val_rotated = (val_shifted & 0xff) as u8 | c;
                self.set_sr_bit(SRMask::Carry, val_shifted > 0xff);
                self.set_nz(val_rotated);
                self.data[addr as usize] = val_rotated;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ROL_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                let c = if self.get_psr_bit(SRMask::Carry) {1} else {0};
                let val_shifted = (val as u16) << 1;
                let val_rotated = (val_shifted & 0xff) as u8 | c;
                self.set_sr_bit(SRMask::Carry, val_shifted > 0xff);
                self.set_nz(val_rotated);
                self.data[addr as usize] = val_rotated;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LSR_ZPG => {
                 self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                self.set_sr_bit(SRMask::Carry, (val & 0x01) != 0);
                let new_val = val >> 1;
                self.set_nz(new_val);
                self.data[addr as usize] = new_val;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LSR_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                self.set_sr_bit(SRMask::Carry, (val & 0x01) != 0);
                let new_val = val >> 1;
                self.set_nz(new_val);
                self.data[addr as usize] = new_val;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ROR_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                let c = if self.get_psr_bit(SRMask::Carry) {1} else {0};
                self.set_sr_bit(SRMask::Carry, (val & 0x01) != 0);
                let val_shifted = val >> 1;
                let val_rotated = val_shifted | c << 7;
                self.set_nz(val_rotated);
                self.data[addr as usize] = val_rotated;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::ROR_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                let c = if self.get_psr_bit(SRMask::Carry) {1} else {0};
                self.set_sr_bit(SRMask::Carry, (val & 0x01) != 0);
                let val_shifted = val >> 1;
                let val_rotated = val_shifted | c << 7;
                self.set_nz(val_rotated);
                self.data[addr as usize] = val_rotated;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STX_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                self.data[addr as usize] = self.x;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::STX_ZPG_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.y);
                self.data[addr as usize] = self.x;
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDX_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                self.x = self.data[addr as usize];
                self.pc = self.pc.wrapping_add(1);
                self.set_nz(self.x);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::LDX_ZPG_Y => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.y);
                self.x = self.data[addr as usize];
                self.pc = self.pc.wrapping_add(1);
                self.set_nz(self.x);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::DEC_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                let new_val = val.wrapping_sub(1);
                self.data[addr as usize] = new_val;
                self.set_nz(new_val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::DEC_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                let new_val = val.wrapping_sub(1);
                self.data[addr as usize] = new_val;
                self.set_nz(new_val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::INC_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte();
                let val = self.data[addr as usize];
                let new_val = val.wrapping_add(1);
                self.data[addr as usize] = new_val;
                self.set_nz(new_val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }
            Instruction::INC_ZPG_X => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_byte().wrapping_add(self.x);
                let val = self.data[addr as usize];
                let new_val = val.wrapping_add(1);
                self.data[addr as usize] = new_val;
                self.set_nz(new_val);
                self.pc = self.pc.wrapping_add(1);
                println!("${:02X}", addr);
                Ok(())
            }

            
            Instruction::JMP_ABS => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_addr();
                self.pc = addr;
                println!("#${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_IMM => {
                self.pc = self.pc.wrapping_add(1);
                let val = self.read_byte_at(self.pc);
                println!("#${:02X}", val);
                self.a = val;
                self.pc = self.pc.wrapping_add(1);
                Ok(())
            }
            Instruction::STA_ABS => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_addr();
                println!("${:04X}", addr);
                self.data[addr as usize] = self.a;
                self.pc = self.pc.wrapping_add(2);
                Ok(())
            }
            Instruction::NOP => {
                self.pc = self.pc.wrapping_add(1);
                println!();
                Ok(())
            }
            _ => {
                println!("... but instruction not supported yet :(");
                Err(EErr::Break)
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

fn main() {
    let mut code: [u8; K32] = [0x00; K32];
    code[0xfffc - K32] = 0x00;
    code[0xfffd - K32] = 0x80;
    code[0] = 0xA9; code[1] = 0xFF;                     // lda #$ff
    code[2] = 0x8D; code[3] = 0x00; code[4] = 0x20;     // sta $2000
    code[5] = 0xA9; code[6] = 0x01;                     // lda $1
    code[7] = 0x85; code[8] = 0x00;                     // sta zp[0]
    // code[9] = 0x4C; code[10] = 0x00; code[11] = 0x80;   // jmp #$8000                     

    let mut e = Emulator::init(code);
    e.run();
}

// TODO:    maybe add WDC extensions as an option? (are conditional enum fields a thing)
//          actually probably just make it a flag to the compiler and check upon encounter