<!--
Copyright (c) 2026 Linaro, LTD
SPDX-License-Identifier: Apache-2.0
-->

# Workshop Repo Scaffold — Instructions for Claude Code

## Context

You are scaffolding a GitHub repo for a 5-hour workshop titled **Rust on Zephyr** at RustWeek 2026 (Utrecht, 2026-05-18).

- **Audience:** experienced Rust developers new to embedded / RTOS.
- **Target repo location:** `github.com/tangybbq/roz-workshop` (confirm the name with the operator; the working default is `roz-workshop`).
- **License:** Apache-2.0.
- **Hardware assumption:** QEMU only — no real boards. Primary target `qemu_cortex_m3`; some exercises also exercise `qemu_riscv32`.

This is the **scaffold pass only**. You are creating the skeleton that subsequent per-exercise instruction docs will fill in. Do **not** write exercise code in this pass.

## Repo pattern: freestanding application (T2 topology)

The repo is a **freestanding application** — Zephyr's "T2: star topology, application is the manifest repo" pattern. The repo is the manifest project; the **west workspace is its parent directory**. Participants make a workspace dir, clone the repo into it, then init and update from the workspace:

```
mkdir roz-ws && cd roz-ws
git clone https://github.com/tangybbq/roz-workshop.git
west init -l roz-workshop
west update
```

After `west update`, Zephyr and its modules land as **siblings** of `roz-workshop/` — `roz-ws/zephyr/`, `roz-ws/modules/`, `roz-ws/.west/`, etc. Nothing west-managed lives inside the repo, so the in-repo `.gitignore` does **not** need to list `/zephyr/`, `/modules/`, `/bootloader/`, `/tools/`, or `/.west/`. Exercise directories live inside the repo as Zephyr applications.

(Equivalent invocation if you've already `cd`-ed into the repo: `west init -l .` — it still creates `.west/` in the parent.)

## Prerequisites you can assume

- Zephyr v4.1.0 is released (April 2026) and is the pin target.
- `github.com/zephyrproject-rtos/zephyr-lang-rust` is the upstream Rust language module, and its `devel` branch is what the workshop pins. The only repo on `tangybbq` is `roz-workshop` itself.
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
  mkdir roz-ws && cd roz-ws
  git clone https://github.com/tangybbq/roz-workshop.git
  west init -l roz-workshop
  west update
  west zephyr-export   # optional; we recommend against for multi-tree users
  # Verify:
  west build -b qemu_cortex_m3 modules/lang/rust/samples/hello_world
  west build -t run
  ```
  (Note: the upstream recommendation from the workshop notes is to skip `west zephyr-export` — mention it, but flag it as optional.)
- **Repo layout** — a short tree showing `ex01-explore/` … `ex05-i2c/`, `docs/`, `common/`, `west.yml`. Explain that `west update` populates `zephyr/`, `modules/`, `bootloader/`, `tools/`, and `.west/` **alongside** the repo (inside the workspace dir that contains it), not inside the repo itself — so there's nothing west-managed to gitignore at the repo root.
- **How to run an exercise** — e.g., `west build -b qemu_cortex_m3 ex03-threads && west build -t run`.
- **Pre-workshop setup:** point at `docs/pre-workshop-setup.md`.
- **License notice:** Apache-2.0.
- **Credit:** presented oy David Brown (tangybbq) at RustWeek 2026.

### 2. `LICENSE` — full Apache-2.0 text

Use the canonical Apache-2.0 text (the full license body, not just the short header). Don't substitute or abbreviate.

### 3. `west.yml` — west manifest

Goals:
- Pin Zephyr to tag `v4.1.0`.
- Override `zephyr-lang-rust` from upstream `zephyrproject-rtos`, branch `devel`, path `modules/lang/rust` (replacing the SHA-pinned entry that ships in Zephyr's `submanifests/optional.yaml`).
- Keep the rest of Zephyr's module imports intact.

Starting point (adjust for correctness):

```yaml
manifest:
  remotes:
    - name: upstream
      url-base: https://github.com/zephyrproject-rtos

  defaults:
    remote: upstream

  projects:
    - name: zephyr
      revision: v4.1.0
      import:
        name-blocklist:
          - zephyr-lang-rust

    - name: zephyr-lang-rust
      revision: devel
      path: modules/lang/rust
```

**Why the `name-blocklist`:** Zephyr v4.1.0 declares `zephyr-lang-rust` in `submanifests/optional.yaml` pinned to a SHA, in the `optional` group (filtered out by Zephyr's own `group-filter`). If we just re-declare it at the top level, west sees a name collision. Blocking it from the import lets our entry stand alone — and because our entry has no `groups:`, it's pulled by default, so participants don't have to flip a `project-filter` config.

**Verify:** `west manifest --resolve` must parse cleanly. After `west update`, run `west list` and confirm `modules/lang/rust` points at `zephyrproject-rtos/zephyr-lang-rust` on `devel` (not at the SHA from `submanifests/optional.yaml`).

**Do not pin to a SHA yet.** The operator will bump the `zephyr-lang-rust` revision to a fixed SHA about one week before the workshop (target date 2026-05-18) for day-of reproducibility. Until then, `devel` is intentional.

### 4. `.gitignore`

Exclude, at minimum:

```
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

Because the west workspace is the **parent** of this repo (T2 topology), the `.west/`, `zephyr/`, `modules/`, `bootloader/`, `tools/` directories are *siblings* of the repo, not children — so they don't need to be in this `.gitignore`. If the layout ever changes to a workspace-application pattern (repo root = workspace), add them back.

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

- Apache-2.0 license header on **every file** you create — source, READMEs, yaml, `.gitignore`, etc. The `LICENSE` file itself is the exception (it is the license body). Use the comment syntax appropriate to the file type:
  - C / C++ / Rust:
    ```
    // Copyright (c) 2026 Linaro, LTD
    // SPDX-License-Identifier: Apache-2.0
    ```
  - YAML / shell / Kconfig / Python / `.gitignore`:
    ```
    # Copyright (c) 2026 Linaro, LTD
    # SPDX-License-Identifier: Apache-2.0
    ```
  - Markdown (HTML comment at the very top, before the first heading):
    ```
    <!--
    Copyright (c) 2026 Linaro, LTD
    SPDX-License-Identifier: Apache-2.0
    -->
    ```
- No GitHub Actions. The operator does not want a CI workflow on this repo.
- No `Makefile` or `justfile` in this pass unless you're confident it helps. Can be added later.

## Verification

After scaffolding, confirm (paths are relative to the workspace dir — the parent of the repo):

1. `west init -l roz-workshop` (or `west init -l .` from inside the repo) runs without error.
2. `west update` completes and produces:
   - `../zephyr/` at workspace root, pinned to `v4.1.0` (check `git -C ../zephyr describe`).
   - `../modules/lang/rust/` on the upstream `zephyrproject-rtos/zephyr-lang-rust` `devel` branch (check `git -C ../modules/lang/rust remote -v` and `git -C ../modules/lang/rust log -1`).
3. `west build -b qemu_cortex_m3 ../modules/lang/rust/samples/hello_world` succeeds.
4. `west build -t run` prints the hello-world output under QEMU.
5. `git status` inside the repo is clean (no west-produced directories appear there — they are siblings).
6. `ls ex0*` shows five exercise directories each with a placeholder `README.md`.
7. First commit is tidy — one commit for the scaffold is fine.

Report each of these back to the operator in your summary, with pass/fail for each check.

## Open questions — surface, don't decide

Bubble these up to the operator rather than committing:

1. **Final repo name.** Working default is `roz-workshop`. If the operator prefers something else (e.g., `rustweek-2026-roz`, `rust-on-zephyr-workshop`), rename before first push.
2. **Should `Cargo.lock` files be committed?** Default recommendation: yes. Flag for the operator.
3. **`west zephyr-export`** — the workshop notes recommend against it for multi-tree users; the quick-start includes it as optional. Confirm the framing is what the operator wants.

## What you're NOT doing in this pass

- No exercise content.
- No per-exercise `Cargo.toml`, `CMakeLists.txt`, `prj.conf`, or `src/` files.
- No `pre-workshop-setup.md` content (placeholder only).
- No `contributing-resources.md` content (placeholder only).
- No slide-related work.

Those land in subsequent instruction docs. Leave clean, obvious placeholders so the shape of the next pass is visible.
