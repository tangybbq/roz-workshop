// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

//! Better-way I²C: typed safe bindings.
//!
//! The same SHT45 read sequence as `hard_way`, expressed through
//! `zephyr::device::i2c::I2c`.  Read this file alongside `hard_way.rs`
//! and identify, for each line, what invariant the type system is
//! enforcing that the raw version was not.

use log::info;
use zephyr::device::i2c::I2c;
use zephyr::time::{sleep, Duration};

const SENSOR_ADDR: u16 = 0x44;

pub fn read_sensor(i2c: &mut I2c) {
    // Trigger a high-precision measurement.
    i2c.write(SENSOR_ADDR, &[0xFDu8])
        .expect("i2c write failed");

    // Wait for conversion (~10 ms per the SHT45 datasheet).
    sleep(Duration::millis_at_least(10));

    // Read 6 bytes: temp (2) + CRC + humidity (2) + CRC.
    let mut buf = [0u8; 6];
    i2c.read(SENSOR_ADDR, &mut buf)
        .expect("i2c read failed");

    info!(
        "raw: {:02x} {:02x} {:02x} {:02x}",
        buf[0], buf[1], buf[3], buf[4]
    );
}
