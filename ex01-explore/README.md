<!--
Copyright (c) 2026 Linaro, LTD
SPDX-License-Identifier: Apache-2.0
-->

# Exercise 01 — Tour the Zephyr tree

A 20-minute guided reading tour. You'll put your fingers on the
files that the rest of the workshop refers back to: a board YAML,
a devicetree source, a generated header, and a Kconfig fragment.

## Before you start

This exercise is a tree tour, not a Zephyr application — there is
nothing to build in `ex01-explore/` itself. Everything you'll look
at lives elsewhere in the workspace tree (`zephyr/`, `modules/`,
`build/`). All paths below are relative to the **workspace
directory** (the dir that contains both `roz-workshop/` and
`zephyr/`).

If the pre-workshop smoke test is still fighting your toolchain,
that's OK: tasks 1 and 2 are pure reading. Do them now, and watch
a neighbor for tasks 3 and 4 until your build comes up.

## Task 1 — Find the board definition (~5 min)

Open `zephyr/boards/qemu/cortex_m3/qemu_cortex_m3.yaml`.

This is what `west build -b qemu_cortex_m3` resolves to. Things to
notice:

- `arch:` — the architecture this board targets.
- `supported:` — which device classes the board exposes (note
  `gpio`, `serial-net`, etc.).
- `toolchain:` — which compiler toolchains are accepted.
- `ram:` / `flash:` — sizes in kilobytes.

There are well over a thousand board YAMLs like this one in the
Zephyr tree. Each is a hardware story.

## Task 2 — Find the console UART in devicetree (~5 min)

Open `zephyr/boards/qemu/cortex_m3/qemu_cortex_m3.dts`. Two things
to find:

1. The `chosen { ... }` block. Note `zephyr,console = &uart0;` —
   Zephyr resolves "the console" to whatever node `zephyr,console`
   points at.
2. The include chain. This file `#include`s `<ti/lm3s6965.dtsi>`,
   which resolves to `zephyr/dts/arm/ti/lm3s6965.dtsi`. Open that
   file and find the `uart0` node. What's its `compatible` string?

You should land at `compatible = "ti,stellaris-uart"`. That string
is what binds a Zephyr UART driver to this hardware. The same
"chosen + compatible + driver" pattern shows up for every device
class — Module 5 builds on it.

## Task 3 — Rebuild hello_world for qemu_riscv32 (~5 min)

Same Rust source, different ISA. From the workspace directory:

```
cd modules/lang/rust/samples/hello_world
west build -p -b qemu_riscv32 .
```

`-p` means *pristine* — required when you change boards in an
existing build directory. Without it you'd get a board-mismatch
error against the pre-workshop smoke-test build.

If you have time, `west build -t run` and confirm RISC-V Rust on
Zephyr boots from the same source you built for ARM. (`Ctrl-a x`
to exit QEMU.)

What changed under `build/` versus the ARM build? Don't read
everything — just notice that essentially the entire generated
tree is re-derived from the new board.

## Task 4 — Skim a generated header (~5 min)

Still inside `modules/lang/rust/samples/hello_world`, open

```
build/zephyr/include/generated/zephyr/devicetree_generated.h
```

This is what Zephyr's devicetree compiles down to from C's point
of view: thousands of lines of preprocessor-macro spaghetti. You
are *not* expected to read it deeply. Things to notice:

- Almost every line is a `#define`.
- There is no `struct` for "the UART" — devicetree facts are
  encoded as macro names that other macros chain on.
- This is the "DT macro soup" Module 1a referred to. It's also
  the thing `zephyr-build`'s `build_dts()` parses to generate the
  typed Rust mirror under `zephyr::devicetree::...` — Module 5
  explains that side of it.

For comparison, the Kconfig-to-C output sits next door at

```
build/zephyr/include/generated/zephyr/autoconf.h
```

— every `CONFIG_FOO` your build saw, expanded as `#define`.

## Stretch — Flip a Kconfig and watch it cascade (5 min)

If you have time, see how a one-line `prj.conf` change flowers
into the full generated config.

```
# Save the current .config so you can diff later.
cp build/zephyr/.config /tmp/before.config
```

Edit `prj.conf` and add a line:

```
CONFIG_LOG_DEFAULT_LEVEL=4
```

Rebuild (no `-p` needed — same board, just a config change):

```
west build -b qemu_riscv32 .
```

Diff:

```
diff /tmp/before.config build/zephyr/.config
```

You'll see your `LOG_DEFAULT_LEVEL` line plus any cascading
defaults the new value pulled in. That cascade is why Kconfig is
a system, not just a key/value file.

When you're done, undo the `prj.conf` edit — it's a tracked file
in the upstream `zephyr-lang-rust` checkout. From this same
directory, `git diff prj.conf` shows your change and `git checkout
prj.conf` discards it.

## Success criteria

By the end of this exercise you should be able to point at the
actual file when someone says:

- "Where is the board defined?"
- "Where does the console UART come from?"
- "Where does Kconfig land in the build tree?"

And you should be collecting questions about Kconfig and DT —
Modules 2 and 5 are where those answers live.
