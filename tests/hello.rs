#![cfg(target_arch = "wasm32")]

// SPDX-FileCopyrightText: 2021 Nick Fitzgerald <fitzgen@gmail.com>
// SPDX-FileCopyrightText: 2021 The Rust and WebAssembly Working Group
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[no_mangle]
pub fn fluxions(x: usize) -> usize {
    unsafe {
        imported(x);
        imported2(x);
        imported3(x);
        imported4(x)
    }
}

#[no_mangle]
pub fn quicksilver(x: usize) {
    if x > 100 {
        snip_me();
    }
}

extern "C" {
    fn imported(x: usize) -> usize;
    fn imported2(x: usize) -> usize;
    fn imported3(x: usize) -> usize;
    fn imported4(x: usize) -> usize;
}

#[inline(never)]
fn snip_me() {
    println!("this is gonna get snipped");
}

pub fn main() {}
