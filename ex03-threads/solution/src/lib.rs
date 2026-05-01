// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)]

use core::mem;
use log::info;
use zephyr::sync::channel::{self, Receiver, Sender};
use zephyr::time::{sleep, Duration};

enum Message {
    Tick(u32),
    Stop,
}

#[zephyr::thread(stack_size = 2048)]
fn producer(sender: Sender<Message>) {
    let mut count: u32 = 0;
    loop {
        sleep(Duration::secs_at_least(1));
        sender.send(Message::Tick(count)).unwrap();
        count = count.wrapping_add(1);
    }
}

#[zephyr::thread(stack_size = 2048)]
fn stopper(sender: Sender<Message>) {
    sleep(Duration::secs_at_least(10));
    sender.send(Message::Stop).unwrap();
    // Dropping the last Sender would try to reclaim the channel's backing
    // storage, but Zephyr's kernel may still hold pointers into it (e.g.,
    // a thread parked inside k_msgq). Reclaiming would be unsound, so the
    // crate panics rather than free. Leak the handle to keep things sound.
    mem::forget(sender);
}

#[zephyr::thread(stack_size = 2048)]
fn consumer(receiver: Receiver<Message>) {
    loop {
        match receiver.recv().unwrap() {
            Message::Tick(n) => info!("tick: {}", n),
            Message::Stop => {
                info!("stopping");
                // Same soundness reason as in stopper(): Zephyr may still
                // reference the queue storage, so leak this handle.
                mem::forget(receiver);
                return;
            }
        }
    }
}

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    let (sender, receiver) = channel::bounded(4);

    let _producer_thread = producer(sender.clone()).start();
    let _stopper_thread = stopper(sender).start();
    let consumer_thread = consumer(receiver).start();

    // Join the consumer: it returns when it sees Message::Stop, after which
    // rust_main returns. Zephyr keeps running its idle thread; QEMU does not
    // exit on its own — leave with Ctrl-a x.
    consumer_thread.join().unwrap();
}
