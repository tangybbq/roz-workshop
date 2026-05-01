// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)]

use embassy_futures::select::{select, Either};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use log::info;
use static_cell::StaticCell;
use zephyr::embassy::Executor;

static EXECUTOR: StaticCell<Executor> = StaticCell::new();
static CHANNEL: Channel<CriticalSectionRawMutex, u32, 4> = Channel::new();

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(producer()).unwrap();
        spawner.spawn(consumer()).unwrap();
    });
}

#[embassy_executor::task]
async fn producer() {
    let mut count: u32 = 0;
    loop {
        Timer::after(Duration::from_secs(1)).await;
        CHANNEL.send(count).await;
        count = count.wrapping_add(1);
    }
}

#[embassy_executor::task]
async fn consumer() {
    loop {
        match select(CHANNEL.receive(), Timer::after(Duration::from_secs(3))).await {
            Either::First(value) => info!("tick: {}", value),
            Either::Second(_) => info!("idle (no tick for 3s)"),
        }
    }
}
