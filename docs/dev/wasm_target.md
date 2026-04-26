# WASM Target Implementation Plan (Draft — v0.2)

**Status:** Design (v0.2+)

This document describes the plan for adding a WebAssembly compilation target to the Ryo compiler at the current stage of development (pre-Milestone 27, before the concurrency runtime exists). It is the actionable counterpart to the long-term WasmFX discussion in [concurrency.md](concurrency.md#future-wasm-target-via-wasmfx).

The plan is intentionally scoped to what is achievable **today** with Cranelift's stable WASM backend and WASI preview 1, and explicitly defers everything that depends on stack switching, threads, or async I/O.

---

## Goals

1. `ryo build --target wasm32-wasip1 <file.ryo>` produces a `.wasm` module that runs under `wasmtime`, `wasmer`, and Node.js (via WASI shim).
2. All synchronous Ryo programs through Milestone 27 (Core Language Complete) compile and run on WASM with identical observable behavior to native targets.
3. The implementation does not regress native AOT or JIT pipelines.
4. The target is gated behind a feature flag until validated, so an incomplete WASM backend cannot break `cargo test` for native users.

## Non-Goals (current stage)

- Browser target (`wasm32-unknown-unknown` with JS host). Tracked separately; needs a binding strategy.
- WASI preview 2 / component model. Wait for stabilization.
- Concurrency, threads, async I/O. Blocked on WasmFX + WASI 0.3 — see [concurrency.md](concurrency.md).
- JIT execution of WASM (`ryo run --target wasm…`). AOT only.
- Source-level debugging (DWARF-in-WASM). Out of scope for v0.2.

---

## Current Compiler State (relevant facts)

- Backend: **Cranelift 0.130** (`src/codegen.rs`, ~625 LOC). IR is largely target-agnostic.
- Linker: `zig cc` driver (`src/linker.rs`, `src/toolchain.rs`). Zig ships `wasm-ld` and a `wasm32-wasi` libc out of the box.
- Pipeline: lex → parse → AST → semantic → UIR → TIR → Cranelift IR → object → link (`src/pipeline.rs`).
- No runtime crate exists yet — the compiled program currently links against system libc only via Zig.
- Concurrency runtime (Phase 5, Milestones 32–34) is **not yet implemented**, so there is no green-thread code to port. This is the easiest possible moment to add WASM support.

---

## Architecture

### Target Selection

Add a `--target` flag to `ryo build` parsed via `target-lexicon` (already a dependency). Recognized values for v0.2:

| Triple | Status |
|---|---|
| `<host>` (default) | Existing native AOT path |
| `wasm32-wasip1` | New — primary WASM target |
| `wasm32-unknown-unknown` | Stretch — emits a `.wasm` with no syscalls; useful for pure-compute libraries |

Internally, plumb a `TargetSpec` struct from `main.rs` → `pipeline.rs` → `codegen.rs` → `linker.rs`. Replace any implicit host-triple usage in `codegen.rs` with the resolved `TargetSpec`.

### Codegen

Cranelift already supports `wasm32` as an ISA target. Concretely:

1. In `codegen.rs`, replace `cranelift_native::builder()` with a triple-aware builder that returns the `wasm32` ISA when targeting WASM.
2. Pointer width becomes 32 bits. Replace any hardcoded `types::I64` for pointer-sized values with `module.target_config().pointer_type()`. Audit:
   - struct field offsets in `tir.rs` and `uir.rs`
   - any `usize`/`isize` lowering in `sema.rs` / `types.rs`
3. Calling convention: Cranelift emits the standard WASM CC automatically. No ABI work needed for the v0.2 type set.
4. Object output: switch from `cranelift-object` (ELF/Mach-O/COFF) to **`cranelift-object` with the WASM target** — Cranelift produces a relocatable WASM object that `wasm-ld` consumes. Verify this against the current Cranelift version; if WASM object emission lags, fall back to writing a single-module `.wasm` directly via `cranelift-wasm` helpers.

### Linker

Replace `zig cc` with `zig wasm-ld` (or invoke `wasm-ld` directly if Zig's bundled copy is reachable):

```
zig wasm-ld -o out.wasm input.o \
    --no-entry --export=_start \
    -L<zig-lib>/wasi/libc -lc
```

Add a `Linker::link_wasm` path in `src/linker.rs` parallel to the existing native linker. The Zig toolchain manager (`src/toolchain.rs`) already downloads Zig, so no new toolchain dependency is introduced.

### Runtime / Stdlib Shims

Today's compiled programs use almost no runtime. The required shims for v0.2 WASM:

| Need | Implementation |
|---|---|
| Program entry | Emit `_start` (WASI convention) instead of `main`. Map Ryo's top-level / `main` to `_start`. |
| `panic` / abort | Lower panics to a call into a tiny WASI wrapper that writes the message to fd 2 and calls `proc_exit(1)`. |
| Allocator | WASM has no system allocator. Bundle a minimal `dlmalloc`-style allocator as a Ryo-side static, OR reuse the one Zig links in via `wasi-libc`. Prefer the latter — zero new code. |
| stdout / print | When stdlib `print` lands (Milestone 24), route it to `fd_write` via `wasi-libc`'s `write(2)`. Identical source-level API. |
| File I/O | Same — goes through wasi-libc; transparent. |
| Time / random | wasi-libc forwards to WASI `clock_time_get` and `random_get`. Transparent. |

Net: **no new runtime code** for v0.2 if we lean on wasi-libc through Zig.

### Type System Adjustments

| Type | Native | WASM | Action |
|---|---|---|---|
| `int` (default) | i64 | i64 | unchanged — WASM has native i64 |
| `uint` | u64 | u64 | unchanged |
| `f32` / `f64` | as-is | as-is | unchanged |
| `i8` / `u8` / `i16` / `u16` | native | stored as i32, narrowed at boundaries | Cranelift handles automatically; verify bool layout |
| `i128` / `u128` | software | software (slower) | acceptable, document |
| pointer-sized | 64-bit | 32-bit | **Audit required** — see Codegen step 2 |
| `str` / slice fat pointers | `(ptr: u64, len: u64)` | `(ptr: u32, len: u32)` | layout already abstracted via `pointer_type()` if step 2 done correctly |

### CLI Surface

```
ryo build --target wasm32-wasip1 hello.ryo            # → hello.wasm
ryo build --target wasm32-wasip1 --release hello.ryo
ryo target list                                       # show available targets
```

`ryo run` against a WASM target is **not** implemented in v0.2. Users invoke `wasmtime out.wasm` themselves. A future `ryo run --target wasm…` could shell out to `wasmtime` if it is on `PATH`.

---

## Features That Will Not Work on WASM

This list MUST be reflected in user-facing docs (`docs/installation.md` or a new `docs/targets.md`) when the target ships. It mirrors and extends the table in [concurrency.md](concurrency.md#what-is-explicitly-out-of-scope).

### Hard Blocks (no path forward without new WASM proposals)

| Feature | Reason | Unblocks when |
|---|---|---|
| `task.run`, `future[T]`, `.await` | Requires stack switching; standard WASM has no accessible execution stack | WasmFX (Phase 4) reaches stable runtimes |
| Channels (`chan[T]`) | Built on the task runtime | Same as above |
| `task.delay` / timers | Need scheduler suspension points | Same as above |
| Real OS threads / parallelism | Needs `wasi-threads` or `SharedArrayBuffer` | Host-dependent |
| `Mutex` / `RwLock` with blocking | Single-threaded WASM cannot block usefully | Threaded WASM |
| Stack-overflow recovery as a Ryo error | WASM has no guard pages; host traps the module | WasmFX or host trap handlers |

### Soft Blocks (degraded but possible)

| Feature | Status on WASM |
|---|---|
| Networking (`std.net`) | Not in WASI preview 1. Available in preview 2 (`wasi:sockets`); revisit when the component model stabilizes. |
| Subprocess / `std.process.spawn` | Not in WASI. Will return `Unsupported` at runtime. |
| Filesystem | Works, but limited to **preopened** directories (WASI sandbox). No raw `/`-rooted access. |
| Environment variables | Read-only, host-controlled. |
| Panics with stack traces | No DWARF; only message + `proc_exit`. Backtraces deferred. |
| `unsafe` raw pointers | Pointers are 32-bit indices into linear memory; FFI to native libraries is impossible. WASM imports replace C FFI but need a separate binding model (out of scope v0.2). |
| 128-bit ints | Software-emulated, slower. |
| SIMD | Requires the WASM SIMD proposal; emit only when `--features +simd128` is set. v0.2: disabled. |

### Compile-Time Detection

Programs that use unavailable features should fail **at compile time** with a clear diagnostic, not at runtime. Mechanism (design only — implementation deferred):

- Add a `target` predicate usable in `cfg`-style attributes (mirroring Rust's `#[cfg(target_family = "wasm")]`). The exact syntax is a separate spec proposal.
- The stdlib marks symbols like `task.run`, `std.net.*`, `std.process.spawn` with a `#[unavailable(target = "wasm32-*")]` attribute. The semantic analyzer rejects calls to such symbols when the active target matches.
- Until the attribute system exists, document the gaps in `docs/targets.md` and rely on link-time errors (missing WASI imports) as a backstop.

---

## Phased Implementation

Each phase ends in a green CI run and a demo. Estimates assume the same ~8 hr/week pace as the main roadmap.

### Phase A — Plumbing (1 week)

- Add `--target` CLI flag, `TargetSpec` struct, target-aware ISA builder.
- Audit pointer-width assumptions; replace with `pointer_type()`.
- Feature-gate the WASM path behind `cargo build --features wasm` so partial work cannot break native CI.
- **Demo:** `ryo build --target wasm32-wasip1 trivial.ryo` produces a WASM object file (not yet linked).

### Phase B — Linking & Hello Exit Code (1 week)

- Implement `Linker::link_wasm` invoking `wasm-ld` via the Zig toolchain.
- Map top-level / `main` to `_start`; emit a tiny WASI exit wrapper.
- **Demo:** `wasmtime hello.wasm; echo $?` returns the value of a Ryo program that evaluates to an integer (Milestone 3 parity, on WASM).

### Phase C — Core Language Coverage (1–2 weeks)

- Run the existing Milestone 4–14 integration tests through the WASM target. Fix any 32-bit pointer or i32/i64 narrowing bugs surfaced.
- Add a `cargo test --features wasm` job that runs the WASM suite under `wasmtime` (CI: install `wasmtime` via release tarball, ~3 MB).
- **Demo:** factorial, fizzbuzz, struct/enum examples from `docs/examples/` all run identically on native and WASM.

### Phase D — Stdlib Plumbing (concurrent with Milestone 24)

- When `print`, file I/O, and core stdlib symbols land, ensure their wasi-libc routing is correct.
- Document gaps in `docs/targets.md`.
- **Demo:** `hello_world.ryo` prints "Hello, world!" under `wasmtime --dir=.`.

### Phase E — Polish

- `ryo run --target wasm32-wasip1` shells out to `wasmtime` if available.
- `ryo target list` / `ryo target add` (matches the proposal in [proposals.md](proposals.md)).
- Size optimization pass: `--release` invokes `wasm-opt` (from `binaryen`) if present.

### Out of Phase (deferred)

- Browser target (needs JS bindings, no `_start`, no WASI).
- Concurrency on WASM — covered by the WasmFX section of [concurrency.md](concurrency.md).
- WASI preview 2 / component model.
- DWARF debug info and source-mapped backtraces.

---

## Risks & Open Questions

1. **Cranelift WASM object emission maturity.** `cranelift-object` targeting `wasm32` may not be a fully supported configuration in 0.130. Mitigation: spike during Phase A; if blocked, fall back to single-module emission via `cranelift-wasm`.
2. **Pointer-width audit completeness.** Any silently-baked 64-bit assumption in `tir.rs` / `uir.rs` will produce subtly wrong code. Mitigation: a focused review pass plus running every existing integration test under the WASM target before declaring Phase C done.
3. **Zig bundled `wasm-ld` invocation surface.** Verify that the Zig version pinned by `src/toolchain.rs` exposes `zig wasm-ld` cleanly. If not, document a `wasm-ld` system dependency and fall back gracefully.
4. **Allocator coupling.** Pulling in wasi-libc's `malloc` is convenient but ties the binary size floor to ~30 KB. If size becomes a concern, swap in a small Rust-side allocator.
5. **Future concurrency divergence.** When the green-thread runtime lands (M32+), a WASM build must produce a clear "feature not available on this target" error rather than silently linking nothing. Phase A's feature-gating gives us the hook to enforce this.

---

## References

- Spec: §1 (Target Domains, mentions Wasm), §17 (Implementation Strategy — Cranelift WebAssembly support)
- Dev: [concurrency.md](concurrency.md#future-wasm-target-via-wasmfx) (WasmFX deferral), [compilation_pipeline.md](compilation_pipeline.md), [proposals.md](proposals.md) (cross-compilation CLI)
- Milestone: Slot between Milestones 27 (Core Language Complete) and 28+; adds no language features. To be linked from `implementation_roadmap.md` once approved.
- External: [Cranelift WASM docs](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift), [WASI preview 1 spec](https://github.com/WebAssembly/WASI/tree/main/legacy/preview1), [wasi-libc](https://github.com/WebAssembly/wasi-libc)
