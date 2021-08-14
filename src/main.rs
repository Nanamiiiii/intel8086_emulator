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

#[allow(dead_code)]
impl Registers {
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

fn main() {
    let mut registers = Registers {
        al: 0, ah: 0,
        bl: 0, bh: 0,
        cl: 0, ch: 0,
        dl: 0, dh: 0,
        sp: 0,
        bp: 0,
        si: 0,
        di: 0,
        ip: 0,
        flags_l: 0, flags_h: 0,
        cs: 0,
        ds: 0,
        ss: 0,
        es: 0,
    };
    registers.reset();

    println!("Hello, world!");
}
