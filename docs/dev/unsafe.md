This is a classic language design dilemma: **How to allow dangerous things (which are necessary) without letting the average user shoot themselves in the foot.**

Since Ryo is written in Rust, we can leverage the compiler to act as a strict **Gatekeeper**.

We will solve this using a **Capability-Based System**. `unsafe` exists in the language syntax, but the compiler **bans it by default** unless the project explicitly declares itself as a "System Binding Package."

Here is the architectural proposal:

---

### 1. The Architecture: "App" vs. "Binding" Packages

We split the ecosystem into two tiers of users:

1.  **Application Developers (99%):** Write web servers, CLI tools, scripts.
    *   **Rule:** If they type `unsafe`, the compiler throws a **Hard Error**: *"Unsafe operations are forbidden in application code."*
2.  **Binding Maintainers (1%):** Write wrappers for C libraries (SQLite, TileLang, GTK).
    *   **Rule:** They must explicitly opt-in via configuration.

### 2. How to Implement it (The "Gatekeeper")

#### Step A: The Configuration (`ryo.toml`)
In the package manifest, we add a specific flag that changes the compiler mode for that specific package.

**For a Normal App (User):**
```toml
[package]
name = "my_web_server"
version = "0.1.0"
# 'kind' defaults to "application"
```

**For a Binding Library (Maintainer):**
```toml
[package]
name = "sqlite-sys"
version = "1.0.0"
kind = "system"  # <--- THE MAGIC SWITCH
```

#### Step B: The Compiler Check (In Rust)
In your Rust compiler (specifically in the `SemanticAnalyzer` or `Checker` pass), you add a simple check when traversing the AST.

```rust
// src/checker.rs

fn check_unsafe_block(&self, block: &UnsafeBlock, config: &PackageConfig) {
    if config.kind != PackageKind::System {
        self.errors.push(Error::new(
            block.span,
            "Forbidden Unsafe: You cannot use 'unsafe' blocks in an Application package.",
            "Hint: Use a pre-existing library or set 'kind = \"system\"' in ryo.toml if you are writing a C binding."
        ));
    }
    // If kind == System, allow it.
}
```

### 3. The `unsafe` Syntax (Hidden but Standard)

Even though normal users can't use it, you must define the syntax for the library authors. It should look familiar (Rust/Go style) but be distinct enough to warn the maintainer.

```ryo
# src/lib.ryo in a 'system' package

# 1. Define the C function
extern "C":
    fn malloc(size: usize) -> *void
    fn free(ptr: *void)

# 2. The Wrapper (Standard Ryo)
pub struct Buffer:
    ptr: *void

# 3. The Unsafe Implementation
impl Buffer:
    fn new(size: int) -> Buffer:
        unsafe:  # Allowed ONLY because ryo.toml says kind="system"
            p = malloc(size)
            return Buffer(ptr=p)

    fn drop(&mut self):
        unsafe:
            free(self.ptr)
```

### 4. Why this solves the problem?

1.  **DX Protection:** A new user trying Ryo cannot accidentally write memory-unsafe code. The compiler stops them immediately.
2.  **Ecosystem Clarity:** If a user sees a package with `kind = "system"`, they know: *"This package contains dangerous code and interacts with C."*
3.  **No "Super-User" Syntax:** We don't need separate file extensions (`.ryo` vs `.ffi`). It's standard language syntax, just gated by project configuration.
4.  **Reviewability:** When auditing dependencies, you only need to audit the packages marked `kind = "system"`. Everything else is guaranteed safe by the compiler.

### 5. Implementation Roadmap Update

To support this, we need to tweak **Milestone 21** in the roadmap.

**Milestone 21: C ABI & The "System" Capability**
*   **Task 1:** Add `kind` field to `ryo.toml` parser.
*   **Task 2:** Implement `unsafe` block parsing.
*   **Task 3:** Implement the "Gatekeeper" pass in the compiler (Block `unsafe` usage if `kind != system`).
*   **Task 4:** Implement `extern "C"` parsing (allowed only in system packages).

### 6. What about the Standard Library?
The Standard Library (`std`) is effectively the "Root System Package."
*   It is written in Ryo.
*   It is compiled with the "System" capability enabled.
*   It internally uses `unsafe` to implement `File.open`, `Socket.connect`, etc.
*   It exposes **Safe** structs (`File`, `Socket`) to the user.

This proves the model works: The user uses `std` safely, never knowing it contains `unsafe` code internally.