
Here's a potential development plan focusing on iterative progress, delivering tangible results early, and aligning with Ryo's core goals. Each milestone aims for roughly **2-4 weeks** assuming ~8 hours/week, but be flexible – the goal is steady progress, not rigid deadlines.

**Guiding Principles:**

* **Iterate:** Get *something* working end-to-end quickly, then refine.
* **Test Early, Test Often:** Integrate basic testing from the start.
* **Focus:** Each milestone adds a specific, visible capability.
* **Simplicity First:** Implement the simplest version that meets the immediate goal.
* **Quality of Life:** Don't skip documentation (for yourself!), basic error reporting, and simple tooling.

**Phase 1: The Core Foundation (Getting Something Running)**

* **Milestone 1: Setup & Lexer Basics**
    * **Goal:** Parse basic Ryo syntax into tokens.
    * **Tasks:**
        * Set up Rust project (`cargo new ryo_compiler`).
        * Add dependencies (`logos`, `chumsky`, `clap`).
        * Define core tokens (`Token` enum in `src/token.rs`) using `logos`: keywords (`def`, `if`, `else`, `return`, `mut`, `struct`, `enum`, `match`, `async`, `await`), identifiers, integer literals, basic operators (`=`, `+`, `-`, `*`, `/`, `:`), punctuation (`(`, `)`, `{`, `}`). Handle whitespace/comments.
        * Write basic tests for the lexer.
        * Create a simple CLI harness (`src/main.rs`) using `clap` that takes a filename and prints the tokens.
    * **Visible Progress:** `ryo lex <file.ryo>` prints token stream.

* **Milestone 2: Parser & AST Basics**
    * **Goal:** Parse simple variable declarations and integer literals into an Abstract Syntax Tree (AST).
    * **Tasks:**
        * Define basic AST nodes (`struct Program`, `struct Statement`, `enum StmtKind::VarDecl`, `struct Expression`, `enum ExprKind::Literal`, `struct Ident`, `struct TypeExpr`) in `src/ast.rs`, including spans (`chumsky::SimpleSpan`).
        * Implement parser using `chumsky` (`src/parser.rs`): Parse `ident: type = literal_int`. Focus only on this specific structure.
        * Integrate parser with the lexer output in `main.rs`.
        * Update CLI: `ryo parse <file.ryo>` prints the generated AST (using `Debug` derive).
        * Write basic parser tests.
    * **Visible Progress:** `ryo parse <file.ryo>` shows the structure of simple variable declarations.

* **Milestone 3: "Hello, Exit Code!" (Cranelift Integration)**
    * **Goal:** Compile a minimal Ryo program (e.g., `exit_code: int = 42`) to native code that returns an exit code.
    * **Tasks:**
        * Add `cranelift`, `cranelift-module`, `cranelift-jit` (or `cranelift-object`) dependencies.
        * Create a basic code generation module (`src/codegen.rs`).
        * Implement logic to translate the simple `VarDecl` AST into Cranelift IR (focus on constants/loading values).
        * Generate code for a main function that loads the value and returns it.
        * Use `cranelift-object` to write an object file OR `cranelift-jit` to execute directly.
        * Update CLI: `ryo run <file.ryo>` compiles and runs the code (check exit code `$?`).
    * **Visible Progress:** `ryo run my_program.ryo` executes and exits with the specified code. **Major morale boost!**

**Phase 2: Building Essential Language Features**

* **Milestone 4: Functions (`def`) & Basic Calls**
    * **Goal:** Define and call simple functions with integer arguments and return values.
    * **Tasks:**
        * Extend AST: `StmtKind::FunctionDef`, `ExprKind::Call`.
        * Extend Parser: Parse `def func_name(arg: type) -> type: ... return expr`. Parse function calls `func_name(arg)`.
        * Extend Codegen: Generate Cranelift IR for function definitions and calls (basic calling convention). Handle `return` statements.
        * Write tests for function definition and calls.
    * **Visible Progress:** Can compile and run programs using simple functions.

* **Milestone 5: Expressions & Operators (+, -, *)**
    * **Goal:** Support basic integer arithmetic expressions.
    * **Tasks:**
        * Extend AST: `ExprKind::BinaryOp`.
        * Extend Parser: Handle operator precedence for `+`, `-`, `*`.
        * Extend Codegen: Generate Cranelift IR for arithmetic operations.
        * Write tests for expressions.
    * **Visible Progress:** `ryo run` can execute programs like `result: int = (2 + 3) * 5`.

* **Milestone 6: Control Flow (`if`) & Booleans**
    * **Goal:** Implement `if/else` statements and boolean logic.
    * **Tasks:**
        * Extend Lexer/Parser/AST: `Token::KwIf/Else`, `Token::Bool`, comparison operators (`==`, `!=`, `<`, etc.), logical `and`/`or`/`not`, `StmtKind::IfStmt`, `ExprKind::Literal(Bool)`, boolean binary ops.
        * Extend Codegen: Generate Cranelift IR for conditional branching and boolean operations.
        * Write tests for `if/else` and boolean expressions.
    * **Visible Progress:** Can run programs with basic conditional logic.

* **Milestone 7: Basic `Result` Enum & Error Reporting**
    * **Goal:** Introduce `Result<T, E>` for error handling (no `?` yet) and improve compiler error messages.
    * **Tasks:**
        * Define a built-in `Result` enum concept in the type system (even if hardcoded initially).
        * Allow functions to return `Result[int, SomeErrorType]`.
        * **Crucially:** Integrate `ariadne` (or similar) for basic syntax/type error reporting using spans from AST/Tokens.
        * Focus on making errors from previous steps (parsing, basic type mismatches) user-friendly.
    * **Visible Progress:** Better error messages! Can define functions returning `Result`.

**Phase 3: Ownership, Interop, and Usability**

* **Milestone 8: Basic Ownership (Move/Copy) & `String` Type**
    * **Goal:** Implement move semantics for a heap-allocated `String` type and `Copy` for `int`. Introduce basic borrow checking concepts.
    * **Tasks:**
        * Define a simple `String` type (needs runtime support for allocation - keep minimal).
        * Implement basic type checking/semantic analysis pass (`src/checker.rs`?).
        * Track variable states (valid, moved). Implement `Copy` trait concept for primitives.
        * Issue errors on use-after-move.
        * No complex lifetimes yet. Focus on simple variable assignments and function calls.
    * **Visible Progress:** Compiler prevents basic use-after-move errors.

* **Milestone 9: Immutable Borrows (`&`)**
    * **Goal:** Allow passing immutable references (`&String`) to functions.
    * **Tasks:**
        * Extend type system/checker: Handle `&T` type.
        * Implement borrow checking rule: Cannot mutate through an immutable borrow.
        * Extend Codegen: Pass pointers for borrows.
    * **Visible Progress:** Can pass strings by reference safely; compiler prevents mutation via `&`.

* **Milestone 10: Basic Collections (`List<int>`) & `for` loop**
    * **Goal:** Implement a simple `List` (for `int` initially) and `for i in range(N):` loops.
    * **Tasks:**
        * Define a basic `List` type (runtime allocation needed). Add `append` method.
        * Extend Parser/AST/Checker/Codegen for `for` loops (simple range only).
        * *Decision:* Hardcode for `List<int>` first *or* implement very basic generics needed for `List<T>`. Hardcoding is simpler initially.
    * **Visible Progress:** Can write simple loops and use dynamic lists.

* **Milestone 11: `mut`, Mutable Borrows (`&mut`), `?` Operator**
    * **Goal:** Implement mutable variables, mutable borrows, and the `?` operator for `Result`.
    * **Tasks:**
        * Extend Parser/AST/Checker: Handle `mut` keyword.
        * Extend Checker: Implement mutable borrow rules (no aliasable mutable borrows).
        * Extend Codegen: Handle loading/storing mutable variables, passing mutable pointers.
        * Extend Parser/Checker/Codegen: Implement `?` operator logic (check for `Err`, return early).
    * **Visible Progress:** Core ownership model complete! `Result` ergonomics improved.

**Phase 4: Concurrency, Interop, Jupyter (MVP)**

* **Milestone 12: Basic `async`/`await` (Single-Threaded)**
    * **Goal:** Implement `async def` and `await` for basic futures, running on a simple single-threaded executor.
    * **Tasks:**
        * Extend Parser/AST/Checker/Codegen: Handle `async`/`await` keywords. Transform async functions (e.g., into state machines or using Rust's `async` blocks internally).
        * Integrate a minimal executor (e.g., Rust's `futures::executor::block_on` or `LocalPool`) into the runtime.
        * Implement a simple `async.sleep()` function.
    * **Visible Progress:** Can define and run simple non-blocking async functions.

* **Milestone 13: Basic Rust/C FFI (`extern`)**
    * **Goal:** Allow Ryo to call a simple external C/Rust function.
    * **Tasks:**
        * Extend Parser/AST/Checker: Handle `extern "C" fn ...` declarations.
        * Extend Codegen: Generate calls using the C ABI.
        * Write a simple C or Rust library with a function (e.g., `void say_hello()`), link against it.
    * **Visible Progress:** Ryo programs can call external native code.

* **Milestone 14: Basic Jupyter Kernel (Non-JIT)**
    * **Goal:** Create a kernel that allows running Ryo code cells in Jupyter, *even if inefficiently*.
    * **Tasks:**
        * Learn Jupyter Messaging Protocol basics. Use a Rust crate like `jupyter-rs` (check its status).
        * Create a new binary (`ryo_kernel`).
        * Implement kernel logic: Receive code cell, save to temp file, invoke `ryo run <temp_file>`, capture stdout/stderr/exit code, send back results.
        * Implement `ryo kernel install` command to register the kernel.
        * *Limitation:* State is *not* preserved between cells with this simple approach.
    * **Visible Progress:** Can run Ryo code in Jupyter Notebooks! Foundational step.

**Beyond:**

* Python Interop (pyo3/maturin inspiration)
* Full Generics & Traits (`#[derive]`)
* LSP Implementation
* Package Manager (`Ryo.toml`, registry?)
* Standard Library Expansion (HTTP, JSON, Filesystem)
* JIT Compilation for Jupyter Kernel
* Debugger Support
* Advanced Borrow Checker Rules/Lifetime Inference
