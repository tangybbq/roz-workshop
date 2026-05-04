<!--
Copyright (c) 2026 Linaro, LTD
SPDX-License-Identifier: Apache-2.0
-->

# Exercise 05 — I²C: hard way vs better way

25 minutes. This is a **reading exercise, not a coding exercise.** Two
files implement the same I²C read of an SHT45 temperature/humidity
sensor at address `0x44`. One uses raw `zephyr-sys` bindings and has a
seeded memory-safety bug; the other uses `zephyr::device::i2c::I2c`.
Your job is to find the bug, then articulate what the type system in
the safe version is enforcing that the raw version is not.

The reading tasks stand alone — you do not need a runtime to do them.
This is a **build-only target**: there is no QEMU board with a working
I²C peripheral that pairs an SHT45-equivalent emulator, and the Renode
STM32 I²C v2 model is incompatible with the Zephyr v2 driver in a way
that hangs the bus. We compile against a real board so the typed
`zephyr::device::i2c::I2c` bindings and the devicetree alias resolve
through real-driver code paths; we do not run it under emulation.

## Build

```
cd roz-workshop/ex05-i2c
west build -b nrf52840dk/nrf52840 .
```

`nrf52840dk/nrf52840` is the chosen build target because:

- it has a Cortex-M4 (no thumbv6m bindgen wrinkle on macOS);
- its `i2c0` controller uses the `nordic,nrf-twi` compatible, which is
  already in the upstream `modules/lang/rust/dt-rust.yaml`, so no
  per-app DT augmentation is needed;
- the upstream board DTS already has `i2c0` pinned and `status = "okay"`
  — `boards/nrf52840dk_nrf52840.overlay` only adds the `i2c-bus` alias
  that the safe binding looks up.

Re-apply the symlink recipe from
[`../common/README.md`](../common/README.md) once for rust-analyzer.

## Task 1 — Find the bug (10 min)

Open [`src/hard_way.rs`](src/hard_way.rs). There is one subtle
memory-safety bug seeded in it. Find it. Then answer:

- **Why didn't the `unsafe` block at the call site catch it?** What
  exactly is `unsafe` claiming about that line?
- **Why would this be easy to miss in code review?** What does
  `measure_cmd_ptr` look like to a reader who is not specifically
  hunting for lifetime bugs?
- **What would the compiler say if you tried to write the equivalent
  with Rust references?** Try it: change `measure_cmd_ptr`'s return
  type from `*const u8` to `&u8` and adjust the body to return `&cmd[0]`.
  Build and read the error.

The point is not just to find the bug — it is to feel the asymmetry
between what the borrow checker enforces on references and what it
silently permits on raw pointers.

## Task 2 — Analyse the type system (10 min)

Open [`src/better_way.rs`](src/better_way.rs). For each invariant
below, point at the line(s) that enforce it in the safe API, then
state what the hard-way version was *not* enforcing:

1. **Wrong device class.** The hard-way `read_sensor` takes a
   `*const raw::device`. What stops you from passing in a UART device
   pointer? In the better-way version, what is the type of the first
   argument, and where does that type come from?

2. **Buffer lifetime.** How does `i2c.write(SENSOR_ADDR, &[0xFDu8])`
   guarantee the command byte is alive for the duration of the call?
   What in the function signature (`fn write(&mut self, addr: u16,
   buf: &[u8]) -> Result<()>`) makes the dangling-pointer bug from
   Task 1 impossible to express?

3. **Length / pointer mismatch.** In the raw version, length is a
   separate `u32` argument that the caller must keep in sync with the
   buffer. In the safe version, where does the length come from? What
   class of bug does this eliminate?

4. **Error handling.** What does the hard-way version do if
   `zr_i2c_write` returns an error code? (Look closely — the
   `unsafe { … }` expression has a return value.) What does the
   better-way version do?

5. **Device availability.** What type does
   `zephyr::devicetree::aliases::i2c_bus::get_instance()` return? What
   would happen if you wrote `let i2c = … .get_instance();` and used
   `i2c` directly? (Hint: the same kind of mistake is impossible
   because of how `Option` is encoded.) The hard-way path has no
   equivalent of `i2c.is_ready()` either — what does that mean for
   start-up ordering?

## Task 3 — Group discussion (5 min)

Raw `zephyr-sys` is available for a reason. With your neighbour:

- **When is reaching for it actually legitimate?** A device class with
  no safe binding yet; exploratory or prototype work; a one-off
  internal tool.
- **What is the minimum discipline to do it safely?** One concrete
  answer: never extract a raw pointer from a local — always pass the
  slice or reference *directly* into the `unsafe` call at the same
  scope where the data lives. The Task 1 bug is impossible to write
  that way.
- **Where does the safe-binding cost get paid?** Once, by whoever
  writes the binding; after that, every caller benefits. That is the
  economic argument from the lecture.

## Success criteria

You can:

- point at the exact line of the bug in `src/hard_way.rs` and explain
  why `unsafe` did not prevent it,
- name at least three invariants the safe API encodes in the type
  system, and
- explain when reaching for raw `zephyr-sys` is actually justified —
  and when it is laziness.

## Why no `west build -t run`?

Short answer: the workshop couldn't find an emulation target that
combines (a) Rust support, (b) a working I²C controller model, and
(c) an emulated peripheral at `0x44` to talk to.

- `qemu_cortex_m3` has no I²C controller at all.
- `myra_sip_baseboard` (STM32G4) has an SHT45 wired up in its Renode
  platform, but Renode's `STM32F7_I2C` model rejects the Zephyr v2
  driver's CR2 setup write (`Changing NBYTES when START is set is not
  permitted`), then fires TXIS continuously without advancing the
  transfer. The driver walks `data->current.buf` past the end of
  flash forever; nothing reaches the console.
- `tiny2040` (RP2040) has a known-working I²C model in Renode but no
  SHT45 in the platform description, and on macOS its Cortex-M0+
  target trips a bindgen issue against Apple's `arm_acle.h`.

So this exercise stops at compilation. That's deliberate — the
learning is in the contrast between the two source files, not in
seeing bytes come back from a sensor model. If you bring real
hardware to the workshop (any board with an SHT4x on an
already-supported I²C controller), the safe path will run unchanged.

## What just happened

You read two implementations of the same trivial sensor read. The raw
version had a bug that compiled cleanly, passes a casual review, and
produces undefined behaviour at runtime. The safe version cannot
express that bug. The cost of the safe version is paid once — in
[`zephyr/src/device/i2c.rs`](../../modules/lang/rust/zephyr/src/device/i2c.rs)
— and amortised across every application that uses it.

Module 6 looks at how to contribute new safe bindings to
`zephyr-lang-rust` when a device class you need does not have one yet.
