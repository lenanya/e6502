#![allow(dead_code)]

use std::{env::args, fs, process};
use raylib;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// 8 Kibibytes
static K8: usize = 0x2000;
/// 16 Kibibytes
static K16: usize = 0x4000;
/// 32 Kibibytes
static K32: usize = 0x8000;
/// 64 Kibibytes
static K64: usize = 0x10000;

/// Start of the ROM in internal memory
static ROM_START: usize = K32;
/// Bottom of the stack
static STACK_BASE: usize = 0x100;
/// Reset Vector low byte
static RV_LOC_LOW: usize = 0xfffc;
/// Reset Vector high byte
static RV_LOC_HIGH: usize = 0xfffd;
/// GPU Flag bytes
static GPU_LOC: usize = 0xfff0;
/// Window Width low byte
static W_W_LOC: usize = 0xfff1;
/// Window Height low byte
static W_H_LOC: usize = 0xfff3;
/// Window title location low byte
static W_T_LOC: usize = 0xfff5;


/// Mask for Break and Reserved bit, as they get ignored when 
/// pulling SR off the stack
static B_R_MASK: u8 = 0b00110000; 
/// Mask to check if a number is negative by checking if the 7th bit is set
static NEG_MASK: u8 = 0b10000000;

/// Struct that contains all the methods and data of the Emulator
struct Emulator {
    /// Accumulator
    a: u8,       
    /// X register       
    x: u8,   
    /// Y register           
    y: u8,       
    /// Program Counter (Instruction Pointer)       
    pc: u16,            
    /// Stack Pointer
    sp: u8,     
    /// Internal RAM and ROM        
    bus: Bus,   
    /// State Register (flags)
    sr: u8,  
    /// Whether to print instructions and state or not
    debug: bool,
    /// Whether the GPU is enabled
    graphical: bool,

}

#[derive(Clone, Copy)]
/// Address Bus which handles all reads and writes, including IO
struct Bus {
    ram: [u8; K16],
    reserved1: [u8; K8],
    gpu: [u8; K8],
    rom: [u8; K32],
    gpu_enable: bool
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
/// An enum of all possible (legal) Opcodes
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
/// Masks to work with single bits of the SR
enum SRMask {
    // Functions as the 8th bit of operations 
    Carry       = 0b00000001, 
    // To tell if the last operations result was 0
    Zero        = 0b00000010, 
    // To tell if there was an interrupt
    Interrupt   = 0b00000100, 
    // Decimal mode?
    Decimal     = 0b00001000, 
    // To tell if the last instruction was a BRK
    Break       = 0b00010000, 
    // Does nothing
    Reserved    = 0b00100000, 
    // To tell if an overflow occured in the last instruction
    Overflow    = 0b01000000, 
    // To tell if the last operations result was negative
    Negative    = 0b10000000   
}

#[derive(Debug)]
/// Possible "Errors" during execution
enum EErr {
    // invalid opcode
    IllegalInstruction(u8), 
    // halt
    Break              
}

impl From<u8> for EErr {
    /// Needed for ?
    fn from(b: u8) -> Self {
        EErr::IllegalInstruction(b)
    }
}


// Methods for the bus
impl Bus {
    /// initialise the bus, cloning the ROM into the correct section
    pub fn init(rom: [u8; K32]) -> Bus {
        Bus {
            ram: [0; K16],
            reserved1: [0; K8],
            gpu: [0; K8],
            rom: rom.clone(),
            gpu_enable: false
        }
    }

    /**
    Read a byte from an address on the bus
    which will redirect to RAM, ROM or the reserved spaces IO
    */
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // RAM
            0x0000..=0x3FFF => {
                self.ram[addr as usize]
            }
            // reserved1
            0x4000..=0x5FFF => {
                0x00 // placeholder
            }
            // gpu
            0x6000..=0x7FFF => {
                // raylib function call arguments
                if matches!(addr, 0x6001..=0x60FF) {
                    return self.gpu[addr as usize - 0x6000]
                }
                // requested key
                if addr == 0x6100 {
                    return self.gpu[addr as usize - 0x6000]
                }
                // otherwise just 0 for now
                0x00
            }
            // ROM 
            0x8000..=0xFFFF => {
                self.rom[addr as usize - ROM_START]
            }
        }
    }

    /**
    Write a byte to an address on the bus
    which will redirect it to RAM or the reserved spaces (IO)
    */
    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            // RAM
            0x0000..=0x3FFF => {
                self.ram[addr as usize] = byte;
            }
            // reserved1
            0x4000..=0x5FFF => {
                if addr == 0x4000 {
                    print!("{}", byte as char);
                }
            }
            // gpu
            0x6000..=0x7FFF => {
                // only do this if gpu is enabled
                if self.gpu_enable {
                    // 0x6000 is like the "Enable Pin" of the "GPU"
                    if addr == 0x6000 {
                        match byte {
                            // BeginDrawing
                            0xBD => {
                                unsafe {
                                    raylib::ffi::BeginDrawing();
                                }
                            }
                            // EndDrawing
                            0xED => {
                                unsafe {
                                    raylib::ffi::EndDrawing();
                                }
                            }
                            // ClearBackground
                            0xCB => {
                                // get colour components
                                let r = self.read(0x6001); 
                                let g = self.read(0x6002);
                                let b = self.read(0x6003);
                                // make colour
                                let col = raylib::ffi::Color {r, g, b, a: 0xFF};
                                unsafe {
                                    // run command
                                    raylib::ffi::ClearBackground(col);
                                }
                            }
                            // DrawRectangle
                            0xD5 => {
                                // read rectangle position and size
                                let x = self.read(0x6001); 
                                let y = self.read(0x6002);
                                let w = self.read(0x6003);
                                let h = self.read(0x6004);
                                // get colour components
                                let r = self.read(0x6005); 
                                let g = self.read(0x6006);
                                let b = self.read(0x6007);
                                // make colour
                                let col = raylib::ffi::Color {r, g, b, a: 0xFF};
                                unsafe {
                                    // run command
                                    raylib::ffi::DrawRectangle(x as i32, y as i32, w as i32, h as i32, col);
                                }
                            }
                            // IsKeyDown
                            0xDE => {
                                // get the key the program wants to know
                                let key = self.read(0x6001);
                                unsafe {
                                    let is_down = raylib::ffi::IsKeyDown(key as i32);
                                    // set whether key is down or not
                                    self.gpu[0x100] = if is_down {0x01} else {0x0};
                                }
                            }
                            _ => {}
                        }
                        // clear arguments after a call
                        for i in 0x6001..=0x60FF {
                            self.write(i, 0x00);
                        }
                    }   

                }
                // 0x6001 - 0x60FF -> Arguments to GPU calls
                if matches!(addr, 0x6001..=0x60FF) {
                    self.gpu[addr as usize - 0x6000] = byte;
                }
            }
            // ROM 
            0x8000..=0xFFFF => {
                // you cant write to ROM, but also dont get an error
            }
        }
    }
}


/// Macro to replace all the instruction printing
/// which automatically checks if self.debug is true in Emulator
macro_rules! trace {
    ($self:ident, $($arg:tt)*) => {
        if $self.debug {
            println!($($arg)*);
        };
    }
}

// implementations of methods for Emulator
impl Emulator {
    /// Initialise the Emulator struct with the supplied ROM data, get the Reset Vector and 
    /// initialise registers
    fn init(code: [u8; K32], debug: bool) -> Emulator {
        let mut bus = Bus::init(code);

        // get the Reset Vector address from the ROM
        // Reset Vector is at 0xfffc - 0xfffd
        let rv_low = bus.read(RV_LOC_LOW as u16) as u16;
        let rv_high = bus.read(RV_LOC_HIGH as u16) as u16;
        let rv: u16 = rv_high << 8 | rv_low;

        let use_graphical = bus.read(GPU_LOC as u16) != 0;
        bus.gpu_enable = use_graphical;

        Emulator {
            a: 0,
            x: 0,
            y: 0,
            pc: rv,   // start execution at the address in the RV
            sp: 0xff, // stack starts at 0x1ff since it grows down
            bus: bus.clone(),
            sr: SRMask::Reserved as u8, // bit 5 is always set when pushing so set it
            debug: debug,
            graphical: use_graphical // whether to use raylib
        }
    }

    /// Print the current state of the CPU
    fn print_state(&mut self) {
        println!("-----------");
        println!("A:  0x{:02X}", self.a);
        println!("X:  0x{:02X}", self.x);
        println!("Y:  0x{:02X}", self.y);
        println!("SP: 0x{:02X}", self.sp);
        println!("SR: 0b{:08b}", self.sr);
        println!("-----------");
    }

    /// Read the byte at PC 
    fn read_byte(&mut self) -> u8 {
        self.bus.read(self.pc)
    }

    /// Read the byte at the supplied address
    fn read_byte_at(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    /// Write supplied byte to supplied address
    fn write_byte_at(&mut self, addr: u16, byte: u8) {
        self.bus.write(addr, byte);
    }

    /// Read an address at PC and PC+1, handle little endianness
    fn read_word(&mut self) -> u16 {
        let low = self.read_byte() as u16;
        let addr = self.pc;
        let high = self.read_byte_at(addr + 1) as u16;
        high << 8 | low // combine low and high into a u16 address
    }

    /// Read an address at pos and pos+1
    fn read_word_at(&mut self, pos: u16) -> u16 {
        let low = self.read_byte_at(pos) as u16;
        let high = self.read_byte_at(pos.wrapping_add(1)) as u16;
        high << 8 | low
    }

    /// Pop an address off the stack
    fn read_word_from_stack(&mut self) -> u16 {
        let addr_low = self.pop_from_stack() as u16; // low first, little endian
        let addr_high = self.pop_from_stack() as u16;
        addr_high << 8 | addr_low
    }

    /// Get the opcode at PC, if its illegal return the byte for logging, else the enum
    fn read_instruction(&mut self) -> Result<Instruction, u8> {
        let byte = self.read_byte();
        Instruction::from_u8(byte).ok_or(byte)
    }

    /// Set a flag in the SR
    fn set_sr_bit(&mut self, bit: SRMask, val: bool) {
        if val {
            self.sr |= bit as u8;
        } else {
            self.sr &= !(bit as u8);
        }
    }

    /// Get the value of a flag in the SR
    fn get_psr_bit(&mut self, bit: SRMask) -> bool {
        self.sr & bit as u8 != 0
    }

    /// Push a byte to the stack, stack wraps and grows down
    fn push_to_stack(&mut self, byte: u8) {
        self.bus.write(STACK_BASE as u16 | self.sp as u16, byte);
        self.sp = self.sp.wrapping_sub(1); // automatically handles the wrapping since sp is u8
    }

    /// Pull a byte from the stack
    fn pop_from_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1); // automatic wrapping again
        let byte = self.bus.read(STACK_BASE as u16 | (self.sp) as u16);
        byte
    }

    /// Get an address stored on the zeropage in Indirect, X mode
    fn x_ind(&mut self) -> u16 {
        // read zeropage address at PC, then add x
        let ind = self.bus.read(self.pc).wrapping_add(self.x); // zeropage wraps
        self.read_word_at(ind as u16) // read the address stored there
    }

    /// Get an address stored on the zeropage in Indirect, Y Indexed mode
    fn ind_y(&mut self) -> u16 {
        // read zeropage address at PC
        let ind = self.bus.read(self.pc);
        // get the address stored there and add Y to it to index
        self.read_word_at(ind as u16).wrapping_add(self.y as u16)
    }

    /// Set the Negative and Zero SR flags according to the byte supplied
    fn set_nz(&mut self, byte: u8) {
        self.set_sr_bit(SRMask::Zero, byte == 0);
        self.set_sr_bit(SRMask::Negative, (byte & NEG_MASK) != 0);
    }

    /**
    Add with carry using carry as an 8th bit for the operation
    overflow if a 9th bit would be used

    Updates Negative, Zero, Carry and Overflow flag
    */
    fn adc(&mut self, byte: u8) {
        // get the 8th bit
        let c: u16 = if self.get_psr_bit(SRMask::Carry) {1} else {0};
        // calculate the value
        let true_sum = self.a as u16 + byte as u16 + c;
        // set the carry bit if the operation is larger than a byte
        self.set_sr_bit(SRMask::Carry, true_sum > 0xff);
        // get the byte value
        let res: u8 = (true_sum & 0xff) as u8;
        // get signs 
        let a_sign = (self.a & NEG_MASK) != 0;
        let byte_sign = (byte & NEG_MASK) != 0;
        let res_sign = (res & NEG_MASK) != 0;
        // reset overflow bit
        self.set_sr_bit(SRMask::Overflow, false);
        if a_sign != byte_sign { // if not the same sign
            // result has to have the same sign as A, otherwise an overflow occured
            self.set_sr_bit(SRMask::Overflow, a_sign != res_sign);
        }
        self.a = res;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    /// Branch if cond is true, else just skip the offset
    fn branch(&mut self, cond: bool) {
        if !cond {
            self.pc = self.pc.wrapping_add(1);
            return;
        }
        let offset = self.read_byte() as i8; // get the offset
        self.pc = self.pc.wrapping_add(1); // go past the offset
        let idk = offset as i16; // cast to i16 incase its negative
        // adding unsigned == subtracting signed, since it wraps
        self.pc = self.pc.wrapping_add(idk as u16); 
    }

    /**
    Subtract with carry using carry as an 8th bit for the operation
    overflow if a 9th bit would be used

    Updates Negative, Zero, Carry and Overflow flag
    */
    fn sbc(&mut self, byte: u8) {
        // get the 8th bit
        let c: u16 = if self.get_psr_bit(SRMask::Carry) { 1 } else { 0 };
        // calculate value using 2s complement
        let true_sum = self.a as u16 + (!byte) as u16 + c;
        // set 8th bit if its used
        self.set_sr_bit(SRMask::Carry, true_sum > 0xff);
        // get byte
        let res = (true_sum & 0xff) as u8;
        // get their signs
        let a_sign = (self.a & NEG_MASK) != 0;
        let byte_sign = (byte & NEG_MASK) != 0;
        let res_sign = (res & NEG_MASK) != 0;
        // reset overflow bit
        self.set_sr_bit(SRMask::Overflow, false);
        if a_sign != byte_sign { // if not the same sign
            // result has to have the same sign as A, otherwise an overflow occured
            self.set_sr_bit(SRMask::Overflow, a_sign != res_sign);
        }
        self.a = res;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Compare the value in the Accumulator to byte

    This just does a subtraction where the result is discarded

    Updates Negative, Zero and Carry flag
    */
    fn cmp(&mut self, byte: u8) {
        let res = self.a.wrapping_sub(byte); // get result 
        self.set_nz(res); // set flags based on result
        // if result is larger than byte it wrapped
        self.set_sr_bit(SRMask::Carry, self.a >= byte);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Compare the value in the Y Register to byte

    This just does a subtraction where the result is discarded

    Updates Negative, Zero and Carry flag  
    */
    fn cpy(&mut self, byte: u8) {
        let res = self.y.wrapping_sub(byte);
        self.set_nz(res);
        self.set_sr_bit(SRMask::Carry, self.y >= byte);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Compare the value in the X Register to byte

    This just does a subtraction where the result is discarded

    Updates Negative, Zero and Carry flag
    */
    fn cpx(&mut self, byte: u8) {
        let res = self.x.wrapping_sub(byte);
        self.set_nz(res);
        self.set_sr_bit(SRMask::Carry, self.x >= byte);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Performs an OR between Accumulator and Byte and stores the result in A

    Updates Negative and Zero flag
    */
    fn ora(&mut self, byte: u8) {
        self.a = self.a | byte;
        self.set_nz(self.a); // set flags
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Performs an AND between Accumulator and Byte and stores the result in A

    Updates Negative and Zero flag
    */
    fn and(&mut self, byte: u8) {
        self.a = self.a & byte;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Performs an Exclusive OR between Accumulator and Byte and stores the result in A

    Updates Negative and Zero flag
    */
    fn eor(&mut self, byte: u8) {
        self.a = self.a ^ byte;
        self.set_nz(self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Load byte into the Accumulator

    Updates Negative and Zero flag 
    */
    fn lda(&mut self, byte: u8) {
        self.a = byte;
        self.set_nz(self.a); // update flags
        self.pc = self.pc.wrapping_add(1);
    }
    
    /**
    Load byte into the X Register

    Updates Negative and Zero flag 
    */
    fn ldx(&mut self, byte: u8) {
        self.x = byte;   
        self.set_nz(self.x);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Load byte into the Y Register

    Updates Negative and Zero flag 
    */
    fn ldy(&mut self, byte: u8) {
        self.y = byte;   
        self.set_nz(self.y);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Performs an AND on Accumulator and byte but discards the result and updates bit 6 and 7 
    of SR based on bit 6 and 7 of byte

    Updates Negative, Zero and Overflow flag
    */
    fn bit(&mut self, byte: u8) {
        self.set_sr_bit(SRMask::Zero, (self.a & byte) == 0);
        self.set_sr_bit(SRMask::Negative, (byte & SRMask::Negative as u8) != 0);
        self.set_sr_bit(SRMask::Overflow, (byte & SRMask::Overflow as u8) != 0);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Store byte in Y Register at address supplied
    */
    fn sty(&mut self, addr: u16) {
        self.write_byte_at(addr, self.y);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Store byte in Accumulator at address supplied
    */
    fn sta(&mut self, addr: u16) {
        self.write_byte_at(addr, self.a);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Store byte in X Register at address supplied
    */
    fn stx(&mut self, addr: u16) {
        self.write_byte_at(addr, self.x);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Rotate byte left by 1 bit, using Carry as 8th bit

    Updates Negative, Zero and Carry flag 
    */
    fn rol(&mut self, byte: u8) -> u8 {
        // get carry bit
        let c = if self.get_psr_bit(SRMask::Carry) {1} else {0};
        // shift byte, using u16 to keep 7th bit
        let byte_shifted = (byte as u16) << 1;
        // add the carry bit back on the end, and cut to byte
        let byte_rotated = (byte_shifted & 0xff) as u8 | c;
        // update carry flag 
        self.set_sr_bit(SRMask::Carry, byte_shifted > 0xff);
        self.set_nz(byte_rotated);
        byte_rotated
    }

    /**
    Rotate byte at address supplied left by 1 bit, using Carry as 8th bit

    Updates Negative, Zero and Carry flag 
    */
    fn rol_addr(&mut self, addr: u16) {
        let byte = self.read_byte_at(addr); // get the byte
        let byte_rotated = self.rol(byte);
        self.write_byte_at(addr, byte_rotated);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Rotate byte right by 1 bit, using Carry as 8th bit

    Updates Negative, Zero and Carry flag 
    */
    fn ror(&mut self, byte: u8) -> u8 {
        // get 8th bit
        let c = if self.get_psr_bit(SRMask::Carry) {1} else {0};
        self.set_sr_bit(SRMask::Carry, (byte & 0x01) != 0);
        let byte_shifted = byte >> 1;
        let byte_rotated = byte_shifted | c << 7;
        self.set_nz(byte_rotated);
        byte_rotated
    }

    /**
    Rotate byte at address supplied right by 1 bit, using Carry as 8th bit

    Updates Negative, Zero and Carry flag 
    */
    fn ror_addr(&mut self, addr: u16) {
        let byte = self.read_byte_at(addr); // get the byte
        let byte_rotated = self.ror(byte);
        self.write_byte_at(addr, byte_rotated);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Perform a Logical Shift Right on byte using Carry as 8th bit

    Updates Negative, Zero and Carry flag 
    */
    fn lsr(&mut self, byte: u8) -> u8 {
        // put 0th bit of byte into carry 
        self.set_sr_bit(SRMask::Carry, (byte & 0x01) != 0);
        let new_byte = byte >> 1;
        self.set_nz(new_byte);
        new_byte
    }

    /**
    Perform a Logical Shift Right on byte at address supplied using Carry as 8th bit

    Updates Negative, Zero and Carry flag 
    */
    fn lsr_addr(&mut self, addr: u16) {
        let byte = self.read_byte_at(addr);
        let new_byte = self.lsr(byte);
        self.write_byte_at(addr, new_byte);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Perform an Arithmetic Shift Left on byte using Carry as 8th bit

    Updates Negative, Zero and Carry flag 
    */
    fn asl(&mut self, byte: u8) -> u8 {
        // shift as u16 to not lose 7th bit
        let v_shifted = (byte as u16) << 1;
        // put shifted 7th bit into carry
        self.set_sr_bit(SRMask::Carry, v_shifted > 0xff);
        // cut down to byte
        let new_byte = (v_shifted & 0xff) as u8;
        self.set_nz(new_byte);
        new_byte
    }

    /**
    Perform an Arithmetic Shift Left on byte at address supplied using Carry as 8th bit

    Updates Negative, Zero and Carry flag 
    */
    fn asl_addr(&mut self, addr: u16) {
        let byte = self.read_byte_at(addr);
        let new_byte = self.asl(byte);
        self.write_byte_at(addr, new_byte);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Decrement byte at address supplied

    Updates Negative and Zero flag
    */
    fn dec(&mut self, addr: u16) {
        let byte = self.read_byte_at(addr);
        let new_byte = byte.wrapping_sub(1);
        self.write_byte_at(addr, new_byte);
        self.set_nz(new_byte);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Increment byte at address supplied

    Updates Negative and Zero flag
    */
    fn inc(&mut self, addr: u16) {
        let byte = self.read_byte_at(addr);
        let new_byte = byte.wrapping_add(1);
        self.write_byte_at(addr, new_byte);
        self.set_nz(new_byte);
        self.pc = self.pc.wrapping_add(1);
    }

    /**
    Execute the Instruction at PC, returns EErr on illegal opcode or BRK
    */
    fn exec_instruction(&mut self) -> Result<(), EErr> {
        if self.debug {
            self.print_state();
        }
        // get the opcode, propagate error if illegal
        let inst = self.read_instruction()?; 
        // print the current PC, and the instruction read
        if self.debug {
            print!("0x{:04X}: {:?} ", self.pc, inst);
        }
        // increment PC to the Operand or next instruction
        self.pc = self.pc.wrapping_add(1);

        match inst {
            Instruction::BRK => {
                // halt execution
                trace!(self, ""); // do nothing and return Break
                Err(EErr::Break)
            }
            Instruction::BPL => {
                // branch if not negative
                let cond = !self.get_psr_bit(SRMask::Negative);
                self.branch(cond);
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::JSR => {
                // jump to subroutine
                // read address of subroutine to jump to
                let addr = self.read_word();

                // keeping the logic as in the actual 6502
                // cause for some reason the other increment is done in RTS
                // as this is PC+2 which the original 6502 apparently did
                self.pc = self.pc.wrapping_add(1);
                // turn into bytes to push
                let ret_high: u8 = (self.pc >> 8) as u8;
                let ret_low: u8 = (self.pc & 0xff) as u8;
                // push return address
                self.push_to_stack(ret_high);
                self.push_to_stack(ret_low);
                // jump
                self.pc = addr;
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::BMI => {
                // branch if negative
                let cond = self.get_psr_bit(SRMask::Negative);
                self.branch(cond);
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::RTI => {
                // return from interrupt
                // get SR from stack
                let new_sr = self.pop_from_stack();
                self.sr = new_sr;
                // get return address from stack and jump
                self.pc = self.read_word_from_stack();
                trace!(self, "-> ${:04X}", self.pc);
                Ok(())
            }
            Instruction::BVC => {
                // branch if no overflow occured
                let cond = !self.get_psr_bit(SRMask::Overflow);
                self.branch(cond);
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::RTS => {
                // return from subroutine
                // get return address
                let new_pc = self.read_word_from_stack();

                // we increment pc by 1 so we dont execute the last byte of the address
                // that the JSR read
                self.pc = new_pc.wrapping_add(1);
                trace!(self, "-> ${:04X}", self.pc);
                Ok(())
            }
            Instruction::BVS => {
                // branch if overflow occured 
                let cond = self.get_psr_bit(SRMask::Overflow);
                self.branch(cond);
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::BCC => {
                // branch on carry == 0
                let cond = !self.get_psr_bit(SRMask::Carry);
                self.branch(cond);
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::LDY_IMM => {
                // load immediate value into Y register
                let byte = self.read_byte();
                self.ldy(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::BCS => {
                // branch on carry == 1
                let cond = self.get_psr_bit(SRMask::Carry);
                self.branch(cond);
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::CPY_IMM => {
                // compare immediate value to Y register
                let byte: u8 = self.read_byte();
                self.cpy(byte);
                trace!(self, "{:02X}", byte);
                Ok(())
            }
            Instruction::BNE => {
                // branch on zero == 1 (Values were not equal in comparison)
                let cond = !self.get_psr_bit(SRMask::Zero);
                self.branch(cond);
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::CPX_IMM => {
                // compare immediate value to X register
                let byte: u8 = self.read_byte();
                self.cpx(byte);
                trace!(self, "{:02X}", byte);
                Ok(())
            }
            Instruction::BEQ => {
                // branch on zero == 0 (Values were equal in comparison)
                let cond = self.get_psr_bit(SRMask::Zero);
                self.branch(cond);
                trace!(self, "#${:04X}", self.pc);
                Ok(())
            }
            Instruction::ORA_X_IND => {
                // perform an OR on Accumulator and the value at 
                // an address on the zeropage, X indexed
                let addr = self.x_ind();
                let byte = self.read_byte_at(addr);
                self.ora(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ORA_IND_Y => {
                // perform an OR on Accumulator and the value at 
                // an address on the zeropage, indirect, + Y
                let addr = self.ind_y();
                let byte = self.read_byte_at(addr);
                self.ora(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::AND_X_IND => {
                // perform an AND on Accumulator and the value at 
                // an address on the zeropage, X indexed
                let addr = self.x_ind();
                let byte = self.read_byte_at(addr);
                self.and(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::AND_IND_Y => {
                // perform an AND on Accumulator and the value at 
                // an address on the zeropage, indirect, + Y
                let addr = self.ind_y();
                let byte = self.read_byte_at(addr);
                self.and(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_X_IND => {
                // perform an exclusive OR on Accumulator and the value at 
                // an address on the zeropage, X indexed
                let addr = self.x_ind();
                let byte = self.read_byte_at(addr);
                self.eor(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_IND_Y => {
                // perform an exclusive OR on Accumulator and the value at 
                // an address on the zeropage, indirect, + Y
                let addr = self.ind_y();
                let byte = self.read_byte_at(addr);
                self.eor(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_X_IND => {
                // perform an add with carry on Accumulator and the value at 
                // an address on the zeropage, X indexed
                let addr = self.x_ind();
                let byte = self.read_byte_at(addr);
                self.adc(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_IND_Y => {
                // perform an add with carry on Accumulator and the value at 
                // an address on the zeropage, indirect, + Y
                let addr = self.ind_y();
                let byte = self.read_byte_at(addr);
                self.adc(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::STA_X_IND => {
                // store Accumulator at 
                // an address on the zeropage, X indexed
                let addr = self.x_ind();
                self.sta(addr);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::STA_IND_Y => {
                // store Accumulator at 
                // an address on the zeropage, indirect, + Y
                let addr = self.ind_y();
                self.sta(addr);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_X_IND => {
                // load value at an address on the zeropage, X indexed
                // into X
                let addr = self.x_ind();
                let byte = self.read_byte_at(addr);
                self.lda(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_IND_Y => {
                // load value at an address on the zeropage, indirect, + Y
                // into Accumulator
                let addr = self.ind_y();
                let byte = self.read_byte_at(addr);
                self.lda(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_X_IND => {
                // compare Accumulator with an address on the zeropage, X indexed
                let addr = self.x_ind();
                let byte: u8 = self.read_byte_at(addr);
                self.cmp(byte);
                trace!(self, "{:04X}", addr);
                Ok(())
            }
            Instruction::CMP_IND_Y => {
                // compare Accumulator with an address on the zeropage, indirect, + Y
                let addr = self.ind_y();
                let byte: u8 = self.read_byte_at(addr);
                self.cmp(byte);
                trace!(self, "{:04X}", addr);
                Ok(())
            }
            Instruction::SBC_X_IND => {
                // perform a subtraction with carry on Accumulator and the value at 
                // an address on the zeropage, X indexed
                let addr = self.x_ind();
                let byte = self.read_byte_at(addr);
                self.sbc(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::SBC_IND_Y => {
                // perform a subtraction with carry on Accumulator 
                // and the value at  an address on the zeropage, indirect, + Y
                let addr = self.ind_y();
                let byte = self.read_byte_at(addr);
                self.sbc(byte);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDX_IMM => {
                // load the immediate value into X
                let byte = self.read_byte();
                self.ldx(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::BIT_ZPG => {
                // do a bit test on A and a value on the zeropage
                // also copy bit 6 and 7 from the value to SR
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.bit(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::STY_ZPG => {
                // store Y into an address on the zeropage
                let addr = self.read_byte();
                self.sty(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::STY_ZPG_X => {
                // store Y into an address on the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                self.sty(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::LDY_ZPG => {
                // load the value from an address on the zeropage into Y
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.ldy(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::LDY_ZPG_X => {
                // load the value from an address on 
                // the zeropage, X indexed into Y
                let addr = self.read_byte().wrapping_add(self.x);
                let byte = self.read_byte_at(addr as u16);
                self.ldy(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::CPY_ZPG => {
                // compare the value in Y to the value 
                // at the address on the zeropage
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.cpy(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::CPX_ZPG => {
                // compare the value in X to the value 
                // at the address on the zeropage
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.cpx(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ORA_ZPG => {
                // perform an OR on Accumulator and a value at 
                // the address on the zeropage
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.ora(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ORA_ZPG_X => {
                // perform an OR on Accumulator and a value at 
                // the address on the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                let byte = self.read_byte_at(addr as u16);
                self.ora(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::AND_ZPG => {
                // perform an AND on Accumulator and a value at 
                // the address on the zeropage
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.and(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::AND_ZPG_X => {
                // perform an OR on Accumulator and a value at 
                // the address on the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                let byte = self.read_byte_at(addr as u16);
                self.and(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::EOR_ZPG => {
                // perform an exclusive OR on Accumulator and a value at 
                // the address on the zeropage
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.eor(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::EOR_ZPG_X => {
                // perform an exclusive OR on Accumulator and a value at 
                // the address on the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                let byte = self.read_byte_at(addr as u16);
                self.eor(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ADC_ZPG => {
                // perform an add with carry on Accumulator and a value at 
                // the address on the zeropage
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.adc(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ADC_ZPG_X => {
                // perform an add with carry on Accumulator and a value at 
                // the address on the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                let byte = self.read_byte_at(addr as u16);
                self.adc(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::STA_ZPG => {
                // store value from Accumulator at address on the zeropage
                let addr = self.read_byte();
                self.sta(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::STA_ZPG_X => {
                // store value from Accumulator at address on the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                self.sta(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::LDA_ZPG => {
                // load value at address on zeropage into Accumulator
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.lda(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::LDA_ZPG_X => {
                // load value at address on zeropage, X indexed into Accumulator
                let addr = self.read_byte().wrapping_add(self.x);
                let byte = self.read_byte_at(addr as u16);
                self.lda(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::CMP_ZPG => {
                // compare Accumulator and value at address on zeropage
                let addr = self.read_byte();
                let byte: u8 = self.read_byte_at(addr as u16);
                self.cmp(byte);
                trace!(self, "{:02X}", addr);
                Ok(())
            }
            Instruction::CMP_ZPG_X => {
                // compare Accumulator and value at address on zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                let byte: u8 = self.read_byte_at(addr as u16);
                self.cmp(byte);
                trace!(self, "{:02X}", addr);
                Ok(())
            }
            Instruction::SBC_ZPG => {
                // perform a subtraction with carry on Accumulator and a value at 
                // the address on the zeropage
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.sbc(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::SBC_ZPG_X => {
                // perform a subtraction with carry on Accumulator and a value at 
                // the address on the zeropage,X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                let byte = self.read_byte_at(addr as u16);
                self.sbc(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ASL_ZPG => {
                // perform an Arithmetic Shift Left on a value at an address on
                // the zeropage
                let addr = self.read_byte();
                self.asl_addr(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ASL_ZPG_X => {
                // perform an Arithmetic Shift Left on a value at an address on
                // the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                self.asl_addr(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ROL_ZPG => {
                // perform a Rotate Left on a value at an address on
                // the zeropage
                let addr = self.read_byte();
                self.rol_addr(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ROL_ZPG_X => {
                // perform a Rotate Left on a value at an address on
                // the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                self.rol_addr(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::LSR_ZPG => {
                // perform a Logical Shift Right on a value at an address on
                // the zeropage
                let addr = self.read_byte();
                self.lsr_addr(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::LSR_ZPG_X => {
                // perform a Logical Shift Right on a value at an address on
                // the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                self.lsr_addr(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ROR_ZPG => {
                // perform a Rotate Right on a value at an address on
                // the zeropage
                let addr = self.read_byte();
                self.ror_addr(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::ROR_ZPG_X => {
                // perform a Rotate Right on a value at an address on
                // the zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                self.ror_addr(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::STX_ZPG => {
                // store value in X at address on zeropage
                let addr = self.read_byte();
                self.stx(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::STX_ZPG_Y => {
                // store value in X at address on zeropage, Y indexed
                let addr = self.read_byte().wrapping_add(self.y);
                self.stx(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::LDX_ZPG => {
                // load value at address on zeropage into X
                let addr = self.read_byte();
                let byte = self.read_byte_at(addr as u16);
                self.ldx(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::LDX_ZPG_Y => {
                // load value at address on zeropage, Y indexed into X
                let addr = self.read_byte().wrapping_add(self.y);
                let byte = self.read_byte_at(addr as u16);
                self.ldx(byte);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::DEC_ZPG => {
                // decrement a value at address on zeropage
                let addr = self.read_byte();
                self.dec(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::DEC_ZPG_X => {
                // decrement a value at address on zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                self.dec(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::INC_ZPG => {
                // increment a value at address on zeropage
                let addr = self.read_byte();
                self.inc(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::INC_ZPG_X => {
                // increment a value at address on zeropage, X indexed
                let addr = self.read_byte().wrapping_add(self.x);
                self.inc(addr as u16);
                trace!(self, "${:02X}", addr);
                Ok(())
            }
            Instruction::PHP => {
                // push SR to stack with the Break bit set
                self.push_to_stack(self.sr | SRMask::Break as u8);
                trace!(self, );
                Ok(())
            }
            Instruction::CLC => {
                // clear carry bit
                self.sr = self.sr & !(SRMask::Carry as u8);
                trace!(self, );
                Ok(())
            }
            Instruction::PLP => {
                // pull SR from stack, ignoring Break and Reserved bit
                let stack_sr = self.pop_from_stack();
                self.sr = stack_sr & !B_R_MASK | self.sr & B_R_MASK;
                trace!(self, );
                Ok(())
            }
            Instruction::SEC => {
                // set carry bit
                self.sr = self.sr | SRMask::Carry as u8;
                trace!(self, );
                Ok(())
            }
            Instruction::PHA => {
                // push Accumulator to stack
                self.push_to_stack(self.a);
                trace!(self, );
                Ok(())
            }
            Instruction::CLI => {
                // clear interrupt bit
                self.sr = self.sr & !(SRMask::Interrupt as u8);
                trace!(self, );
                Ok(())
            }
            Instruction::PLA => {
                // pull Accumulator from stack
                self.a = self.pop_from_stack();
                self.set_nz(self.a);
                trace!(self, );
                Ok(())
            }
            Instruction::SEI => {
                // set interrupt bit
                self.sr = self.sr | SRMask::Interrupt as u8;
                trace!(self, );
                Ok(())
            }
            Instruction::DEY => {
                // decrement Y
                self.y = self.y.wrapping_sub(1);
                self.set_nz(self.y);
                trace!(self, );
                Ok(())
            }
            Instruction::TYA => {
                // transfer Y to Accumulator
                self.a = self.y;
                self.set_nz(self.a);
                trace!(self, );
                Ok(())
            }
            Instruction::TAY => {
                // transfer Acccumulator to Y
                self.y = self.a;
                self.set_nz(self.y);
                trace!(self, );
                Ok(())
            }
            Instruction::CLV => {
                // clear overflow bit
                self.sr = self.sr & !(SRMask::Overflow as u8);
                trace!(self, );
                Ok(())
            }
            Instruction::INY => {
                // increment Y
                self.y = self.y.wrapping_add(1);
                self.set_nz(self.y);
                trace!(self, );
                Ok(())
            }
            Instruction::CLD => {
                // clear Decimal bit
                self.sr = self.sr & !(SRMask::Decimal as u8);
                trace!(self, );
                Ok(())
            }
            Instruction::INX => {
                // increment X
                self.x = self.x.wrapping_add(1);
                self.set_nz(self.x);
                trace!(self, );
                Ok(())
            }
            Instruction::SED => {
                // set Decimal bit
                self.sr = self.sr | SRMask::Decimal as u8;
                trace!(self, );
                Ok(())
            }
            Instruction::ORA_IMM => {
                // perform an OR with Accumulator and immediate value
                let byte = self.read_byte();
                self.ora(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::ORA_ABS_Y => {
                // perform an OR with accumulator and value at absolute address + Y
                let addr = self.read_word().wrapping_add(self.y as u16);
                let byte = self.read_byte_at(addr);
                self.ora(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::AND_IMM => {
                // perform an AND with Accumulator and immediate value
                let byte = self.read_byte();
                self.and(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::AND_ABS_Y => {
                // perform an AND with accumulator and value at absolute address + Y
                let addr = self.read_word().wrapping_add(self.y as u16);
                let byte = self.read_byte_at(addr);
                self.and(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_IMM => {
                // perform an exclusive OR with Accumulator and immediate value
                let byte = self.read_byte();
                self.eor(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::EOR_ABS_Y => {
                // perform an exclusive OR with Accumulator and value at absolute address + Y
                let addr = self.read_word().wrapping_add(self.y as u16);
                let byte = self.read_byte_at(addr);
                self.eor(byte);
                self.pc = self.pc.wrapping_add(1); 
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_IMM => {
                // perform an add with carry on Accumulator and immediate value
                let byte = self.read_byte();
                self.adc(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::ADC_ABS_Y => {
                // perform an add with carry on Accumulator and value at absolute address + Y
                let addr = self.read_word().wrapping_add(self.y as u16);
                let byte = self.read_byte_at(addr);
                self.adc(byte);
                self.pc = self.pc.wrapping_add(1); 
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::STA_ABS_Y => {
                // store Accumulator at absolute address + Y
                let addr = self.read_word().wrapping_add(self.y as u16);
                self.sta(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_IMM => {
                // load immediate value into Accumulator
                let byte = self.read_byte();
                self.lda(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::LDA_ABS_Y => {
                // load value at absolute address + Y into Accumulator
                let addr = self.read_word().wrapping_add(self.y as u16);
                let byte = self.read_byte_at(addr);
                self.lda(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_IMM => {
                // compare Accumulator with immediate value
                let byte = self.read_byte();
                self.cmp(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::CMP_ABS_Y => {
                // compare Accumulator with value at absolute address + Y
                let addr = self.read_word().wrapping_add(self.y as u16);
                let byte = self.read_byte_at(addr);
                self.cmp(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "#${:04X}", byte);
                Ok(())
            }
            Instruction::SBC_IMM => {
                // perform a subtraction with carry on Accumulator
                // and immediate value
                let byte = self.read_byte();
                self.sbc(byte);
                trace!(self, "#${:02X}", byte);
                Ok(())
            }
            Instruction::SBC_ABS_Y => {
                // perform a subtraction with carry on Accumulator 
                // and value at absolute address + Y
                let addr = self.read_word().wrapping_add(self.y as u16);
                let byte = self.read_byte_at(addr);
                self.sbc(byte);
                self.pc = self.pc.wrapping_add(1); 
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ASL_A => {
                // perform an Arithmetic Shift Left on Accumulator
                self.a = self.asl(self.a);
                trace!(self, "A");
                Ok(())
            }
            Instruction::ROL_A => {
                // perform a Rotate Left on Accumulator
                self.a = self.rol(self.a);
                trace!(self, "A");
                Ok(())
            }
            Instruction::LSR_A => {
                // perform a Logical Shift Right on Accumulator
                self.a = self.lsr(self.a);
                trace!(self, "A");
                Ok(())
            }
            Instruction::ROR_A => {
                // perform a Rotate Right on Accumulator
                self.a = self.ror(self.a);
                trace!(self, "A");
                Ok(())
            }
            Instruction::TXA => {
                // transfer X to Accumulator
                self.a = self.x;
                self.set_nz(self.a);
                trace!(self, );
                Ok(())
            }
            Instruction::TXS => {
                // transer X to stack pointer
                self.sp = self.x;
                trace!(self, );
                Ok(())
            }
            Instruction::TAX => {
                // transfer Accumulator to X
                self.x = self.a;
                self.set_nz(self.x);
                trace!(self, );
                Ok(())
            }
            Instruction::TSX => {
                // transfer stack pointer to X
                self.x = self.sp;
                self.set_nz(self.x);
                trace!(self, );
                Ok(())
            }
            Instruction::DEX => {
                // decrement X
                self.x = self.x.wrapping_sub(1);
                self.set_nz(self.x);
                trace!(self, );
                Ok(())
            }
            Instruction::NOP => {
                // literally do nothing, since the PC is incremented
                // before the match starts
                trace!(self, );
                Ok(())
            }
            Instruction::BIT_ABS => {
                // perform a bit test on the value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.bit(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::JMP_ABS => {
                // jump to absolute address
                let addr = self.read_word();
                self.pc = addr;
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::JMP_IND => {
                // jump to current address + offset
                let addr = self.read_word();
                let dest = self.read_word_at(addr);
                self.pc = dest;
                trace!(self, "${:04X}", dest);
                Ok(())
            }
            Instruction::STY_ABS => {
                // store Y at absolute address
                let addr = self.read_word();
                self.sty(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDY_ABS => {
                // load value at absolute address into Y
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.ldy(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDY_ABS_X => {
                // load value at absolute address, X indexed into Y
                let addr = self.read_word().wrapping_add(self.x as u16);
                let byte = self.read_byte_at(addr);
                self.ldy(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::CPY_ABS => {
                // compare Y to value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.cpy(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::CPX_ABS => {
                // compare X to value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.cpx(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ORA_ABS => {
                // perform OR on Accumulator and value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.ora(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ORA_ABS_X => {
                // perform OR on Accumulator and value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                let byte = self.read_byte_at(addr);
                self.ora(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::AND_ABS => {
                // perform AND on Accumulator and value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.and(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::AND_ABS_X => {
                // perform AND on Accumulator and value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                let byte = self.read_byte_at(addr);
                self.and(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_ABS => {
                // perform EOR on Accumulator and value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.eor(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::EOR_ABS_X => {
                // perform EOR on Accumulator and value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                let byte = self.read_byte_at(addr);
                self.eor(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_ABS => {
                // perform add with carry on Accumulator 
                // and value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.adc(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ADC_ABS_X => {
                // perform add with carry on Accumulator 
                // and value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                let byte = self.read_byte_at(addr);
                self.adc(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::STA_ABS => {
                // store Accumulator at absolute address
                let addr = self.read_word();
                self.sta(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::STA_ABS_X => {
                // store Accumulator at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                self.sta(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_ABS => {
                // load value from absolute address into Accumulator
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.lda(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDA_ABS_X => {
                // load value from absolute address, X indexed into Accumulator
                let addr = self.read_word().wrapping_add(self.x as u16);
                let byte = self.read_byte_at(addr);
                self.lda(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_ABS => {
                // compare Accumulator to value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.cmp(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::CMP_ABS_X => {
                // compare Accumulator to value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                let byte = self.read_byte_at(addr);
                self.cmp(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::SBC_ABS => {
                // perform subtraction with carry on A and value at absolute address
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.sbc(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::SBC_ABS_X => {
                // perform subtraction with carry on A 
                // and value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                let byte = self.read_byte_at(addr);
                self.sbc(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ASL_ABS => {
                // perform an Arithmetic Shift Left 
                // on value at absolute address
                let addr = self.read_word();
                self.asl_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ASL_ABS_X => {
                // perform an Arithmetic Shift Left 
                // on value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                self.asl_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ROL_ABS => {
                // perform a Rotate Left
                // on value at absolute address
                let addr = self.read_word();
                self.rol_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ROL_ABS_X => {
                // perform a Rotate Left
                // on value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                self.rol_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LSR_ABS => {
                // perform a Logical Shift Right 
                // on value at absolute address
                let addr = self.read_word();
                self.lsr_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LSR_ABS_X => {
                // perform a Logical Shift Right 
                // on value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                self.lsr_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ROR_ABS => {
                // perform a Rotate Right
                // on value at absolute address
                let addr = self.read_word();
                self.ror_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::ROR_ABS_X => {
                // perform a Rotate Right
                // on value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                self.ror_addr(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::STX_ABS => {
                // store X at absolute address
                let addr = self.read_word();
                self.stx(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDX_ABS => {
                // load value at absolute address into X
                let addr = self.read_word();
                let byte = self.read_byte_at(addr);
                self.ldx(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::LDX_ABS_Y => {
                // store X at absolute address, Y indexed
                let addr = self.read_word().wrapping_add(self.y as u16);
                let byte = self.read_byte_at(addr);
                self.ldx(byte);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::DEC_ABS => {
                // decrement value at absolute address
                let addr = self.read_word();
                self.dec(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::DEC_ABS_X => {
                // decrement value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                self.dec(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::INC_ABS => {
                // increment value at absolute address
                let addr = self.read_word();
                self.inc(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
            Instruction::INC_ABS_X => {
                // increment value at absolute address, X indexed
                let addr = self.read_word().wrapping_add(self.x as u16);
                self.inc(addr);
                self.pc = self.pc.wrapping_add(1);
                trace!(self, "${:04X}", addr);
                Ok(())
            }
        }
    }
    
    /// Run the currently loaded ROM until a BRK or error occurs
    fn run(&mut self) {
        if self.graphical {
            // get the window width from the ROM
            let width = self.read_word_at(W_W_LOC as u16);
            // get the window height from the ROM
            let height = self.read_word_at(W_H_LOC as u16);
            // get the position of the title in the ROM from the ROM
            let title_ptr = self.read_word_at(W_T_LOC as u16);
            // vec to store raw title bytes
            let mut title_vec: Vec<i8> = vec![];
            // start at title_ptr
            let mut i = title_ptr;
            // read the raw bytes into the vec until null terminator
            while self.bus.read(i) != 0x00 {
                title_vec.push(self.bus.read(i) as i8);
                i += 1;
            }
            title_vec.push(0); // push null for terminator for raylib

            unsafe {
                raylib::ffi::InitWindow(width as i32, height as i32, title_vec.as_ptr());
                raylib::ffi::SetTargetFPS(30);
            }
            

        }


        'end: loop {
            if self.graphical {
                unsafe {
                    if raylib::ffi::WindowShouldClose() {
                        break 'end;
                    }
                }
            }
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

        if self.graphical {
            unsafe {
                raylib::ffi::CloseWindow();
            }
        }
    }
}

/// Load ROM from file into byte array
fn load_rom(path: &str, rom: &mut [u8; K32]) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|e| format!("IO Error: {}", e))?;

    for (i, byte) in bytes.iter().enumerate() {
        if i < K32 { // less than 32KiB
            rom[i as usize] = *byte;
        } else {
            return Err("ROM is larger than 32KiB".to_string())
        }
    }
    Ok(())
    
}

/// Entry point (duh)
fn main() {
    let mut a = args().into_iter();
    let _program_name = a.next(); // ignored
    let rom_name = a.next().expect("You need to supply a 32KiB ROM");
    let mut code: [u8; K32] = [0x00; K32];
    
    // load the ROM 
    load_rom(&rom_name, &mut code).unwrap_or_else(|e| {
        eprintln!("[ERROR]: {}", e);
        process::exit(1);
    });
    
    // init and run emulator
    let mut e = Emulator::init(code, false);
    e.run();
}

// TODO:    maybe add WDC extensions as an option? (are conditional enum fields a thing)
//          actually probably just make it a flag to the compiler and check upon encounter
// TODO:    add an option to replicate the JMP_IND page boundary bug
// TODO:    use bitflags crate
// TODO:    make debug a flag 