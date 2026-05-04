<!--
Copyright (c) 2026 Linaro, LTD
SPDX-License-Identifier: Apache-2.0
-->

# Workshop exercise tasks

Brief for Claude Code working in this repo. The Cowork agent (David, via
Claude, mostly looking at the notes in `roz-rw2026.md`) writes this file;
Claude Code reads it as the specification for each exercise-content pass.

Only flesh out exercises marked **READY FOR CODE PASS**. Leave
**DRAFT UNSPEC** entries alone — their shape is still being discussed.

## Shared constraints

Apply to every exercise unless overridden:

- **Apache-2.0 header on every file you create.** Markdown uses the HTML
  comment form at the very top (see this file). C/Rust uses `//`,
  YAML/shell/Python/Kconfig/`.gitignore` uses `#`. The `LICENSE` file is
  the only exception.
- **Hardware: QEMU only.** Primary board `qemu_cortex_m3`. Only reach for
  `qemu_riscv32` in exercises that explicitly call for it (currently just
  ex01 as a stretch).
- **T2 topology.** The workspace dir is the parent of this repo. The
  canonical build invocation is from inside the exercise directory, so
  each exercise keeps its own `build/`:
  ```
  cd roz-workshop/ex0N-…
  west build -b qemu_cortex_m3 .
  west build -t run
  ```
  Per-exercise READMEs should use that form. The top-level `README.md`
  retains a workspace-dir invocation in its quick-start section, but
  only as a smoke test against `modules/lang/rust/samples/hello_world`.
- **Each exercise is a standalone Zephyr application.** `CMakeLists.txt`,
  `prj.conf`, `Cargo.toml`, `src/main.rs`, plus whatever Kconfig fragments
  the exercise needs. Use `modules/lang/rust/samples/hello_world` as the
  canonical shape — do not invent a different layout.
- **Commit `Cargo.lock`** for each exercise (reproducibility).
- **Don't commit `.cargo/config.toml`** — already in `.gitignore`, it's a
  symlink generated per build.
- **Reference `common/README.md`** for the symlink trick; don't duplicate
  those instructions into per-exercise READMEs.
- **Do NOT run `west build -t run` or `west build -t qemu` during a code
  pass.** QEMU runs interactively and has no reliable exit path from a
  non-interactive shell. Verify that the application *builds* cleanly
  (`west build -b qemu_cortex_m3 .`), but do not launch it. If you need
  to confirm runtime output, flag it in the commit message and ask David
  to run it manually.

## Cross-cutting content

### QEMU virtual time — mention in the first exercise that exhibits it

When every thread is blocked waiting on a timer or sleep, QEMU advances
its virtual clock to the next scheduled event instead of wall-sleeping.
Printed uptimes are truthful — Zephyr sees the elapsed virtual ticks —
but terminal output arrives in a burst, not one line per wall-second. A
real board paces naturally.

This first shows up in **ex03** (the threaded ticker). Include a short
callout there. Later exercises (ex04) can reference it rather than
repeating.

## Exercise index

| #  | Directory          | Teaches                        | After section      | Time  | Status              |
|----|--------------------|--------------------------------|--------------------|-------|---------------------|
| 01 | ex01-explore       | Tour the Zephyr tree           | Module 1a (What is Zephyr)   | 20m   | READY FOR CODE PASS |
| 02 | ex02-rust-analyzer | rust-analyzer + Cargo edit     | Module 2a (Build pipeline)   | 15m   | READY FOR CODE PASS |
| 03 | ex03-threads       | Threads + channel ticker       | Module 3a (Threads + sync)   | 25m   | READY FOR CODE PASS |
| 04 | ex04-async         | Embassy async rewrite of ex03  | Module 4a (Async)            | 25m   | READY FOR CODE PASS |
| 05 | ex05-i2c           | Safe bindings vs raw sys       | Module 5a (Devices)          | 25m   | READY FOR CODE PASS |

Times are targets against a ~55/45 lecture/hands-on pacing over 5 hours;
confirm once slide timings are firmer.

## ex01-explore — READY FOR CODE PASS

**Placement:** Module 1b in `workshop-outline.md`, immediately after
the "What is Zephyr" lecture (Module 1a). 20 min budget.

**What attendees have at this point:**
- A working toolchain — or a half-broken one (see "Tolerance" below).
  They've built `modules/lang/rust/samples/hello_world` as the
  pre-workshop smoke test, so the build environment has been exercised
  at least once.
- Mental model of Zephyr at the level of Module 1a: board, shield,
  Kconfig, DT, west, "SEGFAULT-typed C," kitchen-sink kernel.
- *Not yet:* the build-pipeline lecture (Module 2a), rust-analyzer
  setup, the zephyr crate tour, threads, sync, async, drivers.

**Important — exception to the shared-app constraint:** this exercise
is a guided reading tour over the upstream Zephyr tree and the existing
`samples/hello_world` build. It does NOT have its own Zephyr
application. `ex01-explore/` contains only `README.md`. The "Each
exercise is a standalone Zephyr application" shared constraint does not
apply here.

**Tolerance for half-broken environments:** tasks 1 and 2 are pure
reading. Someone whose toolchain is still misbehaving can do them and
observe a neighbor for the build steps. Don't structure the README to
force the tasks in strict order if the build is failing — make it clear
that 1 and 2 stand alone.

**README.md walks attendees through (~5 min each):**

1. **Find a board definition YAML.** Open the qemu_cortex_m3 board
   YAML in the Zephyr tree (likely
   `zephyr/boards/qemu/qemu_cortex_m3/qemu_cortex_m3.yaml` after the
   v3.6 board reorg, but verify against v4.4.0 at code-pass time).
   Read it. What does this declare? Arch, supported features, default
   toolchain, ram/rom budget. Note: this is one of >1000 such files
   in tree.

2. **Locate the DTS for the same board.** Walk the `.dts` / `.dtsi`
   include chain. Find the UART node serving the console (look for
   `zephyr,console` chosen — code pass: confirm exact label). What's
   the node's compatible string?

3. **Rebuild `hello_world` for `qemu_riscv32`.** From the workspace dir:
   ```
   cd modules/lang/rust/samples/hello_world
   west build -p -b qemu_riscv32 .
   ```
   `-p` is pristine — required when changing boards. Same Rust source,
   different ISA. Note what changed in `build/`. (Stretch goers can
   `west build -t run` and confirm it works under qemu_riscv32 too.)

4. **Skim a generated header.** Open
   `build/zephyr/include/generated/devicetree_generated.h` (verify
   exact path in v4.4.0 — it has moved before). It's preprocessor-macro
   spaghetti. You're not expected to read it deeply — note that this
   is what Zephyr's "device tree" actually compiles down to from C's
   POV. Module 5 will explain how `zephyr-build`'s `build_dts()`
   parses these into typed Rust modules.

**Stretch:** flip a Kconfig value in `samples/hello_world/prj.conf`
(e.g., `CONFIG_LOG_DEFAULT_LEVEL=4`), rebuild, and `diff` the new
`build/zephyr/.config` against the previous one. See how a one-line
Kconfig change cascades into the generated config.

**Success criteria:** each attendee can answer "where is the UART node
defined?" and "what changes when you switch to `qemu_riscv32`?" by
having put their fingers on the actual files. They form questions
about Kconfig and DT that Modules 2 and 5 will answer.

**Not this exercise's job:** explaining what Kconfig or DT *means*
beyond what Module 1a covered, running anything they've authored
themselves (ex02 onward), or any Walkthrough content.

**Open decisions / code-pass verifications:**
- Board YAML and DTS paths in v4.4.0 — Zephyr restructured the
  `boards/` tree at v3.6. Verify exact paths against the pinned tree
  before committing the README.
- `devicetree_generated.h` is the canonical "DT macro soup" example,
  but `build/zephyr/include/generated/zephyr/autoconf.h` is also a
  good "Kconfig→C" exhibit. Pick one or include both as a
  compare-and-contrast.
- For the Kconfig stretch, pick a flag whose diff is illustrative but
  not destructive — `CONFIG_LOG_DEFAULT_LEVEL` is safe;
  `CONFIG_PRINTK` toggle is too aggressive.
- Should the riscv32 build be required or stretch? Outline lists it as
  task 3 (required). My instinct says required, since attendees will
  need riscv32 elsewhere too — but it does mean they all need to have
  installed `riscv32imac-unknown-none-elf`. Confirm in
  `pre-workshop-setup.md` that this is a required prereq, not optional.

## ex02-rust-analyzer — READY FOR CODE PASS

**Placement:** Module 2b in `workshop-outline.md`, immediately after
Module 2a covers the build pipeline and the cargo-config bridge.
15 min budget.

**What attendees have:** ex01 done (or skipped — tree-tour was
recovery-friendly). Module 2a heard: Zephyr-Rust build sequence,
cargo-config template, symlink concept just lectured but not yet
applied. Not yet: zephyr crate tour (Module 2c, after this), threads,
sync, async, drivers.

**Starter source:** copy the upstream `samples/hello_world` from
`modules/lang/rust/samples/hello_world` verbatim into
`ex02-rust-analyzer/`. Keep it faithful to upstream — Module 2a
referred to "the hello_world layout" specifically. Add Apache-2.0
headers to any files that don't already have a compatible one.

**README.md walks attendees through:**

1. Build first (so the cargo-config template exists):
   ```
   cd roz-workshop/ex02-rust-analyzer
   west build -b qemu_cortex_m3 .
   ```

2. Apply the symlink recipe from
   [`../common/README.md`](../common/README.md):
   ```
   mkdir -p .cargo
   ln -sf ../build/rust/sample-cargo-config.toml .cargo/config.toml
   ```

3. Run `cargo check` from the exercise dir. Should succeed.

4. Open the exercise dir in your editor. Confirm rust-analyzer is
   happy: hover types, jump-to-definition, autocomplete on `zephyr::`.

5. **Now make a real change — add a `no_std` ecosystem crate.** Add
   ```toml
   heapless = "0.8"
   ```
   to `Cargo.toml` under `[dependencies]`. Run `cargo check` — it
   should pull `heapless` and pass.

6. In `src/main.rs`, build a message into a stack-allocated buffer with
   `heapless::String` and pass it through `printk!`:
   ```rust
   use heapless::String;
   use core::fmt::Write;

   let mut msg: String<32> = String::new();
   write!(&mut msg, "hello from rust-on-zephyr").unwrap();
   printk!("{}\n", msg);
   ```
   (Code pass: pin the right `heapless` version against the toolchain
   and verify the snippet compiles. v0.7→v0.8 had API churn around
   `String` constructors; pick a snippet that actually builds.)

7. Rebuild and run from the exercise dir:
   ```
   west build -b qemu_cortex_m3 .
   west build -t run
   ```
   See your message under QEMU.

**Note:** if you wipe `build/` or switch boards, re-run the symlink
recipe — the template gets regenerated and the symlink target is
content-equivalent but pristine-built.

**Success criteria:** rust-analyzer is working in the exercise dir;
attendees added a `no_std` ecosystem crate as a real Cargo dep, used it
in `rust_main`, built it, and saw the output under QEMU.

**Punchline (debrief slide):** adding a `no_std` ecosystem crate "just
works" on Rust-on-Zephyr — no platform-specific dance, no special
build flags. The broader `no_std` Rust ecosystem comes along for the
ride. This is one of the genuine wins of Rust-on-Zephyr over either
plain Zephyr-C or a more minimal embedded Rust HAL.

**Open decisions / code-pass verifications:**
- Pick the `heapless` version against whatever toolchain
  `zephyr-lang-rust` `devel` is pulling in transitively. Verify the
  snippet compiles unmodified.
- Side-bonus task between steps 4 and 5: type `zephyr::` somewhere and
  watch autocomplete come up. Cheap emotional payoff before they touch
  the Cargo edit.

**Parked for "if pre-workshop time allows":** a "this is broken — find
and fix it" challenge variant where the starter has a planted
rust-analyzer-visible error before they run `cargo check`. Sharper
pedagogy ("rust-analyzer caught it before the compiler"), but the
positive-feedback heapless beat is the primary version.

## ex03-threads — READY FOR CODE PASS

**Placement:** Module 3b in `workshop-outline.md`, immediately after
the "Threads and synchronization" lecture (Module 3a). 25 min budget.

**What attendees have at this point:**
- ex01 and ex02 done. Working rust-analyzer, successful builds, no_std
  crate experience.
- Module 3a heard: static allocation, `#[zephyr::thread]` proc macro,
  `ReadyThread` → `.start()` → `RunningThread` lifecycle, thread pools,
  `join`/`join_timeout`. Sync primitives covered: atomics, SpinMutex,
  Mutex, condvar, channels. The static/Pin problem explained.
- *Not yet:* async (Module 4a/4b), devices (Module 5).

**Application structure:** a standalone Zephyr application — the usual
`CMakeLists.txt`, `prj.conf`, `Cargo.toml`, `Cargo.lock`, and
`src/lib.rs`. Use `modules/lang/rust/samples/hello_world` as the shape
template.

**prj.conf:**
```
CONFIG_RUST=y
CONFIG_RUST_ALLOC=y
CONFIG_MAIN_STACK_SIZE=4096
CONFIG_LOG=y
CONFIG_LOG_BACKEND_RTT=n
```
`CONFIG_RUST_ALLOC=y` is required: `zephyr::sync::channel::bounded`
allocates its message pool once via `Box` at channel-creation time
(not per message). This is the right embedded choice — allocate up
front, not in the hot path — but it does need the heap.

**Design decisions (all settled):**

- **Go straight to two threads.** No single-threaded warm-up phase.
  The interesting part of this exercise is the channel and the thread
  lifecycle, not a simple sleep loop.
- **`#[zephyr::thread(stack_size = 2048)]` with no `pool_size`.**
  `pool_size` defaults to 1, making each declaration a singleton. One
  declaration for the producer, one for the consumer.
- **`.start()` dance is required.** The current API creates threads in
  a non-running state (`K_FOREVER` delay); `.start()` is what wakes them.
  `producer(sender).start()` returns a `RunningThread`. The README
  should make the two-step visible — it mirrors what the lecture said.
- **Bounded channel of depth 4.** `zephyr::sync::channel::bounded(4)`
  returns `(Sender<u32>, Receiver<u32>)`. The `Sender` and `Receiver`
  are `Send`, so they can be passed directly as thread arguments.
- **Channel-only synchronization.** No Mutex needed — the channel is the
  coordination point. SpinMutex/Mutex are lecture content, not exercise
  content here.
- **`zephyr::time::sleep(Duration)` for the 1 Hz tick.** Timer callbacks
  are the wrong API; `sleep` in a loop is idiomatic and matches the
  philosophers sample.
- **Runs forever; exit with `Ctrl-a x`.** Zephyr on `qemu_cortex_m3`
  has no programmatic QEMU exit mechanism. Both threads loop forever.
  `rust_main` starts both and then joins the producer (which also blocks
  forever). `Ctrl-a x` is how you exit QEMU from the exercise.
- **Stack size 2048 for thread stacks.** Should be adequate for a simple
  ticker with `log::info!`. Note in the README: stack overflow detection
  is unreliable on this target — if things crash mysteriously, suspect
  the stack. The `CONFIG_MAIN_STACK_SIZE=4096` in prj.conf covers
  `rust_main` itself.

**Starter source:** new application, not copied from hello_world. Write
`src/lib.rs` from scratch using hello_world's logger-init pattern, but
with the two-thread producer/consumer structure below. The file must
have the Apache-2.0 header.

**`src/lib.rs` structure (canonical shape for the code pass):**

```rust
// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)]

use zephyr::sync::channel::{self, Sender, Receiver};
use zephyr::time::{sleep, Duration};
use log::info;

#[zephyr::thread(stack_size = 2048)]
fn producer(sender: Sender<u32>) {
    let mut count: u32 = 0;
    loop {
        sleep(Duration::secs_at_least(1));
        sender.send(count).unwrap();
        count += 1;
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
    unsafe { zephyr::set_logger().unwrap(); }

    let (sender, receiver) = channel::bounded(4);

    let producer_thread = producer(sender).start();
    let _consumer_thread = consumer(receiver).start();

    // Both threads loop forever; join the producer so rust_main
    // doesn't return (which would idle Zephyr, not exit QEMU).
    producer_thread.join().unwrap();
}
```

**Verify before committing:** the above is the intended shape — confirm
it compiles and produces output under `west build -t run` before writing
the README.

**QEMU virtual time — include this callout in the README:**

> **Note on QEMU timing:** When every thread is blocked waiting on a
> sleep or channel receive, QEMU advances its virtual clock to the next
> scheduled event instead of wall-sleeping. Printed tick counts are
> truthful — Zephyr sees the correct elapsed ticks — but the terminal
> output may arrive in a burst rather than one line per wall-second.
> On a real board, output paces naturally.

**Stack overflow note (include in the README as a callout):**

> **If the exercise crashes unexpectedly:** stack overflow detection is
> unreliable on `qemu_cortex_m3` — the guard pages don't always fire
> before a crash. 2048 bytes is enough for this exercise, but if you
> extend it heavily (e.g., with format strings or large local variables),
> raise `stack_size` before debugging further.

**README.md structure (what Claude Code should write):**

The README should walk participants through at roughly the following
pace:

1. **Orientation** (1 min read) — what they're building: a producer
   thread that counts at 1 Hz and sends over a channel; a consumer
   that receives and logs. Point forward to ex04: this same structure
   gets rewritten as Embassy tasks.

2. **Build and run the starter** — the repo ships the complete starter
   for this exercise. Build it, run it under QEMU, observe the output.
   Include the QEMU virtual-time callout here.

3. **Read the thread declarations** — have participants look at the
   `#[zephyr::thread(stack_size = 2048)]` annotations on `producer`
   and `consumer`. What does the proc macro do? (Answer: declares
   static stack and thread data, wraps the function so calling it
   returns a `ReadyThread`.) Refer back to Module 3a's static-allocation
   discussion.

4. **Trace the lifecycle in `rust_main`** — `channel::bounded(4)`,
   `producer(sender).start()`, `consumer(receiver).start()`,
   `producer_thread.join()`. Have them identify: what type does
   `producer(sender)` return? What does `.start()` do? Why does the
   channel need depth at all — what happens if the producer is faster
   than the consumer?

5. **Modify it** — change the tick interval to 500 ms, or change the
   logged format to include the Zephyr uptime (`zephyr::time::now()`
   can give an `Instant`; its `.ticks()` value is printable). Rebuild
   and confirm. This is a short modification, not a full rewrite — the
   point is to touch the code, not invent a new exercise.

6. **Stretch: mixed-message stream** — change the channel type to a
   `Message` enum:
   ```rust
   enum Message {
       Tick(u32),
       Stop,
   }
   ```
   Add a third thread that sleeps for 10 seconds then sends `Message::Stop`.
   Have the consumer `match` on the received value and break out of its
   loop on `Stop`. `rust_main` joins the consumer instead of the producer.
   Exit should be clean (consumer logs "stopping" and returns; QEMU may
   or may not exit depending on Zephyr's idle behavior — that's fine).

**Success criteria:** participants have seen a working two-thread
producer/consumer, traced the `ReadyThread → .start() → RunningThread`
lifecycle in real code, and made at least one modification that required
a rebuild. They leave with a mental model of how channels replace shared
state and why bounded depth matters.

**What this exercise is NOT:** it is not asking participants to design
the threading structure from scratch. The starter ships working code.
The cognitive work is reading, tracing, and modifying — not blank-page
coding. This prepares them for ex04, where they do the rewrite.

**Solution directory:** create `ex03-threads/solution/` containing the
stretch goal implementation — the three-thread variant using a `Message`
enum. The solution is a complete, buildable Zephyr application (its own
`CMakeLists.txt`, `prj.conf`, `Cargo.toml`, `Cargo.lock`, `src/lib.rs`)
with the channel type changed to `bounded::<Message>(4)` where:
```rust
enum Message {
    Tick(u32),
    Stop,
}
```
The third thread sleeps 10 seconds then sends `Message::Stop`. The
consumer `match`es on the value and `break`s cleanly on `Stop`, logs
"stopping", and returns. `rust_main` joins the consumer thread (not the
producer) so it exits when the consumer does. This is self-contained
enough that participants can read and run it without the base ex03.

**Open decisions / code-pass verifications:**
- Confirm that `Duration::secs_at_least` is the right constructor (check
  fugit API in the version the devel branch is on). The philosophers
  sample uses `Duration::secs_at_least` and `Duration::millis_at_least`.
- Confirm `channel::bounded` import path is `zephyr::sync::channel::bounded`
  against the current devel tree.
- Confirm `zephyr::set_logger()` is the right logger init call (matches
  hello_world pattern).
- Confirm the `#[zephyr::thread]` macro attribute name against devel —
  it has been `#[zephyr::thread]` in the codebase but verify.
- Run under QEMU and confirm output actually appears (virtual-time burst
  is fine; silence is not).

## ex04-async — READY FOR CODE PASS

**Placement:** Module 4b in `workshop-outline.md`, immediately after
the "Async on Zephyr" lecture (Module 4a). 25 min budget.

**What attendees have at this point:**
- ex01–ex03 done. A working producer/consumer in ex03 that they built
  and modified. They know the `#[zephyr::thread]` + channel pattern.
- Module 4a heard: async refresher, why async on an RTOS, the work-queue
  executor journey, Embassy executor on a Zephyr thread, `StaticCell`
  pattern, `select!` / `join!` composability, limitations, vision.
- *Not yet:* devices (Module 5).

**The exercise in one sentence:** convert ex03's two `#[zephyr::thread]`
functions into two `#[embassy_executor::task]` async functions, keeping
identical observable behavior, then add a `select!` idle-timeout branch
to the consumer that the thread version fundamentally cannot express.

**Application structure:** standalone Zephyr application — same shape as
ex03. `CMakeLists.txt`, `prj.conf`, `Cargo.toml`, `Cargo.lock`,
`src/lib.rs`. Start from the ex03 shape as a reference but write the
files fresh; do not copy ex03 verbatim.

**prj.conf:**
```
CONFIG_RUST=y
CONFIG_RUST_ALLOC=y
CONFIG_MAIN_STACK_SIZE=4096
CONFIG_LOG=y
CONFIG_LOG_BACKEND_RTT=n
CONFIG_POLL=y
```
`CONFIG_POLL=y` is required by the Embassy executor on Zephyr.

**Cargo.toml:**
```toml
[dependencies]
zephyr = { version = "0.1.0", features = ["time-driver", "executor-zephyr"] }
log = "0.4.22"
static_cell = "2.1"
embassy-executor = { version = "0.7.0", features = ["log", "task-arena-size-2048"] }
embassy-sync = "0.6.2"
embassy-time = "0.4.0"
embassy-futures = "0.1.1"
```

`time-driver` and `executor-zephyr` on the `zephyr` crate wire the
Embassy time driver and executor backend into Zephyr's kernel.
`task-arena-size-2048` sets the Embassy task memory pool. If the arena
is too small the executor panics at spawn time — easy to diagnose and
fix by increasing the number. 2048 is sufficient for two simple tasks.
`embassy-time` is left without a `tick-hz-*` feature; the default 1 MHz
assumption involves a runtime conversion but this is irrelevant on QEMU.

**Design decisions (all settled):**

- **Channel: `embassy_sync::channel::Channel`, not `zephyr::sync::channel::bounded`.**
  The Zephyr bounded channel's `.recv()` blocks the OS thread, which
  would freeze the entire executor. `embassy_sync::channel::Channel`
  is truly async — `.receive().await` suspends only the task. It is also
  declared as a `static` constant with no alloc:
  ```rust
  static CHANNEL: Channel<CriticalSectionRawMutex, u32, 4> = Channel::new();
  ```
  This distinction is worth one sentence in the README — it is the
  concrete form of the "async composability" argument from the lecture.

- **Timer: `embassy_time::Timer::after(Duration::from_secs(1)).await`.**
  Replaces `zephyr::time::sleep(...)`. Note that `embassy_time::Duration`
  and `zephyr::time::Duration` are different types; imports must be
  explicit to avoid ambiguity.

- **Executor: `zephyr::embassy::Executor` on the main thread.**
  `executor.run()` never returns, so `rust_main` never returns either.
  No `join` needed.

- **`select!` branch: 3-second idle timeout in the consumer.**
  Uses `embassy_futures::select::{select, Either}`. Under normal
  operation (producer at 1 Hz, timeout at 3 s) the `Either::Second`
  branch never fires — but participants can verify it by temporarily
  bumping the producer sleep to 5 seconds.

- **Starter ships with boilerplate in place; participants write task bodies.**
  The executor setup, static declarations, and empty task stubs with
  `// TODO` comments are provided. Participants fill in the task bodies
  and add the `select!` branch. That is the right scope for 25 minutes.

- **Solution directory at `ex04-async/solution/`.**
  Complete working implementation for participants who get stuck.
  See also `ex03-threads/solution/` for the ex03 stretch goal
  (the `Message` enum / three-thread variant) — create that too as
  part of this code pass.

**`src/lib.rs` starter shape (what Claude Code should write and ship):**

```rust
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
    unsafe { zephyr::set_logger().unwrap(); }
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(producer()).unwrap();
        spawner.spawn(consumer()).unwrap();
    });
}

#[embassy_executor::task]
async fn producer() {
    // TODO: Loop forever, sending an incrementing u32 counter into CHANNEL
    // every 1 second using Timer::after(...).await.
    // Hint: CHANNEL.send(value).await
}

#[embassy_executor::task]
async fn consumer() {
    // TODO: Loop forever, waiting on CHANNEL.receive() OR a 3-second idle
    // timeout using select(...).await. Log which branch fired and what value
    // was received (if any).
    // Hint: match select(CHANNEL.receive(), Timer::after(...)).await { ... }
}
```

**Verify before committing:** confirm the starter builds cleanly
(`west build -b qemu_cortex_m3 .`) with the empty task bodies. The
solution in `ex04-async/solution/` must also build cleanly.

**README.md structure:**

1. **Orientation** (1 min read) — what they're doing: same producer/
   consumer as ex03, different mechanism. Two tasks on one thread instead
   of two threads. Point to the `solution/` directory for when they get
   stuck.

2. **Run the starter** — build and run. The tasks are stubs so nothing
   happens (or it panics if the stubs don't compile — confirm during
   the code pass which behavior the empty bodies produce and note it).

3. **Write the producer** (5–8 min) — fill in the `producer()` body.
   Call out: `Timer::after` returns a `Future`; `.await` is what
   actually suspends the task. Compare to ex03's `sleep()` which
   blocked the thread. Rebuild and confirm ticks appear.

4. **Write the consumer — first version** (5 min) — simple
   `CHANNEL.receive().await` loop with `info!`. Rebuild, confirm output
   matches ex03 behavior. Note that both tasks are running on the same
   thread — no OS thread switch happens between a send and its receive.

5. **Add the `select!` branch** (5–8 min) — the payoff. Replace the
   simple receive with the `select(CHANNEL.receive(), Timer::after(...))`.
   Rebuild. Then deliberately break it: bump the producer sleep to
   `Duration::from_secs(5)` and observe the idle branch firing. Restore.

6. **Compare to ex03** (2 min, discussion prompt) — how many static
   allocations did ex03 need vs. ex04? (ex03: two thread stacks at 2048
   each + two `ThreadData` structs + the channel's `Box`ed pool; ex04:
   one `StaticCell<Executor>` + the task arena at 2048 + the static
   `Channel`.) Which is more? Does it matter? When would you prefer
   threads?

7. **Stretch: two executors at different priorities** — declare a second
   Zephyr thread via `kobj_define!` + `StaticThread`, run a second
   `StaticCell<Executor>` on it at a lower priority, spawn the consumer
   there. Observe that the producer (high priority) can preempt the
   consumer between sends. This is the multi-executor priority story
   from Module 4a. This is a genuine stretch — it involves
   `kobj_define!` which hasn't been introduced yet, so provide a hint
   pointing at `modules/lang/rust/samples/embassy/src/lib.rs` for the
   pattern.

**Success criteria:** participants have a working async producer/consumer
with a `select!` idle-timeout branch, have observed the 3-second timeout
fire by slowing the producer, and can articulate the difference between
`select!` on Embassy futures vs. `k_poll` on Zephyr kernel objects.

**Open decisions / code-pass verifications:**
- Confirm `CONFIG_POLL=y` is required in prj.conf (check against the
  async-philosophers sample — it has this).
- Confirm `zephyr::embassy::Executor` is the correct import path on the
  current devel tree.
- Confirm `embassy_futures::select::{select, Either}` is the right API
  (not a `select!` macro) — the async-philosophers sample uses the
  function form, not a macro.
- Confirm the empty `async fn` stubs in the starter either compile
  cleanly or produce a clear error; note the outcome in a code comment
  so the README guidance is accurate.
- The arena-size analysis script (David has it) can be used to verify
  2048 is sufficient if there is any doubt.

## ex05-i2c — READY FOR CODE PASS

**Placement:** Module 5b in `workshop-outline.md`, immediately after
the "Devices: hard way vs better way" lecture (Module 5a). 25 min budget.

**What attendees have at this point:**
- ex01–ex04 done. Full thread + async experience.
- Module 5a heard: Zephyr C device model (`void *` vtable), the hard-way
  (raw `zephyr-sys`), its problems (type confusion, buffer lifetime,
  unchecked address, no thread-safety, C glue), the devicetree bridge
  (`build_dts()` → `zephyr::devicetree`), the better-way (typed safe
  bindings), the economic argument.
- *Not yet:* Module 6 (contributing).

**The exercise in one sentence:** read two implementations of the same
I²C read — one raw and unsafe, one using the typed bindings — find the
seeded memory-safety bug in the raw version, then articulate what the
type system is doing in the safe version.

**This is a reading exercise, not a coding exercise.** The starter ships
two complete source files. There is no blank page. The cognitive work is
analysis: finding a subtle bug and explaining type-system invariants.

---

### Board and build target

**Primary board: `myra_sip_baseboard`.**

`qemu_cortex_m3` has no I²C hardware; the better-way code uses
`zephyr::devicetree::aliases::i2c_bus::get_instance()` which requires a
board with an `i2c-bus` DTS alias. `myra_sip_baseboard` is the right
choice: Cortex-M4F (`RUST_SUPPORTED=y` via `CPU_CORTEX_M`), I²C
enabled, and its Renode platform file already has
`sht4x: I2C.SHT45 @ i2c1 0x44` wired up — enabling the optional stretch.

Build command for this exercise:
```
cd roz-workshop/ex05-i2c
west build -b myra_sip_baseboard .
```

**Reading tasks stand alone even without a successful build.** Most
participants won't have encountered `myra_sip_baseboard` before. That's
fine — the point is not the board, it's the contrast between the two
files. If the build fails for any reason, the reading tasks still work.
The README should say this explicitly.

**Board overlay** (`boards/myra_sip_baseboard.overlay`):
```dts
/ {
    aliases {
        i2c-bus = &i2c1;
    };
};
```
This wires the `i2c_bus` alias to the I²C controller that has the SHT45
on it in the Renode platform description.

**prj.conf:**
```
CONFIG_RUST=y
CONFIG_RUST_ALLOC=y
CONFIG_MAIN_STACK_SIZE=4096
CONFIG_I2C=y
CONFIG_LOG=y
CONFIG_LOG_BACKEND_RTT=n
```

**Cargo.toml:** standard — just `zephyr = "0.1.0"` and `log = "0.4.22"`.
No extra crates needed.

---

### The scenario

Both files interact with an SHT45 temperature/humidity sensor at I²C
address `0x44`. The SHT45 protocol is simple: write a one-byte command
(`0xFD` for a high-precision measurement), wait ~10 ms for conversion,
then read 6 bytes back (2 bytes temp + 1 CRC + 2 bytes humidity + 1 CRC).
This is two separate I²C transactions with a sleep between them — not a
combined write-read with a RESTART. The code examples implement this
sequence and log the raw bytes.

---

### `src/hard_way.rs` — the seeded bug

**The bug: a helper function returns a dangling pointer.**

```rust
// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

//! Hard-way I²C: raw zephyr-sys bindings.
//!
//! This file has a subtle memory-safety bug. Find it.

use zephyr::raw;

const SENSOR_ADDR: u16 = 0x44;

/// Returns a pointer to the SHT45 measurement command byte.
///
/// Called just before the I²C write.
fn measure_cmd_ptr() -> *const u8 {
    let cmd = [0xFDu8];  // high-precision measurement command
    cmd.as_ptr()         // BUG: cmd is dropped at end of this function;
                         //      the returned pointer is immediately dangling.
}

pub fn read_sensor(dev: *const raw::device) {
    // Trigger a high-precision measurement.
    //
    // Note: `unsafe` here signals that *this call* has been audited.
    // It says nothing about the validity of the pointer argument.
    let ptr = measure_cmd_ptr();
    unsafe { raw::zr_i2c_write(dev, ptr, 1, SENSOR_ADDR) };

    // Wait for the sensor to complete its measurement (~10 ms).
    unsafe { raw::k_msleep(10) };

    // Read the result: 2 bytes temp, 1 CRC, 2 bytes humidity, 1 CRC.
    let mut buf = [0u8; 6];
    unsafe { raw::zr_i2c_read(dev, buf.as_mut_ptr(), 6, SENSOR_ADDR) };

    // Use zephyr::printkln! for output even in the hard-way version.
    // raw::printk is a variadic C function that bindgen cannot expose in a
    // callable form from stable Rust, so it is not in the zephyr-sys
    // allowlist.  printkln! is available to any crate depending on `zephyr`
    // and is intentionally not hidden here — the dangerous part of the
    // hard-way path is the device-pointer handling, not the output.
    zephyr::printkln!("raw: {:02x} {:02x} {:02x} {:02x}",
        buf[0], buf[1], buf[3], buf[4]);
}
```

**Why the `unsafe` block didn't catch it:** the dangling pointer is
created in the *safe* function `measure_cmd_ptr()`. Nothing about that
function is syntactically unsafe — it takes no raw pointers in, it just
constructs a local array and extracts a pointer to it. The caller's
`unsafe` block audits the `zr_i2c_write` call itself; it cannot see
through the opaque `*const u8` to check where that pointer came from.
The borrow checker stops tracking provenance the moment a reference is
cast to a raw pointer.

**Why code review misses it:** `measure_cmd_ptr()` looks like a
reasonable abstraction — a function that returns "the command to send."
Its return type `*const u8` is unremarkable in a codebase that passes
raw pointers everywhere. The caller's `unsafe` block provides a false
sense of local correctness: "I checked that `zr_i2c_write` is safe to
call here," which is true — except for the argument it was handed.

**In safe Rust this is impossible:** `fn foo() -> &u8 { let x = 0u8; &x }`
is a compile error ("returns a reference to data owned by the current
function"). The raw pointer version of the same mistake compiles without
warning.

---

### `src/better_way.rs` — the safe version

```rust
// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

//! Better-way I²C: typed safe bindings.

use zephyr::device::i2c::I2c;
use zephyr::time::{sleep, Duration};
use log::info;

const SENSOR_ADDR: u16 = 0x44;

pub fn read_sensor(i2c: &mut I2c) {
    // Trigger a high-precision measurement.
    i2c.write(SENSOR_ADDR, &[0xFDu8]).expect("i2c write failed");

    // Wait for conversion.
    sleep(Duration::millis_at_least(10));

    // Read 6 bytes: temp (2) + CRC + humidity (2) + CRC.
    let mut buf = [0u8; 6];
    i2c.read(SENSOR_ADDR, &mut buf).expect("i2c read failed");

    info!("raw: {:02x} {:02x} {:02x} {:02x}", buf[0], buf[1], buf[3], buf[4]);
}
```

**`src/lib.rs` wiring both together** (ships as the exercise entry point):

```rust
// Copyright (c) 2026 Linaro, LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)]

mod hard_way;
mod better_way;

#[no_mangle]
extern "C" fn rust_main() {
    unsafe { zephyr::set_logger().unwrap(); }

    // --- Better way ---
    let mut i2c = zephyr::devicetree::aliases::i2c_bus::get_instance()
        .expect("i2c_bus alias not found in devicetree");
    assert!(i2c.is_ready(), "I2C bus not ready");
    better_way::read_sensor(&mut i2c);

    // --- Hard way (do not call in normal use — dangling pointer bug) ---
    // hard_way::read_sensor(/* *const raw::device */);
}
```

The hard-way call is commented out in `rust_main` so that the exercise
builds and runs the better-way path. Participants can see how they would
get the `*const device` for the hard-way version — and note that there
is no equivalent of `is_ready()` in the raw path.

---

### README.md structure

**Task 1 — Find the bug (10 min)**

Open `src/hard_way.rs`. There is one subtle memory-safety bug seeded in
it. Find it. Then answer:
- Why did the `unsafe` block at the call site not catch it?
- Why would this be easy to miss in a code review?
- What would the compiler say if you tried to write the equivalent with
  Rust references instead of raw pointers? (Try it: change
  `measure_cmd_ptr` to return `&u8` instead of `*const u8`.)

**Task 2 — Analyse the type system (10 min)**

Open `src/better_way.rs`. For each point below, find the line(s) in the
better-way code that enforce the invariant, and identify what the
hard-way version was *not* enforcing:

- **Wrong device class:** how does the type system prevent passing a UART
  where an I²C controller is expected?
- **Buffer lifetime:** how does `write` ensure the command slice is alive
  for the duration of the call?
- **Length / pointer mismatch:** how is the length of the buffers
  communicated to the driver?
- **Error handling:** what does the hard-way version do if `zr_i2c_write`
  returns an error code? What does the better-way version do?
- **Device availability:** what type does `get_instance()` return, and
  what would happen if you ignored that return value?

**Task 3 — Group discussion (5 min)**

When is reaching for raw `zephyr-sys` actually legitimate?
- A device class with no binding yet.
- Exploratory / prototype work.
- A one-off, never-reviewed internal tool.

What is the minimum discipline to do it safely? (One answer: never
extract a raw pointer from a local — always pass the slice or reference
directly into the `unsafe` call at the same scope level where the data
is alive.)

**Success criteria:** participants can point at the exact bug in the
hard-way file, explain why `unsafe` didn't prevent it, and name at least
three invariants the safe API encodes in the type system.

---

### Stretch: run it under Renode

If Renode is installed and `myra_sip_baseboard` builds cleanly:

```
west build -b myra_sip_baseboard .
west build -t run   # launches Renode with the emulated SHT45 at 0x44
```

The Renode platform file already has `sht4x: I2C.SHT45 @ i2c1 0x44`
wired up. The better-way version should produce live (emulated) sensor
readings. The hard-way version's dangling-pointer bug may or may not
manifest — that's the point. Runtime UB is not a reliable detector.

Note: `west build -t run` for Renode targets launches an interactive
Renode session, not a QEMU terminal. Exit with the Renode GUI or
`Ctrl-C`. Do NOT use `west build -t run` in non-interactive CI without
a timeout.

---

### Open decisions / code-pass verifications

**Prerequisites — must be confirmed by David before starting the code pass:**

- **`myra_sip_baseboard` + Rust buildability.** The board is STM32
  Cortex-M4 (SOC_MYRA → SOC_STM32G491XX), so `RUST_SUPPORTED` should
  be true via `CPU_CORTEX_M`, but this has not been verified end-to-end.
  David will attempt `west build -b myra_sip_baseboard
  modules/lang/rust/samples/hello_world` and confirm it succeeds before
  the ex05 code pass begins. If it fails, fall back to source-analysis-
  only: omit the board overlay, comment out the better-way call in
  `rust_main`, and note in the README that `west build` is not required
  for the reading tasks.
- **Renode availability.** Renode is expected to be installed as part of
  the Zephyr SDK. The `myra_sip_baseboard` Renode platform file already
  has `sht4x: I2C.SHT45 @ i2c1 0x44` wired up — no extra Renode config
  is needed. David will confirm `west build -t run` launches Renode for
  this board before documenting it as a stretch goal.

**Already confirmed from codebase inspection:**

- `zr_i2c_write(dev, buf_ptr, len_u32, addr_u16)` and `zr_i2c_read`
  exist in `zephyr-sys/wrapper.h` with the exact signatures used above.
  ✅
- `k_msleep` is covered by the `allowlist_function("k_.*")` wildcard
  in `zephyr-sys/build.rs` and will be available as `raw::k_msleep`.
  ✅
- `raw::printk` is NOT usable — it is variadic and not in the
  allowlist. The hard-way code uses `zephyr::printkln!` instead (see
  code above). ✅
- `zephyr::device::i2c::I2c` is the correct import path
  (`zephyr/src/device/i2c.rs` exists, `write`/`read`/`is_ready` all
  present). ✅
- `zephyr::devicetree::aliases::i2c_bus::get_instance()` is the correct
  pattern — the upstream i2c-controller test uses it verbatim. No
  application-level `build.rs` is needed; the `zephyr` crate's own
  `build.rs` calls `build_dts()`. ✅
- The board overlay only needs the aliases block (no extra pinmux lines
  — `myra_sip_baseboard` already has `i2c1` pinctrl configured in its
  DTS, unlike the tiny2040 test overlay which had to add pinmux). ✅
- The hard-way call in `rust_main` is commented out; the `hard_way`
  module is still compiled (the bug is visible to rust-analyzer and the
  type-experiment in Task 1 still works). Confirm it compiles cleanly
  with it commented out. ✅ (expected — module is compiled, not called)
- Do NOT run under Renode during the code pass (no reliable
  non-interactive exit).

## Not in scope for this brief

- Slide content (lives in the presenter's `roz-rw2026.md`).
- `docs/pre-workshop-setup.md` body — still placeholder.
- `docs/contributing-resources.md` body — still placeholder.
- Any edit to `CLAUDE.md` (see open questions).

## Open questions for David

_Currently empty for the READY FOR CODE PASS exercises (ex01, ex02).
ex03–05 design discussions are ongoing in the planning conversation
and tracked inline in each exercise's section. Log new questions here
as they surface._
