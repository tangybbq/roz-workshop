// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

//! Hard-way I²C: raw `zephyr-sys` bindings.
//!
//! This file has a subtle memory-safety bug.  Find it before reading
//! [`crate::better_way`].  The bug is real C-style undefined behaviour;
//! whether it manifests at runtime is up to the optimiser and the stack
//! frame layout of the day.

use zephyr::raw;

const SENSOR_ADDR: u16 = 0x44;

/// Returns a pointer to the SHT45 measurement command byte.
///
/// Called just before the I²C write.
fn measure_cmd_ptr() -> *const u8 {
    let cmd = [0xFDu8]; // high-precision measurement command
    cmd.as_ptr()
    // BUG: `cmd` is dropped at end of this function; the returned
    // pointer is immediately dangling.  The borrow checker stops
    // tracking provenance the moment the reference becomes a raw
    // pointer, so this compiles without warning.
}

pub fn read_sensor(dev: *const raw::device) {
    // Trigger a high-precision measurement.
    //
    // Note: `unsafe` here signals that *this call* has been audited.
    // It says nothing about the validity of the pointer argument —
    // `ptr` came from a safe function and looks innocuous at this
    // call site.
    let ptr = measure_cmd_ptr();
    unsafe { raw::zr_i2c_write(dev, ptr, 1, SENSOR_ADDR) };

    // Wait for the sensor to complete its measurement (~10 ms).
    unsafe { raw::k_msleep(10) };

    // Read the result: 2 bytes temp, 1 CRC, 2 bytes humidity, 1 CRC.
    let mut buf = [0u8; 6];
    unsafe { raw::zr_i2c_read(dev, buf.as_mut_ptr(), 6, SENSOR_ADDR) };

    // `printkln!` is used here instead of `log::info!` so the output is
    // unmistakably from the hard-way path.  `raw::printk` itself is
    // variadic and not in the `zephyr-sys` allowlist; the dangerous
    // surface in this exercise is the device-pointer plumbing, not
    // formatted output.
    zephyr::printkln!(
        "raw: {:02x} {:02x} {:02x} {:02x}",
        buf[0], buf[1], buf[3], buf[4]
    );
}
