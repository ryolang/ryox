# Project Structure

## Current Structure (Single Crate)

Ryo is a single Cargo binary crate with flat modules under `src/`. This is intentional — at this stage (~2K lines, pre-alpha) a workspace would add boilerplate without benefit.

```
ryo/
├── Cargo.toml          # Single crate, all dependencies
├── src/
│   ├── main.rs         # CLI definition (clap) and command dispatch
│   ├── pipeline.rs     # Pipeline orchestration (lex, parse, lower, compile, run, build)
│   ├── lexer.rs        # Logos-based tokenizer (produces Token stream)
│   ├── indent.rs       # CPython-style Indent/Dedent token insertion
│   ├── parser.rs       # Chumsky-based parser (produces AST)
│   ├── ast.rs          # AST node definitions
│   ├── lower.rs        # AST → HIR lowering with scope resolution and type checking
│   ├── hir.rs          # High-level IR data structures (post-analysis, fully typed)
│   ├── builtins.rs     # Builtin function registry (print, future builtins)
│   ├── codegen.rs      # Cranelift IR generation from HIR (JIT and AOT)
│   ├── linker.rs       # Executable linking via managed Zig toolchain
│   ├── toolchain.rs    # Zig toolchain management (download, version pinning, path resolution)
│   └── errors.rs       # CompilerError type definitions
├── tests/
│   └── integration_tests.rs  # End-to-end compilation and execution tests
├── examples/           # Example Ryo programs
└── docs/               # Documentation
```

### Compilation Pipeline

```
Source → Lexer → Indent Preprocessor → Parser → Lower (HIR) → Codegen → Object File → Linker → Executable
```

Module dependencies flow left-to-right through the pipeline. `pipeline.rs` orchestrates the full chain. `main.rs` dispatches CLI commands to `pipeline.rs` entry points.

### Key Design Decisions

- **HIR layer (`lower.rs` + `hir.rs`):** The AST is lowered to a typed HIR before codegen. This separates parsing concerns from type resolution and scope analysis.
- **Indent preprocessor (`indent.rs`):** Inserted between lexer and parser. Converts tab-based indentation into explicit Indent/Dedent tokens, following CPython's approach.
- **Managed Zig toolchain (`toolchain.rs`):** Ryo downloads and manages its own Zig installation under `~/.ryo/toolchain/`. The linker never probes the system PATH.
- **Builtin registry (`builtins.rs`):** Centralized registry for builtin functions (currently `print`). Keeps builtin knowledge out of the parser and codegen.

---

## Future Structure (Workspace)

When the codebase grows to ~5-10K lines or needs external consumers (LSP, formatter), the natural split is a Cargo workspace. The target is **few crates, done well** — not one crate per file.

### Recommended First Split (~5K lines)

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
│       ├── hir.rs          # HIR data structures
│       ├── types.rs        # Type representations
│       └── errors.rs       # Diagnostic types
├── ryo-frontend/           # Lexing, parsing, lowering
│   └── src/
│       ├── lib.rs
│       ├── lexer.rs
│       ├── indent.rs
│       ├── parser.rs
│       ├── lower.rs
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
| `ryo-checker` | When borrow checking / ownership rules are implemented | Type checker, borrow checker, ownership analysis |
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
