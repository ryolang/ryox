# Known Issues

Compiler issues identified during source review. Each entry is independently actionable; severity reflects impact on correctness, future feature work, or code health — not user impact today (the compiler is pre-alpha).

Resolved entries are removed (not kept around as a changelog). Look at `git log` if you need history.

---

## Severity Legend

- 🔴 **Blocking** — prevents implementing roadmap features as currently designed.
- 🟡 **Correctness/Hygiene** — silent bug or invariant gap; works today, will bite later.
- 🟢 **Cleanup** — code health, ergonomics, minor.

---

## 🔴 Blocking

### I-003 — No control flow in the compiler
**Files:** `src/lexer.rs`, `src/parser.rs`, `src/hir.rs`, `src/codegen.rs`
**Summary:** `if`/`else`/`match` are lexed as keywords but have no parser, no HIR variants, and no codegen support (no block branching or phi handling). This is the single largest blocker for advancing past Milestone 4.
**Resolution:** Add `HirStmt::If` / `HirExpr::If` (block-expression form), parser productions, and Cranelift multi-block emission.

### I-020 — `inst_values` memoizer will break under multi-block control flow
**Files:** `src/codegen.rs` (`inst_values: HashMap<InstRef, Value>`)
**Summary:** Codegen lazily memoizes Cranelift `Value`s keyed by `InstRef`. This is sound today because every function has a single `entry_block` and UIR is tree-shaped. Once `if`/`loop` (I-003) introduce multiple basic blocks, an `InstRef` evaluated in one block and re-used from another will trigger Cranelift dominator errors: the memoized `Value` was emitted into a block that does not dominate the consumer.
**Resolution:** Before control flow lands, switch to per-block eager evaluation (likely via a TIR pass that anchors each instruction to its parent block, as gestured at in the docs). Drop the cross-block memoization, or scope it to the current block only and re-materialize / phi-merge across block boundaries.

### I-004 — String type is a raw pointer with no length
**Files:** `src/codegen.rs` (`HirExprKind::StrLiteral`, `generate_print_call`)
**Summary:** `StrLiteral` codegen returns a bare `global_value` pointer. There is no length, no slice ABI, no ownership metadata. `print()` works only on string literals because it grabs the length from the AST node, not from the runtime value. Any non-literal string operation will require a fat-pointer (`*u8, usize`) ABI decision.
**Resolution:** Design and implement a string slice ABI before adding string operations. Reference: Zig's `[]const u8`.

---

## 🟡 Correctness / Hygiene

### I-005 — `mut` is parsed but never enforced
**Files:** `src/hir.rs` (`HirStmt::VarDecl.mutable`), `src/sema.rs`
**Summary:** `HirStmt::VarDecl` carries `mutable: bool`, but no pass reads it. The README advertises immutable-by-default semantics. Reassignment isn't parsed yet, so the bug is latent — but the invariant should be checked in sema as soon as assignment lands.
**Resolution:** When assignment is added to the parser, sema must reject reassignment to non-`mut` bindings.

### I-006 — `print` is special-cased in codegen
**Files:** `src/codegen.rs` (`generate_print_call`), `src/sema.rs` (`check_builtin_call`)
**Summary:** Codegen emits a raw `write(2)` syscall wrapper inline for the `print` builtin, and sema has a builtin-specific validator hook to match. Consequences:
- Rejects `print(some_var)` even when the variable is a string.
- No formatting, no automatic newline.
- Already stubbed out on Windows (`return Err(...)`).
- Mixes runtime concerns into the compiler.
**Resolution:** Move `print` to a runtime crate (`ryort` or similar) compiled to an object file and linked in via `zig cc`. Codegen emits a normal call; `sema::check_builtin_call` goes away.

### I-009 — `FunctionContext` rebuilt per HIR statement
**Files:** `src/codegen.rs` (`compile_function`)
**Summary:** A fresh `FunctionContext` struct is constructed for every `HirStmt` inside the loop. Harmless functionally but noisy and obscures intent.
**Resolution:** Lift the `FunctionContext` above the loop, or restructure so `eval_expr` takes `&mut self` directly. Mostly a borrow-checker exercise.

### I-010 — Unused `_bytes_written` from `write(2)` call
**Files:** `src/codegen.rs` (`generate_print_call`)
**Summary:** The result of the `write` syscall is fetched and ignored. A short write or `EINTR` will silently truncate output. Acceptable for a bootstrap, but should be documented or fixed when `print` moves to the runtime (I-006).
**Resolution:** Tracked under I-006.

### I-014 — Lexer errors bypass `DiagSink`
**Files:** `src/lexer.rs` (`LexError`), `src/pipeline.rs` (`parse_source`, `display_tokens`)
**Summary:** Phase 1 of `docs/dev/pipeline_alignment.md` routes parse / ast_lower / sema errors through a `DiagSink` so analysis can continue past the first failure. The lexer was on a parallel branch at the time and still uses a single-shot `Result<_, LexError>`. The driver wraps a `LexError` into a one-element `Vec<Diag>` at the boundary, but problems like "invalid integer literal" or "unknown escape sequence" can't co-surface with a sema error in the same run.
**Resolution:** Thread `&mut DiagSink` into `lexer::lex` and `intern_token`. Replace the early return on the first lex error with `sink.emit(...)` followed by a recovery token (e.g. `Token::Error` for the bad span) so the parser still sees a well-formed stream. Eliminate `LexError` once the migration is complete.

### I-015 — Unknown escape sequences silently preserved
**Files:** `src/lexer.rs` (`unescape`)
**Summary:** When `unescape` encounters `\q` (or any other character not in the small known table) it preserves the backslash and the character verbatim instead of reporting a diagnostic. The user gets no feedback that the escape sequence is unrecognised, and the runtime string contains the literal `\q` bytes. Tracked as a TODO at the function definition.
**Resolution:** Folded in with I-014: once `lexer::lex` has a `&mut DiagSink`, the `Some(c)` arm of `unescape` emits a structured `Diag::error(span, DiagCode::UnknownEscape, …)` and proceeds with the raw character (or skips both bytes — TBD by spec discussion).

### I-016 — Indent processor errors carry no span
**Files:** `src/indent.rs` (`process`), `src/lexer.rs` (`lex` fallback)
**Summary:** `indent::process` returns `Result<_, String>` with a free-form message and no source location. The driver currently fakes a span by reusing the last raw token's span. That's "near" the offending newline but not on it; for "spaces are not allowed" / "dedent doesn't match an outer level" the user wants the squiggle on the indentation itself.
**Resolution:** Have `indent::process` return a richer error type carrying the offending span (the `Newline(s)` token whose whitespace failed validation, or the `Dedent` insertion point) and propagate it into `LexError.span`. The TODO at the lexer fallback covers this from the consumer side.

### I-017 — `i64::MIN` integer literal is unrepresentable
**Files:** `src/lexer.rs` (`RawToken::Int` arm)
**Summary:** Integer literals are parsed as `i64` at lex time, then sign is applied later via the unary `-` operator. That makes `-9_223_372_036_854_775_808` (i.e. `i64::MIN`) unspellable: the positive form `9_223_372_036_854_775_808` overflows `i64`. Hits the negation-overflow corner Rust itself fixed via `IntLit` / `IntLitMin` token-level distinction.
**Resolution:** Either parse as `u64` and resolve negation+overflow at sema time, or add an `IntLitMin` token variant that the parser recognises only as the operand of unary `-`. Coordinate with the broader numeric-tower design before picking either.

---

## 🟢 Cleanup

### I-011 — Manual error enum where `thiserror` would suffice
**Files:** `src/errors.rs` (33 lines)
**Summary:** Hand-rolled `enum CompilerError` with manual `Display` and `From<io::Error>` impls. `thiserror` would cut ~20 lines and make variants more uniform.
**Resolution:** Add `thiserror`, derive `Error` and `Display`, drop the hand-written impls.

### I-012 — `pretty_print` lives on AST nodes
**Files:** `src/ast.rs`
**Summary:** Presentation logic (tree-drawing, prefixes) is mixed into the AST data types. Convenient now, painful as the AST grows. A separate `pretty` module — or `Debug`-derived JSON output for `--parse` — scales better and keeps `ast.rs` focused on data.
**Resolution:** Extract into `src/ast_pretty.rs` (or similar) when the next AST node type is added.

### I-013 — `--emit` flag surface is fragmented across subcommands
**Files:** `src/main.rs`, `src/pipeline.rs`
**Summary:** `lex`, `parse`, `ir` are separate subcommands. Each stage already exists and is wired up; users would benefit from a single `ryo build --emit=tokens|ast|hir|clif|obj` surface (mirroring `zig build-exe -femit-…`).
**Resolution:** Unify under one subcommand with an `--emit` flag. Keep current subcommands as deprecated aliases for one release.

### I-018 — `TypeId` is a newtype, not a typed enum
**Files:** `src/types.rs` (`TypeId`)
**Summary:** Phase 2 §2.2 of `docs/dev/pipeline_alignment.md` originally called for `TypeId` to become an `enum { Void = 0, Bool = 1, ..., Error = 4, Dynamic(NonZeroU32) }` so primitive matches are exhaustive at compile time and the `pool.int()` accessor disappears. The risk register allowed a fallback to a plain `Copy` newtype if the enum encoding fights the borrow checker, which is what we shipped. Cost: the `TypeKind::Tuple` arm we added in `cranelift_type_for` and a couple of sema sites are not statically guaranteed to be covered when a new primitive lands.
**Resolution:** Re-attempt the enum encoding using `repr(u32)` + `Dynamic(NonZeroU32)` once the borrow-checker pain points (mostly around `pool.kind` returning a value that contains a `TypeId`) are characterised. Low priority — the matches we have today still go through `TypeKind`, which *is* exhaustive, so the gap is small.

### I-019 — `tuple_elements_vec` allocates a `Vec` per call
**Files:** `src/types.rs` (`tuple_elements_vec`)
**Summary:** The accessor copies the element-id slice out of `extra` rather than returning a borrowed view, because `TypeId` is not `#[repr(transparent)]` over `u32` and the unsafe transmute to `&[TypeId]` would be UB without it. Today the function is called only by `Display` for diagnostics and by tests; not a hot path.
**Resolution:** Tag `TypeId` with `#[repr(transparent)]` and expose `tuple_elements(id) -> &[TypeId]` alongside the copying accessor. Migrate non-perf-critical callers to it lazily. Defer until tuple codegen lands and the accessor shows up in a profile.

---

### I-021 — `bool` lowered as `types::I8` will mis-ABI across FFI boundaries
**Files:** `src/codegen.rs` (`cranelift_type_for`)
**Summary:** `TypeKind::Bool` maps to Cranelift `I8`. Fine for internal logic, but C ABIs typically pass `_Bool` zero/sign-extended to a full register (often i32 on SysV, register-width on Win64). Passing or returning our raw `I8` across an FFI call would leave the upper bits undefined from the callee's perspective.
**Resolution:** When FFI lands, insert explicit `uext` (zero-extension) on bool arguments at call sites and `ireduce` on bool returns, per the target ABI. Decide at the FFI design stage whether `bool` keeps its `I8` storage type and only widens at the boundary, or becomes register-width throughout. Latent until FFI exists.

### I-022 — String equality not implemented
**Files:** `src/sema.rs` (`check_binary_op`)
**Summary:** `==` / `!=` on `str` is rejected with `"not supported for type 'str' (yet)"`. Blocked on I-004: with strings represented as bare pointers, there is no length to compare against. Fixing this requires the fat-pointer ABI from I-004 plus a `memcmp` libcall (or an inlined byte-compare loop) at codegen.
**Resolution:** After I-004 lands the `(*u8, usize)` slice ABI, implement `==`/`!=` in codegen as length-check + `memcmp`. Sema just needs the rejection removed.

---

## Cross-References

- Roadmap: [docs/dev/implementation_roadmap.md](docs/dev/implementation_roadmap.md)
- Spec: [docs/specification.md](docs/specification.md)
- Phase plan: [docs/dev/pipeline_alignment.md](docs/dev/pipeline_alignment.md)
