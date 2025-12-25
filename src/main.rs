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
    instruction_pointer: u16,
    stack_pointer: u8,
    data: [u8; K64],
    psr: u8
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
enum PsrMask {
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
            instruction_pointer: rv,
            stack_pointer: 0xff,
            data: data.clone(),
            psr: PsrMask::Reserved as u8
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.data[usize::from(self.instruction_pointer)]
    }

    fn read_byte_at(&mut self, addr: u16) -> u8 {
        self.data[usize::from(addr)]
    }

    fn read_addr(&mut self) -> u16 {
        let low = u16::from(self.read_byte());
        let addr: u16 = self.instruction_pointer;
        let high = u16::from(self.read_byte_at(addr + 1));
        high << 8 | low
    }

    fn read_instruction(&mut self) -> Result<Instruction, u8> {
        let b: u8 = self.read_byte();
        Instruction::from_u8(b).ok_or(b)
    }

    fn set_psr_bit(&mut self, b: PsrMask, v: bool) {
        if v {
            self.psr |= b as u8;
        } else {
            self.psr &= !(b as u8);
        }
    }

    fn get_psr_bit(&mut self, b: PsrMask) -> bool {
        self.psr & b as u8 != 0
    }

    fn push_to_stack(&mut self, b: u8) {
        self.data[0x100 | self.stack_pointer as usize] = b;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn pop_from_stack(&mut self) -> u8 {
        let b = self.data[0x100 | (self.stack_pointer) as usize];
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        b
    }

    fn exec_instruction(&mut self) -> Result<(), EErr> {
        let inst = self.read_instruction()?;
        print!("I: {:?} ", inst);

        match inst {
            Instruction::BPL => {
                self.instruction_pointer += 1;
                if self.get_psr_bit(PsrMask::Negative) {
                    self.instruction_pointer += 1;
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.instruction_pointer = self.instruction_pointer.wrapping_add(idk as u16);
                println!("#${:04X}", self.instruction_pointer);
                Ok(())
            }
            Instruction::JSR => {
                self.instruction_pointer += 1;
                let addr = self.read_addr();
                self.instruction_pointer += 2;
                let ret_high: u8 = (self.instruction_pointer >> 4) as u8;
                let ret_low: u8 = (self.instruction_pointer & 0xff) as u8;
                self.push_to_stack(ret_high);
                self.push_to_stack(ret_low);
                self.instruction_pointer = addr;
                println!("#${:04X}", self.instruction_pointer);
                Ok(())
            }
            Instruction::BMI => {
                self.instruction_pointer += 1;
                if !self.get_psr_bit(PsrMask::Negative) {
                    self.instruction_pointer += 1;
                    return Ok(())
                }
                let offset = self.read_byte() as i8;
                let idk = offset as i16;
                self.instruction_pointer = self.instruction_pointer.wrapping_add(idk as u16);
                println!("#${:04X}", self.instruction_pointer);
                Ok(())
            }
            Instruction::BRK => {
                println!();
                return Err(EErr::Break);
            }
            Instruction::LDA_IMM => {
                self.instruction_pointer += 1;
                let val = self.read_byte_at(self.instruction_pointer);
                println!("#${:02X}", val);
                self.a = val;
                self.instruction_pointer += 1;
                Ok(())
            }
            Instruction::STA_ABS => {
                self.instruction_pointer += 1;
                let addr = self.read_addr();
                println!("${:04X}", addr);
                self.data[addr as usize] = self.a;
                self.instruction_pointer += 2;
                Ok(())
            }
            Instruction::STA_ZPG => {
                self.instruction_pointer += 1;
                let addr = u16::from(self.read_byte());
                let zpg_addr = usize::from(addr);
                println!("${:02X}", addr);
                self.data[zpg_addr] = self.a;
                self.instruction_pointer += 1;
                Ok(())
            }
            Instruction::LDY_IMM => {
                self.instruction_pointer += 1;
                let val = self.read_byte();
                println!("#${:02X}", val);
                self.y = val;
                self.instruction_pointer += 1;
                Ok(())
            }
            Instruction::NOP => {
                self.instruction_pointer += 1;
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
        'end: while self.instruction_pointer < 0xFFFF {
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
    let mut code: [u8; K32] = [0; K32];
    code[0xfffc - K32] = 0x00;
    code[0xfffd - K32] = 0x80;
    code[0] = 0xA9; code[1] = 0xFF;                     // lda #$ff
    code[2] = 0x8D; code[3] = 0x00; code[4] = 0x20;     // sta $2000
    code[5] = 0xA9; code[6] = 0x01;                     // lda $1
    code[7] = 0x85; code[8] = 0x00;                     // sta zp[0]
    code[9] = 0x4C; code[10] = 0x00; code[11] = 0x00;   // jmp #$0000                     

    let mut e = Emulator::init(code);
    e.run();
}
