<!--
SPDX-FileCopyrightText: 2021 Nick Fitzgerald <fitzgen@gmail.com>
SPDX-FileCopyrightText: 2021 The Rust and WebAssembly Working Group

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

To regenerate the wasm file:

```
$ rustc +nightly --target wasm32-unknown-unknown -O -g ./hello.rs
```
