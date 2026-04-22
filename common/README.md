<!--
Copyright (c) 2026 Linaro, LTD
SPDX-License-Identifier: Apache-2.0
-->

# Shared workshop tips

## The `.cargo/config.toml` symlink trick

Zephyr's Rust build generates a Cargo config template at `build/rust/sample-cargo-config.toml` during `west build`. To make `cargo check`, `cargo build`, and rust-analyzer work from an exercise directory, symlink that generated file as `.cargo/config.toml`:

```bash
mkdir -p .cargo
ln -sf ../build/rust/sample-cargo-config.toml .cargo/config.toml
```

The symlink is per-build-directory. If you do an out-of-tree build or change boards, re-link. The `.gitignore` excludes these symlinks from git so they don't accidentally get committed.

Every exercise references this page rather than repeating the instructions — if the procedure changes, update it here.
