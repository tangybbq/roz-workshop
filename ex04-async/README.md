<!--
Copyright (c) 2026 Linaro, LTD
SPDX-License-Identifier: Apache-2.0
-->

# Exercise 04 — Embassy async on Zephyr

25 minutes. Same producer/consumer shape you read in ex03, rewritten as
two `#[embassy_executor::task]` async functions on a single executor.
Once the rewrite matches ex03's behavior, you add a `select!` idle-timeout
branch to the consumer — the kind of composition that the threaded
version cannot express without a third thread and shared state.

The starter ships the boilerplate (executor setup, static channel, task
declarations); the two task bodies are stubs marked `// TODO`. You fill
those in. If you get stuck, [`solution/`](solution/) holds a complete
working version.

## 1. Build and run the starter

From the exercise directory:

```
cd roz-workshop/ex04-async
west build -b qemu_cortex_m3 .
west build -t run
```

Re-apply the symlink trick from [`../common/README.md`](../common/README.md)
once for rust-analyzer.

The starter compiles, the executor starts, and both tasks spawn — but
their bodies are empty, so no tick output appears. That is expected.
Exit QEMU with `Ctrl-a x`.

> **Note on QEMU timing.** Same caveat as ex03: when every task is
> awaiting a timer or a channel, QEMU advances virtual time to the next
> scheduled event. Output may arrive in bursts, not paced one-per-second
> at the wall clock. The tick numbers themselves are accurate.

## 2. Read the boilerplate

Open `src/lib.rs`. Three things are already in place:

```rust
static EXECUTOR: StaticCell<Executor> = StaticCell::new();
static CHANNEL: Channel<CriticalSectionRawMutex, u32, 4> = Channel::new();
```

- The `Executor` is held in a `StaticCell` because it is initialized
  exactly once, at runtime, but lives for the rest of the program. This
  is the Embassy idiom that Module 4a covered.
- `CHANNEL` is `embassy_sync::channel::Channel`, **not** the
  `zephyr::sync::channel::bounded` from ex03. The Zephyr channel's
  `recv()` blocks the OS thread, which on a single-threaded executor
  would freeze every task. `embassy_sync`'s channel is async-aware:
  `.receive().await` suspends only the calling task. The channel itself
  is a plain `static` with its 4-slot ring buffer baked into the type
  — no `Box`, no allocation.

```rust
extern "C" fn rust_main() {
    unsafe { zephyr::set_logger().unwrap(); }
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(producer()).unwrap();
        spawner.spawn(consumer()).unwrap();
    });
}
```

`executor.run` never returns. The closure runs once at startup to
spawn the initial tasks; from then on the executor polls them forever.

## 3. Write the producer (~5 min)

Replace the `// TODO` in `producer()` with the body. Goal: send an
incrementing `u32` into `CHANNEL` once a second, forever.

```rust
let mut count: u32 = 0;
loop {
    Timer::after(Duration::from_secs(1)).await;
    CHANNEL.send(count).await;
    count = count.wrapping_add(1);
}
```

`Timer::after` returns a `Future`; the `.await` is what actually
suspends the task and lets the executor run the consumer. Compare to
ex03's `sleep()` call, which blocked the OS thread.

> The starter pre-imports `embassy_time::Duration` and `Timer` for
> you. Note that `embassy_time::Duration` is a *different type* from
> `zephyr::time::Duration`; if you `use` the wrong one, the call won't
> resolve.

Rebuild and run. You should still see no output yet — the consumer is
also a stub.

## 4. Write the consumer — first version (~3 min)

Start with the simplest possible body:

```rust
loop {
    let value = CHANNEL.receive().await;
    info!("tick: {}", value);
}
```

Rebuild and run. `tick: 0`, `tick: 1`, `tick: 2`, … just like ex03.

Notice what just happened: producer and consumer are running on the
*same OS thread*. There is no kernel context switch between
`CHANNEL.send(count).await` returning and `CHANNEL.receive().await`
resuming the consumer — the executor walks from one ready future to
the next.

## 5. Add the `select!` branch (~5–8 min)

Now the payoff. Replace the simple receive with a `select` between the
channel and a 3-second timeout:

```rust
loop {
    match select(CHANNEL.receive(), Timer::after(Duration::from_secs(3))).await {
        Either::First(value) => info!("tick: {}", value),
        Either::Second(_) => info!("idle (no tick for 3s)"),
    }
}
```

Rebuild and run. Under normal operation (producer ticking at 1 Hz, the
3-second timeout never wins) the output looks the same as before.

Now go break it: change the producer's sleep to
`Duration::from_secs(5)` and rebuild. Every third tick or so the
`Either::Second` branch wins — you see `idle` messages between the
slower ticks. Restore it to 1 second when you're done.

`select` does not need a third task and there is no shared state to
guard. That's the headline.

## 6. Compare to ex03 (discussion, ~2 min)

A back-of-the-envelope count of static memory footprint:

| ex03 (threads)                           | ex04 (Embassy)                        |
|------------------------------------------|---------------------------------------|
| 2 × `ThreadStack<2048>` = 4096 B         | 1 × Embassy task arena = 2048 B       |
| 2 × `k_thread` control blocks            | 1 × `StaticCell<Executor>`            |
| Boxed channel ring buffer (heap, once)   | Static `Channel` (compiled-in)        |

ex04 is meaningfully smaller, even before counting the heap that ex03
needs at all (recall ex03's `CONFIG_RUST_ALLOC=y`). And the `select`
branch you just added would have cost ex03 a third thread, an enum
message, and a `match` in the consumer.

When would you still prefer threads? When tasks are CPU-bound (one
runaway task starves the executor), when you want hard priority
preemption between work units (Zephyr's scheduler does that for free
across threads), or when you're integrating with a blocking C API
that you can't drive asynchronously.

## Stretch — two executors at different priorities

The lecture sketched running multiple Embassy executors on different
Zephyr threads, so that a high-priority executor can preempt a
low-priority one. Implement it: declare a second Zephyr thread with
`kobj_define! { … StaticThread … }`, run a second
`StaticCell<Executor>` on it at a lower priority, and move the consumer
onto that low-priority executor.

The pattern is in
[`modules/lang/rust/samples/embassy/src/lib.rs`](../../modules/lang/rust/samples/embassy/src/lib.rs)
— look at `LOW_THREAD`, `LOW_STACK`, `low_executor`, and the
`raw::k_thread_priority_set` call. Don't try to derive it from scratch
in 5 minutes; lift the structure and adapt.

If you can get the producer (high-prio executor) sending into the
shared `CHANNEL` while the consumer (low-prio executor) drains it,
you've reproduced the multi-executor priority story end-to-end.

## If your build fails at task spawn

If `spawner.spawn(...)` panics with an arena-related message at
startup, the Embassy task arena is too small. The starter sets
`task-arena-size-2048` in `Cargo.toml`'s feature flags on
`embassy-executor`; bump it (e.g., to `task-arena-size-4096`) and
rebuild. The 2048 default is sized for these two tasks plus a little
slack.

## Looking ahead

Module 5 moves from kernel primitives to device drivers — the same
DT-and-Kconfig story you toured in ex01, but with Rust types on top.
You'll see the safe-bindings-vs-raw-`zephyr-sys` contrast directly in
ex05.
