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
| 04 | ex04-async         | Embassy async rewrite of ex03  | Module 4a (Async)            | 25m   | DRAFT UNSPEC        |
| 05 | ex05-i2c           | Safe bindings vs raw sys       | Module 5a (Devices)          | 25m   | DRAFT UNSPEC        |

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

## ex04-async — DRAFT UNSPEC

Direction: rewrite ex03's two-thread design as two Embassy tasks on a
single executor, same behavior. Headline: fewer static allocations, no
stacks, `select!` composability. Fits the "tell the journey" async
narrative in the notes.

**Settled decision:** ship a prepared `src/lib.rs` starter with the
`StaticCell<Executor>` boilerplate in place. Participants focus on
converting the thread functions to `#[embassy_executor::task]` async
fns and adding the `select!` branch — not on learning the Embassy
executor wiring under time pressure. The executor setup pattern is
shown in the lecture (Module 4a) and in the starter; the exercise is
about writing async tasks, not about configuring the executor.

Still to specify: exact starter shape, `select!` branch target (idle
timeout vs. a stop signal), and whether the channel is kept from ex03
or replaced with Embassy channels. Come back once ex03 is built and
we can see what the transition naturally looks like.

## ex05-i2c — DRAFT UNSPEC

Direction: two side-by-side samples — "hard way" using raw `zephyr-sys`
with a subtle memory-safety bug to spot, and "safe way" using the
typed I²C bindings. Compile-and-review, not run-on-hardware (no I²C
device on `qemu_cortex_m3`).

Blocked on: I²C safe bindings landing fully in `devel` and the hard-way
example having concrete bindings to call. Per memory as of 2026-04-15,
lightweight bindings are merged; async bindings are more complex and
not needed for this exercise. Revisit closer to the workshop.

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
