// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Nekoer <hcyacg@vip.qq.com>

fn main() {
    println!("cargo:rerun-if-changed=src/sign.c");
    cc::Build::new().file("src/sign.c").compile("sign");
}
