# Project Structure

## Current Structure (Single Crate)

Ryo is a single Cargo binary crate with flat modules under `src/`. This is intentional — at this stage (~7K lines, pre-alpha) a workspace would add boilerplate without benefit.

```
ryo/
├── Cargo.toml          # Single crate, all dependencies
├── src/
│   ├── main.rs         # CLI definition (clap) and command dispatch
│   ├── pipeline.rs     # Pipeline orchestration (lex, parse, astgen, sema, codegen, link, run)
│   ├── lexer.rs        # Logos-based tokenizer; emits interned `Token` stream (StringId/i64 payloads)
│   ├── indent.rs       # CPython-style Indent/Dedent token insertion over raw lexer output
│   ├── parser.rs       # Chumsky-based parser over `Token` (produces AST)
│   ├── ast.rs          # Surface-syntax AST; identifiers/types/strings stored as `StringId`
│   ├── astgen.rs       # AST → UIR structural lowering (named after Zig's `AstGen.zig`)
│   ├── uir.rs          # Untyped IR — flat instruction stream (analogue of Zig's ZIR)
│   ├── sema.rs         # Semantic analysis: type-checks UIR, emits one TIR per function body
│   ├── tir.rs          # Typed IR — per-function-body flat instruction stream (analogue of Zig's AIR)
│   ├── types.rs        # `InternPool` for types and strings (analogue of Zig's `InternPool.zig`)
│   ├── diag.rs         # Structured diagnostics: `Diag`, `DiagCode`, `DiagSink`
│   ├── builtins.rs     # Builtin function registry (currently `print`)
│   ├── codegen.rs      # Cranelift IR generation from TIR (JIT and AOT)
│   ├── linker.rs       # Executable linking via managed Zig toolchain
│   ├── toolchain.rs    # Zig toolchain management (download, version pinning, path resolution)
│   └── errors.rs       # `CompilerError` top-level error type
├── tests/
│   └── integration_tests.rs  # End-to-end compilation and execution tests
├── examples/           # Example Ryo programs
└── docs/               # Documentation
```

### Compilation Pipeline

```
Source
  → Lexer (+ Indent preprocessor)
  → Parser           → AST
  → AstGen           → UIR   (untyped, flat, program-wide arena)
  → Sema             → TIR   (typed, flat, one per function body)
  → Codegen (Cranelift)
  → Object File
  → Linker (Zig)
  → Executable
```

Module dependencies flow left-to-right through the pipeline. `pipeline.rs` orchestrates the full chain. `main.rs` dispatches CLI commands to `pipeline.rs` entry points. The `InternPool` from `types.rs` threads through every stage so identifiers, type names, and string literals stay as `StringId` handles instead of owned `String`s.

### Key Design Decisions

- **Two-IR middle end (UIR + TIR):** The AST is first lowered to a flat untyped IR (`uir.rs`) by `astgen.rs`, then type-checked into a flat typed IR (`tir.rs`) by `sema.rs`. This mirrors Zig's ZIR/AIR split and replaces the earlier tree-shaped HIR. UIR lives in a single program-wide arena; TIR is per-function-body so future generic/comptime instantiations can `clone` a body cheaply.
- **Worklist-driven sema (`sema.rs`):** Decls transition `Unresolved → InProgress → Resolved/Failed` through a queue. Cycle detection is wired in for future inferred return types, comptime, and generics, even though today's bodies only depend on callee signatures.
- **Interned types and strings (`types.rs`):** A single `InternPool` deduplicates types and string bytes. Primitive types sit at fixed indices so hot paths never hash. `TypeId` and `StringId` are `Copy` newtypes.
- **Structured diagnostics (`diag.rs`):** Replaces ad-hoc `Result<_, String>` plumbing. `DiagSink` accumulates diagnostics so passes can continue past the first error; `DiagCode` is an enum so renderers, tests, and future LSP/JSON output can pattern-match without scraping message text.
- **Indent preprocessor (`indent.rs`):** Inserted between lexer and parser. Converts tab-based indentation into explicit `Indent`/`Dedent` tokens, following CPython's approach.
- **Managed Zig toolchain (`toolchain.rs`):** Ryo downloads and manages its own Zig installation under `~/.ryo/toolchain/`. The linker never probes the system `PATH`.
- **Builtin registry (`builtins.rs`):** Centralized registry for builtin functions (currently `print`). Keeps builtin knowledge out of the parser and codegen.

---

## Future Structure (Workspace)

When the codebase grows to ~5-10K lines or needs external consumers (LSP, formatter), the natural split is a Cargo workspace. The target is **few crates, done well** — not one crate per file.

### Recommended First Split (~10K lines)

```
ryo/
├── Cargo.toml              # Workspace definition
├── ryo/                    # CLI binary crate
│   └── src/
│       └── main.rs         # Parses args, dispatches to ryo-driver
├── ryo-core/               # Shared data structures (no logic)
│   └── src/
│       ├── lib.rs
│       ├── ast.rs          # AST node definitions
│       ├── uir.rs          # Untyped IR data structures
│       ├── tir.rs          # Typed IR data structures
│       ├── types.rs        # InternPool, TypeId, StringId
│       ├── diag.rs         # Diagnostics (Diag, DiagCode, DiagSink)
│       └── errors.rs       # CompilerError
├── ryo-frontend/           # Lexing, parsing, astgen, sema
│   └── src/
│       ├── lib.rs
│       ├── lexer.rs
│       ├── indent.rs
│       ├── parser.rs
│       ├── astgen.rs       # AST → UIR
│       ├── sema.rs         # UIR → TIR
│       └── builtins.rs
├── ryo-backend/            # Code generation and linking
│   └── src/
│       ├── lib.rs
│       ├── codegen.rs
│       ├── linker.rs
│       └── toolchain.rs
└── ryo-driver/             # Pipeline orchestration
    └── src/
        ├── lib.rs
        └── pipeline.rs
```

**Dependency graph:**
```
ryo (CLI) → ryo-driver → ryo-frontend → ryo-core
                        → ryo-backend  → ryo-core
```

### Full Workspace (~10K+ lines)

As features mature, further splits become justified:

| Crate | When to Split | Contents |
|-------|---------------|----------|
| `ryo-checker` | When borrow checking / ownership rules outgrow `sema.rs` | Borrow checker, ownership analysis (split out of `ryo-frontend`) |
| `ryo-runtime` | When heap types (str, list, map) need runtime support | Allocation, channels, task spawning, panic handling |
| `ryo-pm` | When package management is implemented | Manifest parsing, dependency resolution, registry client |
| `ryo-errors` | When error reporting grows beyond a single file | Diagnostic formatting, ariadne wrappers, source mapping |
| `ryo-lsp` | When language server is implemented | LSP protocol, completion, diagnostics, hover |

### When to Split

Split a module into its own crate when **at least two** of these are true:

1. The module has **>1K lines** and a clear API boundary
2. An **external consumer** needs it (LSP, formatter, REPL)
3. You want to **enforce a dependency boundary** that `pub(crate)` can't express
4. **Compile times** are noticeably impacted by changes to unrelated modules

Do not split preemptively. Empty crates and speculative abstractions add friction without benefit.

### Benefits of the Workspace (When Ready)

- **Hard module boundaries** — crate walls prevent accidental coupling
- **Independent testing** — `cargo test -p ryo-frontend` tests just the frontend
- **Reusable components** — an LSP can depend on `ryo-core` + `ryo-frontend` without codegen
- **Parallel compilation** — independent crates compile concurrently
