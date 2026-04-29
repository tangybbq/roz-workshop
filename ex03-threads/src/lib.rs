// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)]

use log::info;
use zephyr::sync::channel::{self, Receiver, Sender};
use zephyr::time::{sleep, Duration};

#[zephyr::thread(stack_size = 2048)]
fn producer(sender: Sender<u32>) {
    let mut count: u32 = 0;
    loop {
        sleep(Duration::secs_at_least(1));
        sender.send(count).unwrap();
        count = count.wrapping_add(1);
    }
}

#[zephyr::thread(stack_size = 2048)]
fn consumer(receiver: Receiver<u32>) {
    loop {
        let tick = receiver.recv().unwrap();
        info!("tick: {}", tick);
    }
}

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    let (sender, receiver) = channel::bounded(4);

    let producer_thread = producer(sender).start();
    let _consumer_thread = consumer(receiver).start();

    // Both threads loop forever; join the producer so rust_main does not
    // return. Returning would idle Zephyr, not exit QEMU — exit with Ctrl-a x.
    producer_thread.join().unwrap();
}
