/* -------------------------------
    Intel 8086 Emulator in Rust 

    Author: Nanamiiiii
    Created: 2021-8-14
   ------------------------------- */

/* Type Aliases */
type Word = u16;
type Byte = u8;

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

pub trait Regs16<T> {
    fn ax(&mut self, val: T) -> &u16;
    fn bx(&mut self, val: T) -> &u16;
    fn cx(&mut self, val: T) -> &u16;
    fn dx(&mut self, val: T) -> &u16;
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
    }

    fn reset(&mut self) {
        self.ip = 0x0000;
        self.flags_h = 0x00;
        self.flags_l = 0x00;
        self.cs = 0xFFFF;
        self.ds = 0x0000;
        self.ss = 0x0000;
        self.es = 0x0000;
    }
}

#[allow(unused_variables)]
impl Regs16<()> for Registers {
    fn ax(&mut self, val: ()) -> &u16 {
        unsafe {
            match self.ax.as_ref() {
                Some(ref_val) => ref_val,
                _ => panic!()
            }
        }
    }

    fn bx(&mut self, val: ()) -> &u16 {
        unsafe {
            match self.bx.as_ref() {
                Some(ref_val) => ref_val,
                _ => panic!()
            }
        }
    }

    fn cx(&mut self, val: ()) -> &u16 {
        unsafe {
            match self.cx.as_ref() {
                Some(ref_val) => ref_val,
                _ => panic!()
            }
        }
    }

    fn dx(&mut self, val: ()) -> &u16 {
        unsafe {
            match self.dx.as_ref() {
                Some(ref_val) => ref_val,
                _ => panic!()
            }
        }
    }
}

impl Regs16<u16> for Registers {
    fn ax(&mut self, val: u16) -> &u16 {
        self.al = (val & 0x00FF) as u8;
        self.ah = (val >> 8) as u8;
        unsafe {
            match self.ax.as_ref() {
                Some(ref_val) => {
                    assert_eq!(*ref_val, val);
                    ref_val
                },
                _ => panic!()
            }
        }
    }

    fn bx(&mut self, val: u16) -> &u16 {
        self.bl = (val & 0x00FF) as u8;
        self.bh = (val >> 8) as u8;
        unsafe {
            match self.bx.as_ref() {
                Some(ref_val) => {
                    assert_eq!(*ref_val, val);
                    ref_val
                },
                _ => panic!()
            }
        }
    }

    fn cx(&mut self, val: u16) -> &u16 {
        self.cl = (val & 0x00FF) as u8;
        self.ch = (val >> 8) as u8;
        unsafe {
            match self.cx.as_ref() {
                Some(ref_val) => {
                    assert_eq!(*ref_val, val);
                    ref_val
                },
                _ => panic!()
            }
        }
    }

    fn dx(&mut self, val: u16) -> &u16 {
        self.dl = (val & 0x00FF) as u8;
        self.dh = (val >> 8) as u8;
        unsafe {
            match self.dx.as_ref() {
                Some(ref_val) => {
                    assert_eq!(*ref_val, val);
                    ref_val
                },
                _ => panic!()
            }
        }
    }
}

fn main() {
    let mut registers = Registers::default();
    registers.reset();
    registers.check_mem_layout();
    
    registers.ax(0x7080);
    println!("ax: 0x{:x}", registers.ax(()));
    registers.bx(0x23F6);
    println!("bx: 0x{:x}", registers.bx(()));
    registers.cx(0xD3F6);
    println!("cx: 0x{:x}", registers.cx(()));
    registers.dx(0x2AFC);
    println!("dx: 0x{:x}", registers.dx(()));
}
