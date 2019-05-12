#![allow(non_snake_case)]
pub fn main() {
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    let mut bit_num: i32 = 0;

    let mut byte_acc: u8 = 0;

    let mut i: i32 = 0;
    let mut iter: i32 = 50;

    let mut x:f64 = 0.0;
    let mut y:f64 = 0.0;
    let mut limit:f64 = 2.0;
    let mut Zr:f64 = 0.0;
    let mut Zi:f64 = 0.0;
    let mut Cr:f64 = 0.0;
    let mut Ci:f64 = 0.0;
    let mut Tr:f64 = 0.0;
    let mut Ti:f64 = 0.0;

    w = 160000;
    h = 160000;

    println!("P4\n{} {}", w, h);

    for y in 0..h {
        for x in 0..w {
            Zr = 0.0;
            Zi = 0.0;
            Tr = 0.0;
            Ti = 0.0;
            Cr = (2.0 * x as f64 / w as f64 - 1.5);
            Ci = (2.0 * y as f64 / h as f64 - 1.0);

            i = 0;
            while i < iter && (Tr + Ti <= limit * limit) {
                Zi = 2.0 * Zr * Zi + Ci;
                Zr = Tr - Ti + Cr;
                Tr = Zr * Zr;
                Ti = Zi * Zi;
                i = i + 1;
            }

            byte_acc = byte_acc << 1;
            if Tr + Ti <= limit * limit {
                byte_acc |= 0x01u8;
            }

            bit_num = bit_num + 1;

            if bit_num == 8 {
                byte_acc = byte_acc << (8 - w % 8);
                byte_acc = 0;
                bit_num = 0;
            }
        }
    }
    println!("bit_num = {}", bit_num);
}
