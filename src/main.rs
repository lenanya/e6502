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
    CMD_IND_Y   = 0xD1,
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
    STX_ZPG_X   = 0x96,
    LDX_ZPG     = 0xA6,
    LDX_ZPG_X   = 0xB6,
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

    fn exec_instruction(&mut self) -> Result<(), EErr> {
        let inst = self.read_instruction()?;
        print!("0x{:04X}: {:?} ", self.pc, inst);

        match inst {
            Instruction::BPL => {
                self.pc = self.pc.wrapping_add(1);
                if self.get_psr_bit(SRMask::Negative) {
                    self.pc = self.pc.wrapping_add(1);
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.pc = self.pc.wrapping_add(idk as u16);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::JSR => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_addr();

                // keeping the logic as in the actual 6502
                // cause for some reason the other increment is done in RTS
                // as this is PC+2 which the original 6502 apparently did
                self.pc = self.pc.wrapping_add(1);
                let ret_high: u8 = (self.pc >> 4) as u8;
                let ret_low: u8 = (self.pc & 0xff) as u8;
                self.push_to_stack(ret_high);
                self.push_to_stack(ret_low);
                self.pc = addr;
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::BMI => {
                self.pc = self.pc.wrapping_add(1);
                if !self.get_psr_bit(SRMask::Negative) {
                    self.pc = self.pc.wrapping_add(1);
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.pc = self.pc.wrapping_add(idk as u16);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::RTI => {
                self.pc = self.pc.wrapping_add(1);
                let new_sr = self.pop_from_stack();
                self.sr = new_sr;
                let new_pc_low = self.pop_from_stack() as u16;
                let new_pc_high = self.pop_from_stack() as u16;
                self.pc = new_pc_high << 4 | new_pc_low;
                println!();
                Ok(())
            }
            Instruction::BVC => {
                self.pc = self.pc.wrapping_add(1);
                if self.get_psr_bit(SRMask::Overflow) {
                    self.pc = self.pc.wrapping_add(1);
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.pc = self.pc.wrapping_add(idk as u16);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::RTS => {
                let new_pc_low = self.pop_from_stack() as u16;
                let new_pc_high = self.pop_from_stack() as u16;
                let new_pc = new_pc_high << 4 | new_pc_low;

                // we increment pc by 1 so we dont execute the last byte of the address
                // that the JSR read
                self.pc = new_pc.wrapping_add(1);
                Ok(())
            }
            Instruction::BVS => {
                self.pc = self.pc.wrapping_add(1);
                if !self.get_psr_bit(SRMask::Overflow) {
                    self.pc = self.pc.wrapping_add(1);
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.pc = self.pc.wrapping_add(idk as u16);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::BCC => {
                self.pc = self.pc.wrapping_add(1);
                if self.get_psr_bit(SRMask::Carry) {
                    self.pc = self.pc.wrapping_add(1);
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.pc = self.pc.wrapping_add(idk as u16);
                println!("#${:04X}", self.pc);
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
                if !self.get_psr_bit(SRMask::Carry) {
                    self.pc = self.pc.wrapping_add(1);
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.pc = self.pc.wrapping_add(idk as u16);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::CPY_IMM => {
                self.pc = self.pc.wrapping_add(1);
                let val: u8 = self.read_byte();
                let res = self.y.wrapping_sub(val);
                self.set_sr_bit(SRMask::Zero, res == 0);
                self.set_sr_bit(SRMask::Negative, res & 0x80 == 1);
                self.set_sr_bit(SRMask::Carry, res < self.y);
                println!("{:02X}", val);
                Ok(())
            }
            Instruction::BNE => {
                self.pc = self.pc.wrapping_add(1);
                if self.get_psr_bit(SRMask::Zero) {
                    self.pc = self.pc.wrapping_add(1);
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.pc = self.pc.wrapping_add(idk as u16);
                println!("#${:04X}", self.pc);
                Ok(())
            }
            Instruction::CPX_IMM => {
                self.pc = self.pc.wrapping_add(1);
                let val: u8 = self.read_byte();
                let res = self.x.wrapping_sub(val);
                self.set_sr_bit(SRMask::Zero, res == 0);
                self.set_sr_bit(SRMask::Negative, res & 0x80 == 1);
                self.set_sr_bit(SRMask::Carry, res <= self.x);
                println!("{:02X}", val);
                Ok(())
            }
            Instruction::BEQ => {
                self.pc = self.pc.wrapping_add(1);
                if !self.get_psr_bit(SRMask::Zero) {
                    self.pc = self.pc.wrapping_add(1);
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.pc = self.pc.wrapping_add(idk as u16);
                println!("#${:04X}", self.pc);
                Ok(())
            }

            Instruction::JMP_ABS => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.read_addr();
                self.pc = addr;
                println!("#${:04X}", addr);
                Ok(())
            }
            Instruction::BRK => {
                println!();
                return Err(EErr::Break);
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
            Instruction::STA_ZPG => {
                self.pc = self.pc.wrapping_add(1);
                let addr = u16::from(self.read_byte());
                let zpg_addr = usize::from(addr);
                println!("${:02X}", addr);
                self.data[zpg_addr] = self.a;
                self.pc = self.pc.wrapping_add(1);
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