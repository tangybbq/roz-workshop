<!--
Copyright (c) 2026 Linaro, LTD
SPDX-License-Identifier: Apache-2.0
-->

# Exercise 03 — Threads + a channel ticker

25 minutes. The starter ships complete: a producer thread counts at
1 Hz and sends each tick over a bounded channel; a consumer thread
receives and logs. You build it, run it, then read the code and
make a small change.

The cognitive work here is reading and tracing — not blank-page
coding. Module 4b (ex04-async) rewrites this same producer/consumer
shape as Embassy tasks, so internalize the thread version first.

## 1. Build and run

From the exercise directory:

```
cd roz-workshop/ex03-threads
west build -b qemu_cortex_m3 .
west build -t run
```

If rust-analyzer in your editor was working in ex02, re-apply the
symlink trick from [`../common/README.md`](../common/README.md) — the
per-exercise `.cargo/config.toml` is regenerated under `build/` and
needs to be re-linked once.

You should see `tick: 0`, `tick: 1`, `tick: 2` … under QEMU. Exit
with `Ctrl-a x`.

> **Note on QEMU timing.** When every thread is blocked waiting on a
> sleep or channel receive, QEMU advances its virtual clock to the
> next scheduled event instead of wall-sleeping. Printed tick counts
> are truthful — Zephyr sees the correct elapsed ticks — but the
> terminal output may arrive in a burst rather than one line per
> wall-second. On a real board, output paces naturally.

## 2. Read the thread declarations

Open `src/lib.rs`. Two functions are annotated:

```rust
#[zephyr::thread(stack_size = 2048)]
fn producer(sender: Sender<u32>) { … }

#[zephyr::thread(stack_size = 2048)]
fn consumer(receiver: Receiver<u32>) { … }
```

The `#[zephyr::thread]` proc macro is what made Module 3a's
"static allocation" claim concrete. For each annotated function,
the macro emits:

- a static `ThreadStack<2048>` (the 2 KiB stack you asked for);
- a static `k_thread` control block;
- a wrapper function with the original name that, when called,
  initializes the thread in a *non-running* state and returns a
  `ReadyThread`.

`pool_size` defaults to 1, so each declaration is a singleton —
calling `producer(...)` a second time before the previous one has
exited would panic.

## 3. Trace the lifecycle in `rust_main`

```rust
let (sender, receiver) = channel::bounded(4);

let producer_thread = producer(sender).start();
let _consumer_thread = consumer(receiver).start();

producer_thread.join().unwrap();
```

Three things to identify, in order:

- **What does `producer(sender)` return?** A `ReadyThread` — the
  Zephyr thread exists and is initialized, but Zephyr is not yet
  scheduling it. The macro built it that way deliberately so you
  can adjust priority or other settings before it runs.
- **What does `.start()` do?** Wakes the thread (under the hood, a
  `k_wakeup`) and returns a `RunningThread`. From here it is a
  normal scheduled Zephyr thread.
- **Why does the channel need any depth?** `bounded(4)` allocates
  a 4-slot ring buffer once at creation time (this is what
  `CONFIG_RUST_ALLOC=y` is for in `prj.conf` — the buffer comes
  from the heap, but only this once, not per message). If the
  producer ever runs ahead of the consumer, those 4 slots
  absorb the burst. If the gap exceeds 4, `sender.send(...)`
  blocks until the consumer drains a slot — backpressure for free.

`producer_thread.join()` blocks `rust_main` forever, because the
producer loops forever. We deliberately do not let `rust_main`
return: returning would idle Zephyr, not exit QEMU. Use `Ctrl-a x`
to leave the emulator.

## 4. Make a change

Pick one and rebuild:

- Change the tick interval to 500 ms. `Duration::millis_at_least`
  is the constructor you want.
- Include the Zephyr uptime in the log. `zephyr::time::now()`
  returns an `Instant`; its `.ticks()` method gives a printable
  number.

Either edit lands in one or two lines. Rebuild with
`west build -b qemu_cortex_m3 .` and re-run with
`west build -t run`.

## Stretch — a `Stop` message

Replace the channel item type with an enum, send a `Stop` after
some delay, and have the consumer break out of its loop on it:

```rust
enum Message {
    Tick(u32),
    Stop,
}
```

Sketch:

- Change the channel to `bounded::<Message>(4)`.
- Add a third `#[zephyr::thread]` that sleeps for 10 seconds and
  then sends `Message::Stop`.
- In the consumer, `match` on the received value — log on
  `Tick(n)`, log "stopping" on `Stop`, then `return` from the
  function so the consumer thread exits cleanly.
- In `rust_main`, join the consumer instead of the producer.

When the consumer returns, Zephyr keeps running its idle thread —
QEMU may not exit on its own, and that's fine. The point is that
*your* threads have shut down deterministically.

> **Heads up — bounded channel handles can't be dropped.** Zephyr's
> kernel may still hold pointers into the channel's storage (e.g., a
> thread parked inside `k_msgq`), so reclaiming the storage would be
> unsound — and the crate panics on the last `Drop` of either side
> rather than do so. In a normal exit path you have to leak the handle
> with `core::mem::forget(...)` before the thread returns. See
> [`solution/src/lib.rs`](solution/src/lib.rs) for the full pattern.

## If the exercise crashes unexpectedly

Stack overflow detection is unreliable on `qemu_cortex_m3` — the
guard pages don't always trip before the crash. 2048 bytes is enough
for the starter, but if you extend either thread with bigger format
strings or larger local buffers, raise `stack_size` on the
`#[zephyr::thread]` attribute before chasing the bug elsewhere.
`CONFIG_MAIN_STACK_SIZE=4096` in `prj.conf` covers `rust_main`
itself; the per-thread `stack_size` covers the producer and
consumer.

## Looking ahead

ex04 takes this same producer/consumer pair and rewrites it as two
Embassy `async fn`s on a single executor. The headline difference:
no static stacks per task, fewer kernel objects, and `select!` lets
you compose "tick or stop" without a third thread.
