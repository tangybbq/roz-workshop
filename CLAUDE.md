# Workshop Repo Scaffold — Instructions for Claude Code

## Context

You are scaffolding a GitHub repo for a 5-hour workshop titled **Rust on Zephyr** at RustWeek 2026 (Utrecht, 2026-05-18).

- **Audience:** experienced Rust developers new to embedded / RTOS.
- **Target repo location:** `github.com/tangybbq/roz-workshop` (confirm the name with the operator; the working default is `roz-workshop`).
- **License:** Apache-2.0.
- **Hardware assumption:** QEMU only — no real boards. Primary target `qemu_cortex_m3`; some exercises also exercise `qemu_riscv32`.

This is the **scaffold pass only**. You are creating the skeleton that subsequent per-exercise instruction docs will fill in. Do **not** write exercise code in this pass.

## Repo pattern: self-contained Zephyr workspace-application

The repo is a **workspace-application**: the repo root itself is the west workspace. Participants clone the repo, then:

```
cd roz-workshop
west init -l .
west update
```

After `west update`, Zephyr and modules live as siblings/subdirs inside the repo root, managed by west and excluded from git via `.gitignore`. Exercise directories live inside the repo as Zephyr applications.

This pattern is documented by Zephyr as "application as workspace manifest repository." Use it.

## Prerequisites you can assume

- Zephyr v4.1.0 is released (April 2026) and is the pin target.
- `github.com/tangybbq/zephyr-lang-rust` exists with a `devel` branch the operator controls.
- You're running in a fresh, empty directory that will become the repo root.
- `west`, `cmake`, a Zephyr SDK, and `rustup` with cross-compilation support are available on your host if you want to verify the build.

## Deliverables (scaffold pass)

Create the following at the repo root:

### 1. `README.md` — top-level orientation

Should contain:
- One-paragraph workshop description (feel free to lift phrasing from `workshop-outline.md` in the presenter's notes, though you won't have access to it — you can write from this brief).
- **Prerequisites:** Zephyr SDK, `west`, Python, `rustup` with the appropriate targets (thumbv7m-none-eabi for Cortex-M3).
- **Quick start** section:
  ```
  git clone https://github.com/tangybbq/roz-workshop.git
  cd roz-workshop
  west init -l .
  west update
  west zephyr-export   # optional; we recommend against for multi-tree users
  # Verify:
  west build -b qemu_cortex_m3 modules/lang/rust/samples/hello_world
  west build -t run
  ```
  (Note: the upstream recommendation from the workshop notes is to skip `west zephyr-export` — mention it, but flag it as optional.)
- **Repo layout** — a short tree showing `ex01-explore/` … `ex05-i2c/`, `docs/`, `common/`, `west.yml`, and pointing out that `zephyr/` and `modules/` appear after `west update` and are gitignored.
- **How to run an exercise** — e.g., `west build -b qemu_cortex_m3 ex03-threads && west build -t run`.
- **Pre-workshop setup:** point at `docs/pre-workshop-setup.md`.
- **License notice:** Apache-2.0.
- **Credit:** presented by David Brown (tangybbq) at RustWeek 2026.

### 2. `LICENSE` — full Apache-2.0 text

Use the canonical Apache-2.0 text (the full license body, not just the short header). Don't substitute or abbreviate.

### 3. `west.yml` — west manifest

Goals:
- Pin Zephyr to tag `v4.1.0`.
- Point `zephyr-lang-rust` at `github.com/tangybbq/zephyr-lang-rust`, branch `devel`, path `modules/lang/rust`.
- Keep the rest of Zephyr's module imports intact.

Starting point (adjust for correctness):

```yaml
manifest:
  remotes:
    - name: upstream
      url-base: https://github.com/zephyrproject-rtos
    - name: tangybbq
      url-base: https://github.com/tangybbq

  projects:
    - name: zephyr
      remote: upstream
      revision: v4.1.0
      import: true

    - name: zephyr-lang-rust
      remote: tangybbq
      revision: devel
      path: modules/lang/rust
```

**Verify:** `west manifest --resolve` must parse cleanly. If Zephyr's default manifest imports `zephyr-lang-rust` by default (vs. excluding it via project-filter — the behavior has historically been "disabled by default, enabled via `west config manifest.project-filter +zephyr-lang-rust`"), and your explicit entry above conflicts, use `import.name-blocklist` on the Zephyr project to drop the default entry before re-adding your fork. Confirm behavior by running `west list` after `west update` and making sure the `modules/lang/rust` path points at the `tangybbq` fork on `devel`, not upstream.

**Do not pin to a SHA yet.** The operator will bump the `zephyr-lang-rust` revision to a fixed SHA about one week before the workshop (target date 2026-05-18) for day-of reproducibility. Until then, `devel` is intentional.

### 4. `.gitignore`

Exclude, at minimum:

```
# Zephyr workspace — pulled down by west update, not checked in
/.west/
/zephyr/
/modules/
/bootloader/
/tools/
/test/

# Build artifacts
build/
build-*/

# Cargo
target/

# Per-exercise cargo config — symlinked at build time
**/.cargo/config.toml

# Editor / OS junk
.DS_Store
.vscode/
.idea/
```

Adjust paths once you observe what `west update` actually produces.

**Decision to flag:** whether to commit `Cargo.lock` files inside exercise directories. Recommendation: **yes**, for reproducibility across participants. Don't ignore them.

### 5. Exercise directories

Create these directories, each with a placeholder `README.md`:

```
ex01-explore/
ex02-rust-analyzer/
ex03-threads/
ex04-async/
ex05-i2c/
```

Each placeholder README reads:

```markdown
# Exercise NN: <short name>

Content lands in a later instruction doc. This placeholder exists so the directory is tracked in git and the scaffold is complete.
```

Replace `NN` and `<short name>` appropriately for each directory.

### 6. `docs/` directory

Create `docs/` containing placeholder files:

- `docs/pre-workshop-setup.md` — placeholder `# Pre-workshop setup\n\n_Populated by a later instruction pass._`
- `docs/contributing-resources.md` — placeholder similarly.

### 7. `common/` directory

Create `common/README.md` explaining the `.cargo/config.toml` symlink trick that every exercise will reference:

> Zephyr's Rust build generates a Cargo config template at `build/rust/sample-cargo-config.toml` during `west build`. To make `cargo check`, `cargo build`, and rust-analyzer work from an exercise directory, symlink that generated file as `.cargo/config.toml`:
>
> ```bash
> mkdir -p .cargo
> ln -sf ../build/rust/sample-cargo-config.toml .cargo/config.toml
> ```
>
> This symlink is per-build-directory. If you do an out-of-tree build or change boards, re-link. The `.gitignore` excludes these symlinks from git.

This is content worth writing once and referencing everywhere, so participants don't get this explained to them five times.

## Technical constraints

- Apache-2.0 license header on every source file you create in this scaffold pass (there shouldn't be many; maybe none). Header format:
  ```
  // Copyright (c) 2026 David Brown
  // SPDX-License-Identifier: Apache-2.0
  ```
- No GitHub Actions in this pass. If you think a CI workflow that builds `hello_world` on push would add value, flag it as a suggestion — don't add it unprompted.
- No `Makefile` or `justfile` in this pass unless you're confident it helps. Can be added later.

## Verification

After scaffolding, confirm:

1. `west init -l .` runs without error in the repo root.
2. `west update` completes and produces:
   - `zephyr/` at workspace root, pinned to `v4.1.0` (check `git -C zephyr describe`).
   - `modules/lang/rust/` at the `tangybbq/zephyr-lang-rust` `devel` branch (check `git -C modules/lang/rust remote -v` and `git -C modules/lang/rust log -1`).
3. `west build -b qemu_cortex_m3 modules/lang/rust/samples/hello_world` succeeds.
4. `west build -t run` prints the hello-world output under QEMU.
5. `git status` is clean (all `west`-produced directories ignored).
6. `ls ex0*` shows five exercise directories each with a placeholder `README.md`.
7. First commit is tidy — one commit for the scaffold is fine.

Report each of these back to the operator in your summary, with pass/fail for each check.

## Open questions — surface, don't decide

Bubble these up to the operator rather than committing:

1. **Final repo name.** Working default is `roz-workshop`. If the operator prefers something else (e.g., `rustweek-2026-roz`, `rust-on-zephyr-workshop`), rename before first push.
2. **Exact copyright attribution.** Default above uses "David Brown" — confirm.
3. **Should `Cargo.lock` files be committed?** Default recommendation: yes. Flag for the operator.
4. **GitHub Actions CI?** Not in scope for this pass; flag as a suggestion for later if you think it'd add value.
5. **`west zephyr-export`** — the workshop notes recommend against it for multi-tree users; the quick-start includes it as optional. Confirm the framing is what the operator wants.

## What you're NOT doing in this pass

- No exercise content.
- No per-exercise `Cargo.toml`, `CMakeLists.txt`, `prj.conf`, or `src/` files.
- No `pre-workshop-setup.md` content (placeholder only).
- No `contributing-resources.md` content (placeholder only).
- No slide-related work.

Those land in subsequent instruction docs. Leave clean, obvious placeholders so the shape of the next pass is visible.
