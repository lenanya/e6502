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
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
enum Instruction {
    BRK = 0x00,
    LDA_IMM = 0xA9,
    LDY_IMM = 0xA0,
    STA_ABS = 0x8D,
    STA_ZPG = 0x85,
    NOP = 0xEA,
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

    fn exec_instruction(&mut self) -> Result<(), EErr> {
        let inst = self.read_instruction()?;
        print!("I: {:?} ", inst);

        match inst {
            Instruction::NOP => {
                self.instruction_pointer += 1;
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
                let zpg_addr = usize::from(0x100 | addr);
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
    code[0] = 0xA9; code[1] = 0xFF;                 // lda #$ff
    code[2] = 0x8D; code[3] = 0x00; code[4] = 0x20; // sta $2000
    code[5] = 0xA9; code[6] = 0x01;                 // lda $1
    code[7] = 0x85; code[8] = 0x00;                 // sta zp[0]
    code[9] = 0x00;                                 // brk

    let mut e = Emulator::init(code);
    e.run();
}
