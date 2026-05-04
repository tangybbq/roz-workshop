// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)]

mod better_way;
// `hard_way` is compiled but not called — the call site in `rust_main`
// is commented out (it would invoke undefined behaviour).  Reading the
// module and finding the seeded bug is Task 1 of the exercise.
#[allow(dead_code)]
mod hard_way;

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    // --- Better way ---
    let mut i2c = zephyr::devicetree::aliases::i2c_bus::get_instance()
        .expect("i2c_bus alias not found in devicetree");
    assert!(i2c.is_ready(), "I2C bus not ready");
    better_way::read_sensor(&mut i2c);

    // --- Hard way (do NOT call in normal use — dangling pointer bug) ---
    // Uncomment to invoke; you would obtain `*const raw::device` from the
    // same DT alias by reaching past the safe wrapper, e.g. by exposing
    // `i2c.device` through a pub(crate) accessor.  The point of the
    // exercise is that the hard-way path does not give you `is_ready()`
    // and does not stop you from passing in a UART or a dangling pointer.
    //
    // hard_way::read_sensor(/* *const raw::device */);
}
