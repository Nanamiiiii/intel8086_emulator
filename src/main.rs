/* -------------------------------
    Intel 8086 Emulator in Rust 

    Author: Nanamiiiii
    Created: 2021-8-14
   ------------------------------- */

/* Type Aliases */
type Word = u16;
type Byte = u8;

/* enums */
pub enum Regname {
    AL, AH,
    BL, BH,
    CL, CH,
    DL, DH,
    AX, BX, CX, DX,
    SP, BP, SI, DI,
    IP,
    FlagsL, FlagsH,
    CS, DS, SS, ES,
}

/* Structs */
#[repr(C)]
pub struct Registers {
    /* General Registers */
    al: Byte, ah: Byte, // Accumulator
    bl: Byte, bh: Byte, // Base Register
    cl: Byte, ch: Byte, // Count Register
    dl: Byte, dh: Byte, // Data Register

    /* General Registers as 16bit
       Unsafe Raw Pointer         */
    ax: *const Word,
    bx: *const Word,
    cx: *const Word,
    dx: *const Word,

    /* Special Registers */
    sp: Word,   // Stack Pointer
    bp: Word,   // Base Pointer
    si: Word,   // Source Index
    di: Word,   // Destination Index

    ip: Word,   // Instruction Pointer

    /* Status Flags 

       |                 Higher                ||                 Lower                 |
       | 15 | 14 | 13 | 12 | 11 | 10 |  9 |  8 ||  7 |  6 |  5 |  4 |  3 |  2 |  1 |  0 |
       ----------------------------------------------------------------------------------
       |    |    |    |    | OF | DF | IF | TF || SF | ZF |    | AF |    | PF |    | CF |
    */

    flags_l: Byte,
    flags_h: Byte,  

    /* Segment Register */
    cs: Word,   // Code Segment
    ds: Word,   // Data Segment
    ss: Word,   // Stack Segment
    es: Word,   // Extra Segment
}

impl Default for Registers {
    fn default() -> Self {
        // Initialize
        let mut ret = Self {
            al: 0x00, ah: 0x00,
            bl: 0, bh: 0,
            cl: 0, ch: 0,
            dl: 0, dh: 0,
            ax: &0, bx: &0, cx: &0, dx: &0,
            sp: 0, bp: 0, si: 0, di: 0,
            ip: 0,
            flags_l: 0, flags_h: 0,
            cs: 0, ds: 0, ss: 0, es: 0,
        };

        // Setup 16bit registers
        ret.ax = &ret.al as *const Byte as usize as *const Word;
        ret.bx = &ret.bl as *const Byte as usize as *const Word;
        ret.cx = &ret.cl as *const Byte as usize as *const Word;
        ret.dx = &ret.dl as *const Byte as usize as *const Word;

        return ret;
    }
}

#[allow(dead_code)]
impl Registers {
    fn check_mem_layout(&self) {
        let base_ptr: *const Self = self;
        let base_addr: usize = base_ptr as usize;
        let ptr_al: *const Byte = &self.al;
        let ptr_ah: *const Byte = &self.ah;
        let ptr_bl: *const Byte = &self.bl;
        let ptr_bh: *const Byte = &self.bh;
        let ptr_cl: *const Byte = &self.cl;
        let ptr_ch: *const Byte = &self.ch;
        let ptr_dl: *const Byte = &self.dl;
        let ptr_dh: *const Byte = &self.dh;

        let ptr_fl: *const Byte = &self.flags_l;
        let ptr_fh: *const Byte = &self.flags_h;

        println!("Registers base: 0x{:x}", base_addr);
        assert_eq!(ptr_ah as usize, ptr_al as usize + 1);
        assert_eq!(ptr_bh as usize, ptr_bl as usize + 1);
        assert_eq!(ptr_ch as usize, ptr_cl as usize + 1);
        assert_eq!(ptr_dh as usize, ptr_dl as usize + 1);
        assert_eq!(ptr_fh as usize, ptr_fl as usize + 1);
        println!("Register layout check passed.\n");
    }

    fn reset(&mut self) {
        self.check_mem_layout();
        self.al = 0x00;
        self.ah = 0x00;
        self.bl = 0x00;
        self.bh = 0x00;
        self.cl = 0x00;
        self.ch = 0x00;
        self.dl = 0x00;
        self.dh = 0x00;
        self.sp = 0x0000;
        self.bp = 0x0000;
        self.si = 0x0000;
        self.di = 0x0000;
        self.ip = 0xFFF0;
        self.flags_h = 0x00;
        self.flags_l = 0x00;
        self.cs = 0xF000;
        self.ds = 0x0000;
        self.ss = 0x0000;
        self.es = 0x0000;

        // Setup 16bit registers
        self.ax = &self.al as *const Byte as usize as *const Word;
        self.bx = &self.bl as *const Byte as usize as *const Word;
        self.cx = &self.cl as *const Byte as usize as *const Word;
        self.dx = &self.dl as *const Byte as usize as *const Word;
    }

    pub fn get_byte(&self, name: Regname) -> Byte {
        match name {
            Regname::AL => self.al,
            Regname::AH => self.ah,
            Regname::BL => self.bl,
            Regname::BH => self.bh,
            Regname::CL => self.cl,
            Regname::CH => self.ch,
            Regname::DL => self.dl,
            Regname::DH => self.dh,
            Regname::FlagsL => self.flags_l,
            Regname::FlagsH => self.flags_h,
            _ => unimplemented!()
        }
    }

    pub fn set_byte(&mut self, name: Regname, value: Byte) {
        match name {
            Regname::AL => self.al = value,
            Regname::AH => self.ah = value,
            Regname::BL => self.bl = value,
            Regname::BH => self.bh = value,
            Regname::CL => self.cl = value,
            Regname::CH => self.ch = value,
            Regname::DL => self.dl = value,
            Regname::DH => self.dh = value,
            Regname::FlagsL => self.flags_l = value,
            Regname::FlagsH => self.flags_h = value,
            _ => unimplemented!(),
        }
    }

    pub fn get_word(&self, name: Regname) -> Word {
        match name {
            Regname::AX => unsafe { 
                match self.ax.as_ref() {
                    Some(&value) => value,
                    None => panic!()
                } 
            },
            Regname::BX => unsafe { 
                match self.bx.as_ref() {
                    Some(&value) => value,
                    None => panic!()
                }
            },
            Regname::CX => unsafe { 
                match self.cx.as_ref() {
                    Some(&value) => value,
                    None => panic!()
                }
            },
            Regname::DX => unsafe { 
                match self.dx.as_ref() {
                    Some(&value) => value,
                    None => panic!()
                }
            },
            Regname::SP => self.sp,
            Regname::BP => self.bp,
            Regname::SI => self.si,
            Regname::DI => self.di,
            Regname::IP => self.ip,
            Regname::CS => self.cs,
            Regname::DS => self.ds,
            Regname::SS => self.ss,
            Regname::ES => self.es,
            _ => unimplemented!(),
        }
    }

    pub fn set_word(&mut self, name: Regname, value: Word) {
        match name {
            Regname::AX => { 
                self.al = (value & 0x00FF) as u8;
                self.ah = (value >> 8) as u8;
            },
            Regname::BX => {
                self.bl = (value & 0x00FF) as u8;
                self.bh = (value >> 8) as u8;
            },
            Regname::CX => {
                self.cl = (value & 0x00FF) as u8;
                self.ch = (value >> 8) as u8;
            },
            Regname::DX => {
                self.dl = (value & 0x00FF) as u8;
                self.dh = (value >> 8) as u8;
            },
            Regname::SP => self.sp = value,
            Regname::BP => self.bp = value,
            Regname::SI => self.si = value,
            Regname::DI => self.di = value,
            Regname::IP => self.ip = value,
            Regname::CS => self.cs = value,
            Regname::DS => self.ds = value,
            Regname::SS => self.ss = value,
            Regname::ES => self.es = value,
            _ => unimplemented!(),
        }
    }

    pub fn info_registers(&self) {
        println!("[Information of Registers]");
        println!("AX: 0x{:04x}\tCX: 0x{:04x}\tDX: 0x{:04x}\tBX: 0x{:04x}", self.get_word(Regname::AX), self.get_word(Regname::CX), self.get_word(Regname::DX), self.get_word(Regname::BX));
        println!("SP: 0x{:04x}\tBP: 0x{:04x}\tSI: 0x{:04x}\tDI: 0x{:04x}", self.get_word(Regname::SP), self.get_word(Regname::BP), self.get_word(Regname::SI), self.get_word(Regname::DI));
        println!("IP: 0x{:04x}", self.get_word(Regname::IP));
        println!("FLAGS: 0x{0:02x}{1:02x} ({0:08b}{1:08b})", self.get_byte(Regname::FlagsH), self.get_byte(Regname::FlagsL));
        println!("CS: 0x{:04x}\tDS: 0x{:04x}\tSS: 0x{:04x}\tES: 0x{:04x}", self.get_word(Regname::CS), self.get_word(Regname::DS), self.get_word(Regname::SS), self.get_word(Regname::ES));
        println!();
    }
}

#[repr(C)]
pub struct Memory {
    /* 
        Now, this has a single 64KB segment
    */
    data: [Byte; Self::MAX_MEMSIZE]
}

#[allow(dead_code)]
impl Memory {
    const MAX_MEMSIZE: usize = 1024 * 64;

    fn reset(&mut self) {
        self.data = [0; Self::MAX_MEMSIZE];
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory { data: [0; Self::MAX_MEMSIZE] }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct Processor {
    registers: Registers,
    memory: Memory,
}

#[allow(dead_code, unused_variables)]
impl Processor {
    pub fn reset(&mut self) {
        self.registers.reset();
        self.memory.reset();
    }

    pub fn fetch_inst(&mut self, cycle: &mut u32) -> Byte {
        let data: Byte = self.memory.data[self.registers.ip as usize];
        self.registers.ip += 1;
        *cycle -= 1;
        return data;
    }

    pub fn execute(&mut self, cycle: &mut u32) {
        let mut inst_count = 0;
        let executed_cycle = *cycle;

        self.registers.info_registers();

        println!("Execute...");

        // Execution Loop
        while *cycle > 0 {
            let inst: Byte = self.fetch_inst(cycle);
            inst_count += 1;
            print!("{0: >3}: ", inst_count);
            print!("{:x} ", inst);
            match inst {
                InstSets::MOV_EB_GB => { // MOV r/m8 reg8
                    let arg: Byte = self.fetch_inst(cycle);
                    println!("{:x}", arg);

                    let mod_bits: Byte = (arg >> 6) & 0b011;    // higher 2 bits
                    let reg: Byte = (arg >> 3) & 0b0111;        // 3 bits in the middle
                    let rm: Byte = arg & 0b0111;                // lower 3 bits

                    let src = self.fetch_reg8(reg);
                    if mod_bits == 0b011 { // Register
                        let dst = self.fetch_reg8(rm);
                        self.registers.set_byte(dst, self.registers.get_byte(src));
                    } else { // Memory address
                        let dst = self.fetch_modrm(mod_bits, rm);
                        // TODO: store value to Memory[dst]
                    };
                },
                InstSets::MOV_EV_GV => { // MOV r/m16 reg16
                    let arg: Byte = self.fetch_inst(cycle);
                    println!("{:x}", arg);

                    let mod_bits: Byte = (arg >> 6) & 0b011;
                    let reg: Byte = (arg >> 3) & 0b0111;
                    let rm: Byte = arg & 0b0111;

                    let src = self.fetch_reg16(reg);
                    if mod_bits == 0b011 {
                        let dst = self.fetch_reg16(rm);
                        self.registers.set_word(dst, self.registers.get_word(src));
                    } else {
                        let dst = self.fetch_modrm(mod_bits, rm);
                        // TODO: store value to Memory[dst]
                    }
                },
                _ => unimplemented!()
            }
        } // End of execution loop

        println!("Finished execution.");
        println!("{} Cycle(s) executed.\n", executed_cycle);

        self.registers.info_registers();
    }

    fn fetch_modrm(&self, mod_bits: u8, rm: u8) -> usize {
        unimplemented!()
    }

    fn fetch_reg8(&self, reg: u8) -> Regname {
        match reg {
            0b000 => Regname::AL,
            0b001 => Regname::CL,
            0b010 => Regname::DL,
            0b011 => Regname::BL,
            0b100 => Regname::AH,
            0b101 => Regname::CH,
            0b110 => Regname::DH,
            0b111 => Regname::BH,
            _ => panic!()
        }
    }

    fn fetch_reg16(&self, reg: u8) -> Regname {
        match reg {
            0b000 => Regname::AX,
            0b001 => Regname::CX,
            0b010 => Regname::DX,
            0b011 => Regname::BX,
            0b100 => Regname::SP,
            0b101 => Regname::BP,
            0b110 => Regname::SI,
            0b111 => Regname::DI,
            _ => panic!(),
        }
    }
}

pub struct InstSets;

#[allow(dead_code)]
impl InstSets {
    const MOV_EB_GB: Byte = 0x88; // MOV r/m8 reg8
    const MOV_EV_GV: Byte = 0x89; // MOV r/m16 reg16
}

pub struct Modrm;

#[allow(dead_code)]
impl Modrm {
    
}

fn main() {
    let mut processor = Processor::default();
    processor.reset();
    let pc = processor.registers.ip as usize;
    processor.registers.set_word(Regname::AX, 0xAC24);
    processor.memory.data[pc] = 0x89;
    processor.memory.data[pc + 1] = 0xC1;

    processor.execute(&mut 2);
}
