// Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.
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

const FLAGS_NON_SECURITY_BITS: uint64_t = (0x00FF_FFFF_FFFF_FFC0 | SGX_FLAGS_MODE64BIT | SGX_FLAGS_PROVISION_KEY| SGX_FLAGS_EINITTOKEN_KEY);
const TSEAL_DEFAULT_FLAGSMASK: uint64_t = (!FLAGS_NON_SECURITY_BITS);

const TEST_COUNT: usize = 100_0000;

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate sgx_tse;
extern crate sgx_rand;
extern crate sgx_tseal;

use std::prelude::v1::*;
use std::time::*;
use std::untrusted::time::InstantEx;
use sgx_types::*;
use sgx_rand::Rng;
use sgx_tseal::SgxSealedData;

#[no_mangle]
pub extern "C" fn sgx_api_bench() -> sgx_status_t {
    bench_sgx_get_key();    
    bench_sgx_create_report();
    bench_sgx_verify_report();
    bench_sgx_self_report();
    bench_sgx_seal_data();
    bench_sgx_unseal_data();
    bench_sgx_fopen();
    bench_sgx_fopen_autokey();
    bench_sgx_export_autokey();
    bench_sgx_import_autokey();
    sgx_status_t::SGX_SUCCESS
}

fn bench_sgx_get_key() {
    let report = sgx_tse::rsgx_self_report();
    let key_policy = SGX_KEYPOLICY_MRSIGNER;
    let isv_svn = report.body.isv_svn;
    let reserved1 = 0u16;
    let cpu_svn = report.body.cpu_svn;
    let attribute_mask = sgx_attributes_t{flags: TSEAL_DEFAULT_FLAGSMASK, xfrm: 0};
    let key_id = sgx_key_id_t::default();
    let misc_mask = 0;
    let config_svn = report.body.config_svn;
    let reserved2 = [0u8; SGX_KEY_REQUEST_RESERVED2_BYTES];

    let kr: sgx_key_request_t = sgx_key_request_t {
        key_name: SGX_KEYSELECT_SEAL,
        key_policy,
        isv_svn,
        reserved1,
        cpu_svn,
        attribute_mask,
        key_id,
        misc_mask,
        config_svn,
        reserved2};

    let start = Instant::now();
    for _ in 0..TEST_COUNT {
        let _ = sgx_tse::rsgx_get_key(&kr);
    }
    println!("sgx_get_key, {:?}", start.elapsed());
}

fn bench_sgx_create_report() {
    let target = sgx_target_info_t::default();
    let report_data = sgx_report_data_t::default();
    let start = Instant::now();
    for _ in 0..TEST_COUNT {
        let _ = sgx_tse::rsgx_create_report(&target, &report_data);
    }
    println!("sgx_create_report, {:?}", start.elapsed());
}

fn bench_sgx_verify_report() {
    let target = sgx_target_info_t::default();
    let report_data = sgx_report_data_t::default();
    let start = Instant::now();
    let report = sgx_tse::rsgx_create_report(&target, &report_data).unwrap();
    for _ in 0..TEST_COUNT {
        let _ = sgx_tse::rsgx_verify_report(&report);
    }
    println!("sgx_verify_report, {:?}", start.elapsed());
}

fn bench_sgx_self_report() {
    let start = Instant::now();
    for _ in 0..TEST_COUNT {
        let _ = sgx_tse::rsgx_self_report();
    }
    println!("sgx_self_report, {:?}", start.elapsed());
}

fn bench_sgx_seal_data() {
    let aad: [u8; 0] = [0_u8; 0];
    let mut payload: Vec<u8> = vec![0;100000];
    let mut rnd = sgx_rand::StdRng::new().unwrap();
    rnd.fill_bytes(&mut payload);
    let mut sz = 1;
    for _ in 0..6 {
        let start = Instant::now();
        for _ in 0..TEST_COUNT {
            let _ = SgxSealedData::<[u8]>::seal_data(&aad, &payload[..sz]);
        }
        println!("sgx_seal_data size = {}, {:?}", sz, start.elapsed());
        sz *= 10;
    }
}

fn bench_sgx_unseal_data() {
    let aad: [u8; 0] = [0_u8; 0];
    let mut payload: Vec<u8> = vec![0;100000];
    let mut rnd = sgx_rand::StdRng::new().unwrap();
    rnd.fill_bytes(&mut payload);
    let mut sz = 1;
    for _ in 0..6 {
        let start = Instant::now();
        let d = SgxSealedData::<[u8]>::seal_data(&aad, &payload[..sz]).unwrap();
        for _ in 0..TEST_COUNT {
            let _ = d.unseal_data();
        }
        println!("sgx_seal_data size = {}, {:?}", sz, start.elapsed());
        sz *= 10;
    }
}

fn bench_sgx_fopen() {
    use std::sgxfs::SgxFile;
    let _ = SgxFile::create("sgx_file").unwrap();

    let report = sgx_tse::rsgx_self_report();
    let key_policy = SGX_KEYPOLICY_MRSIGNER;
    let isv_svn = report.body.isv_svn;
    let reserved1 = 0u16;
    let cpu_svn = report.body.cpu_svn;
    let attribute_mask = sgx_attributes_t{flags: TSEAL_DEFAULT_FLAGSMASK, xfrm: 0};
    let key_id = sgx_key_id_t::default();
    let misc_mask = 0;
    let config_svn = report.body.config_svn;
    let reserved2 = [0u8; SGX_KEY_REQUEST_RESERVED2_BYTES];

    let kr: sgx_key_request_t = sgx_key_request_t {
        key_name: SGX_KEYSELECT_SEAL,
        key_policy,
        isv_svn,
        reserved1,
        cpu_svn,
        attribute_mask,
        key_id,
        misc_mask,
        config_svn,
        reserved2};

    let k = sgx_tse::rsgx_get_key(&kr).unwrap();

    let start = Instant::now();
    let fname = std::ffi::CString::new("sgx_file").unwrap();
    let r = std::ffi::CString::new("w").unwrap();
    for _ in 0..TEST_COUNT {
        let _ = sgx_tprotected_fs::SgxFileStream::open(
            &fname,
            &r,
            &k).unwrap();
    };
    println!("sgx_fopen, {:?}", start.elapsed());
}

fn bench_sgx_fopen_autokey() {
    let p = std::ffi::CString::new("sgx_file").unwrap();
    let r = std::ffi::CString::new("w").unwrap();
    let start = Instant::now();
    for _ in 0..TEST_COUNT {
        let _ = sgx_tprotected_fs::SgxFileStream::open_auto_key(
            &p,
            &r).unwrap();
    };
    println!("sgx_fopen_autokey, {:?}", start.elapsed());
}

fn bench_sgx_export_autokey() {
    let p = std::ffi::CString::new("sgx_file").unwrap();
    let r = std::ffi::CString::new("w").unwrap();
    let _ = sgx_tprotected_fs::SgxFileStream::open_auto_key(&p, &r).unwrap();
    let start = Instant::now();
    for _ in 0..TEST_COUNT {
        let _ = sgx_tprotected_fs::export_auto_key(&p).unwrap();
    };
    println!("sgx_fexport_autokey, {:?}", start.elapsed());
}

fn bench_sgx_import_autokey() {
    let p = std::ffi::CString::new("sgx_file").unwrap();
    let r = std::ffi::CString::new("w").unwrap();
    let _ = sgx_tprotected_fs::SgxFileStream::open_auto_key(&p, &r).unwrap();
    let start = Instant::now();
    for _ in 0..TEST_COUNT {
        let ak = sgx_tprotected_fs::export_auto_key(&p).unwrap();
        let _ = sgx_tprotected_fs::import_auto_key(&p, &ak).unwrap();
    };
    println!("sgx_fimport_autokey + sgx_fexport_autokey, {:?}", start.elapsed());
}
