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
| 03 | ex03-threads       | Threads + channel ticker       | Module 3a (Threads + sync)   | 25m   | DRAFT UNSPEC        |
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

## ex03-threads — DRAFT UNSPEC

Direction, not spec:

- Start with the **single-threaded uptime ticker**: one `rust_main()`
  that uses `zephyr::timers::Timer` on a 1s period, `zephyr::time::Instant`
  for elapsed, and `log::info!` for output (with logging init per the
  hello_world pattern). Runs for N iterations then returns.
- **Then extend it** to a two-thread structure: a producer thread ticks
  and sends uptime samples through a bounded `zephyr::sync::channel`; a
  consumer thread receives and logs. Threads are declared via
  `#[zephyr::thread]`.
- This is the first exercise where the QEMU virtual-time caveat bites.
  Include the aside.

Not yet specified: exact API choices (SpinMutex vs Mutex vs channel
only; whether to use the ReadyThread `start()` dance or just drop-and-go;
pool vs singleton threads). Come back after ex01/ex02 land and we've
seen what the notes actually say about ex03.

## ex04-async — DRAFT UNSPEC

Direction: rewrite ex03's two-thread design as two Embassy tasks on a
single executor, same behavior. Headline: fewer static allocations, no
stacks, `select!` composability. Fits the "tell the journey" async
narrative in the notes.

Blocked-on-decision: whether attendees write the executor boilerplate
by hand (`StaticCell<Executor>`, spawn pattern from the notes) or
whether we ship a prepared `main.rs` with the boilerplate in place and
they just add tasks.

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
