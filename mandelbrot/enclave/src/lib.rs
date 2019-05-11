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
    let size = 16000;
    let inv = 2.0 / size as f64;
    let mut xvals = vec![0.0; size];
    let mut yvals = vec![0.0; size];
    for i in 0..size {
        xvals[i] = i as f64 * inv - 1.5;
        yvals[i] = i as f64 * inv - 1.0;
    }
    let xloc = &xvals;
    let yloc = &yvals;

    assert!(size % THREADS == 0);// FIXME
    let handles: Vec<_> = (0..THREADS).map(|e| {
        let xloc = xloc.to_vec();
        let yloc = yloc.to_vec();
        let mut rows = vec![vec![0 as u8; size / 8]; size / THREADS];
            for y in 0..size / THREADS {
                for x in 0..size / 8 {
                    let mut cr = ZEROS;
                    let ci = [yloc[y + e * size / THREADS]; VLEN];
                    for i in 0..VLEN {
                        cr[i] = xloc[8 * x + i];
                    }
                    rows[y][x] = mbrot8(cr, ci);
                }
            }
            rows
    }).collect();

    println!("P4\n{} {}", size, size);
/*    let stdout_unlocked = std::io::stdout();
    let mut stdout = stdout_unlocked.lock();
    for row in handles.into_iter().flat_map(|h| h.into_iter()) {
        stdout.write_all(&row).unwrap();
    }
    stdout.flush().unwrap();*/
    sgx_status_t::SGX_SUCCESS
}

