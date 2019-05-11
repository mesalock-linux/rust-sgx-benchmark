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



const LINE_LENGTH: usize = 60;
const BLOCK_SIZE: usize = LINE_LENGTH * 1024;
const IM: u32 = 139968;

/// Pseudo-random number generator
struct Rng(u32);

impl Rng {
    fn new() -> Self { Rng(42) }

    fn gen(&mut self, probabilities: &[(u32, u8)], block: &mut [u8]) {
        for i in block.iter_mut() {
            self.0 = (self.0 * 3877 + 29573) % IM;
            *i = probabilities.iter().find(|&&(p, _)| p >= self.0).unwrap().1;
        }
    }
}

/// From a probability distribution, generate a cumulative probability distribution.
fn cumulative_probabilities(ps: &[(char, f32)]) -> Vec<(u32, u8)> {
    ps.iter().scan(0., |sum, &(c, p)| {
        *sum += p;
        Some(((*sum * IM as f32).floor() as u32, c as u8))
    }).collect()
}

/// Output FASTA data from the provided generator function.
fn make_fasta<F>(n: usize, mut f: F) -> io::Result<()>
    where F: FnMut(&mut [u8])
{
    let mut block = vec![0; BLOCK_SIZE];

    // Write whole blocks.
    let num_blocks = n / BLOCK_SIZE;
    for _ in 0..num_blocks {
        f(&mut block);
//        write(&block, w)?;
    }

    // Write trailing block.
    let trailing_len = n % BLOCK_SIZE;
    if trailing_len > 0 {
        f(&mut block[..trailing_len]);
//        write(&block[..trailing_len], w)?;
    }
    Ok(())
}

/// Print FASTA data in 60-column lines.
#[inline(always)]
fn write<W: Write>(block: &[u8], output: &mut W) -> io::Result<()> {
    for chunk in block.chunks(LINE_LENGTH) {
        output.write_all(chunk)?;
        output.write_all(b"\n")?;
    }
    Ok(())
}

fn run(n: usize) -> io::Result<()> {
//    let mut out = BufWriter::new(io::stdout());

    // Generate a DNA sequence by copying from the given sequence.

    const ALU: &'static [u8] =
        b"GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGA\
          TCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACT\
          AAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAG\
          GCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCG\
          CCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA";
    let mut it = ALU.iter().cloned().cycle();

//    writeln!(out, ">ONE Homo sapiens alu")?;
    make_fasta(n * 2, |block| for i in block {
        *i = it.next().unwrap()
    })?;

    // Generate DNA sequences by weighted random selection from two alphabets.

    let p0 = cumulative_probabilities(
        &[('a', 0.27), ('c', 0.12), ('g', 0.12), ('t', 0.27), ('B', 0.02),
          ('D', 0.02), ('H', 0.02), ('K', 0.02), ('M', 0.02), ('N', 0.02),
          ('R', 0.02), ('S', 0.02), ('V', 0.02), ('W', 0.02), ('Y', 0.02)]);

    let p1 = cumulative_probabilities(
        &[('a', 0.3029549426680), ('c', 0.1979883004921),
          ('g', 0.1975473066391), ('t', 0.3015094502008)]);

    let mut rng = Rng::new();

//    writeln!(out, ">TWO IUB ambiguity codes")?;
    make_fasta(n * 3, |block| rng.gen(&p0, block))?;

//    writeln!(out, ">THREE Homo sapiens frequency")?;
    make_fasta(n * 5, |block| rng.gen(&p1, block))?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn fasta() -> sgx_status_t{
    let n = 25000000;
    for _ in 0..20{
    run(n).unwrap();}
    sgx_status_t::SGX_SUCCESS
}
