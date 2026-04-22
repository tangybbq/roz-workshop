<!--
Copyright (c) 2026 Linaro, LTD
SPDX-License-Identifier: Apache-2.0
-->

# Rust on Zephyr — RustWeek 2026 workshop

A five-hour, hands-on workshop for experienced Rust developers who are new to embedded / RTOS programming. We use [Zephyr RTOS](https://zephyrproject.org/) together with [`zephyr-lang-rust`](https://github.com/zephyrproject-rtos/zephyr-lang-rust) to build small Rust applications that run under QEMU — no physical hardware required. Each exercise stands alone, so you can skip ahead if you get stuck.

Presented by **David Brown** ([@tangybbq](https://github.com/tangybbq)) at **[RustWeek 2026](https://rustweek.org/)**, Utrecht, 2026-05-18.

## Prerequisites

Before the workshop, install:

- A recent [Zephyr SDK](https://docs.zephyrproject.org/latest/develop/getting_started/index.html#install-the-zephyr-sdk) (toolchains, QEMU, etc.).
- [`west`](https://docs.zephyrproject.org/latest/develop/west/index.html) and its Python dependencies (a `pip install -r zephyr/scripts/requirements.txt` after `west update`, as described in Zephyr's getting-started guide).
- [`rustup`](https://rustup.rs/) with the `thumbv7m-none-eabi` target installed:
  ```
  rustup target add thumbv7m-none-eabi
  ```
  A few exercises also exercise `qemu_riscv32`; if you want to follow those, also add `riscv32imac-unknown-none-elf`.

See [`docs/pre-workshop-setup.md`](docs/pre-workshop-setup.md) for the detailed checklist.

## Quick start

This repo uses Zephyr's **T2 (star) topology**: the repo is the west manifest project, and the west workspace is the directory that *contains* the repo. After `west update`, `zephyr/`, `modules/`, `bootloader/`, `tools/`, and `.west/` land as siblings of `roz-workshop/` — not inside it.

```sh
mkdir roz-ws && cd roz-ws
git clone https://github.com/tangybbq/roz-workshop.git
west init -l roz-workshop
west update
# Optional — we recommend skipping this if you use other Zephyr trees:
# west zephyr-export

# Smoke test: build and run the upstream Rust hello-world sample under QEMU.
west build -b qemu_cortex_m3 modules/lang/rust/samples/hello_world
west build -t run
```

You should see the hello-world output under QEMU. Press `Ctrl-a x` to exit QEMU.

## Repo layout

```
roz-ws/                        ← your workspace directory
├── .west/                     ← created by `west init` (ignored by roz-workshop)
├── zephyr/                    ← pulled by `west update`, pinned to v4.1.0
├── modules/                   ← Zephyr modules, including lang/rust on upstream `devel`
├── bootloader/
├── tools/
└── roz-workshop/              ← this repo
    ├── west.yml               ← the manifest (pins everything)
    ├── ex01-explore/          ← exercise directories — each is a Zephyr application
    ├── ex02-rust-analyzer/
    ├── ex03-threads/
    ├── ex04-async/
    ├── ex05-i2c/
    ├── common/                ← shared tips (e.g. the cargo-config symlink trick)
    ├── docs/                  ← pre-workshop setup, contributing resources
    └── README.md
```

## Running an exercise

From the workspace dir (the parent of `roz-workshop/`):

```sh
west build -b qemu_cortex_m3 roz-workshop/ex03-threads
west build -t run
```

Each exercise's `README.md` notes any deviations (different board, extra Kconfig, etc.). For rust-analyzer and `cargo check` support inside an exercise directory, see [`common/README.md`](common/README.md).

## License

Apache-2.0 — see [`LICENSE`](LICENSE).
