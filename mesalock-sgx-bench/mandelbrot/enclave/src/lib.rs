// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::slice;
use std::io::{self, Write};



const THREADS: usize = 20;
const MAX_ITER: usize = 50;
const VLEN: usize = 8;
const ZEROS: Vecf64 = [0.0; VLEN];

pub type Vecf64 = [f64; VLEN];

fn mul2 (x: Vecf64, y: Vecf64) -> Vecf64 {
    let mut res = ZEROS;
    for i in 0..VLEN { res[i] = x[i] * y[i]; }
    res
}
fn add2 (x: Vecf64, y: Vecf64) -> Vecf64 {
    let mut res = ZEROS;
    for i in 0..VLEN { res[i] = x[i] + y[i]; }
    res
}
fn sub2 (x: Vecf64, y: Vecf64) -> Vecf64 {
    let mut res = ZEROS;
    for i in 0..VLEN { res[i] = x[i] - y[i]; }
    res
}

pub fn mbrot8(cr: Vecf64, ci: Vecf64) -> u8 {
    let mut zr = cr;
    let mut zi = ci;
    let mut esc_bits = 0;
    for _ in 0..MAX_ITER {
        // Find Re(z)^2 and Im(z)^2
        let rr  = mul2(zr,zr);
        let ii  = mul2(zi,zi);
        // Check if we escape
        // May as well store this info in
        // same byte as output
        let mag = add2(rr, ii);
        for i in 0..VLEN {
            if mag[i] > 4.0 { esc_bits |= 128 >> i; }
        }
        // If no more work, break early
        if esc_bits == 0xff { break; }
        // Find Im(z^2)
        let ir = mul2(zr, zi);
        // Set Re(z^2)
        zr = sub2(rr, ii);
        // Set Im(z^2)
        zi = add2(ir, ir);
        // Add c
        zr = add2(zr, cr);
        zi = add2(zi, ci);
    }
    !esc_bits
}

#[no_mangle]
pub extern "C" fn uniform() -> sgx_status_t{
    main();
    sgx_status_t::SGX_SUCCESS
}

fn main() {
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

    println!("big_num = {}", bit_num);
}
