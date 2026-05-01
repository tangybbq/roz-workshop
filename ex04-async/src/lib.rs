// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)]
// The two task bodies below start out empty; the imports and the static
// CHANNEL they will need are already in place. These `unused`/`dead_code`
// allows keep the starter quiet until participants fill the bodies in.
#![allow(unused_imports, dead_code)]

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
    // TODO: Loop forever, sending an incrementing u32 counter into CHANNEL
    // every 1 second.
    //
    // Hint:
    //     Timer::after(Duration::from_secs(1)).await;
    //     CHANNEL.send(value).await;
}

#[embassy_executor::task]
async fn consumer() {
    // TODO: Loop forever, waiting on either CHANNEL.receive() OR a 3-second
    // idle timeout, and log a message for whichever branch fires.
    //
    // Hint:
    //     match select(CHANNEL.receive(), Timer::after(Duration::from_secs(3))).await {
    //         Either::First(value) => info!("tick: {}", value),
    //         Either::Second(_) => info!("idle (no tick for 3s)"),
    //     }
}
