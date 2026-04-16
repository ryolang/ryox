You have built a very solid architectural foundation. However, to transition from a "Toy Language" to a "Production Language" (especially one targeting Web Services and Data Science), you must address the **Operational** and **Ecosystem** realities of 2025.

Here are the final 5 considerations to add to your design document.

---

### 1. Supply Chain Security (The "NPM/Pip" Problem)
Modern developers are terrified of software supply chain attacks (typosquatting, malicious build scripts). Since Ryo uses a central registry (Phase 5), you must design security **into the client**.

*   **The Risk:** A user adds `pkg:left-pad`. That package contains a `build.rs` (or equivalent) that steals SSH keys during installation.
*   **Ryo Consideration:**
    *   **No "Install Scripts":** By default, installing a package should **never** execute code. It should only download sources.
    *   **Sandboxed Builds:** If a "System" package needs to compile C code (Milestone 21), it must happen in a restricted environment or explicitly ask permission: *"Package 'sqlite-sys' wants to run a build script. Allow? [y/N]"*.
    *   **Lockfile Hashing:** `ryo.lock` must store cryptographic hashes of tarballs, not just versions.

### 2. Observability Hooks (For the "Ambient Runtime")
You are building a Green Thread runtime for Network Services (Phase 5). In production, people need to know: *"Why is this request slow?"*

*   **The Problem:** In a "Colorless" async world (Green Threads), traditional profilers often get confused by stack swapping. They see the Scheduler running, not the Request logic.
*   **Ryo Consideration:**
    *   **Runtime Events:** The `libryo_runtime` (Rust) must expose an event stream (e.g., `on_thread_park`, `on_thread_start`).
    *   **Context Propagation:** The Thread-Local Runtime Context needs a slot for **Trace IDs** (OpenTelemetry). This allows a Request ID to survive a stack swap automatically, enabling distributed tracing without user code changes.

### 3. Cross-Compilation (The "Mac vs. Linux" Reality)
Your target audience uses macOS (Apple Silicon) for development but deploys to Linux (AMD64/ARM64) containers.

*   **The Problem:** If Ryo relies heavily on the system C compiler (`cc`) for linking (Milestone 3), cross-compiling becomes a nightmare of installing GCC toolchains.
*   **Ryo Consideration:**
    *   **Zig-style Linking:** Eventually, you might want to bundle `lld` (LLVM Linker) or use `zig cc` as a backend to allow `ryo build --target x86_64-unknown-linux-gnu` to work out of the box from a Mac.
    *   **Pure Ryo/Rust Deps:** Encourage libraries to be pure Ryo/Rust where possible to avoid libc dependency hell.

### 4. Source Maps / Debug Info (DWARF)
You have `panic` stack traces (Milestone 25), but can a user attach a debugger (GDB/LLDB) and step through code?

*   **The Problem:** Cranelift generates machine code. If you don't emit DWARF debug data, GDB only sees assembly, not Ryo source lines.
*   **Ryo Consideration:**
    *   **Line Number Mapping:** Your Codegen phase must strictly map Cranelift instructions back to `Span` (File/Line/Col) in the AST.
    *   **Struct Layouts:** You need to describe Ryo `structs` to DWARF so GDB knows that "Offset 0 is `id`" and "Offset 8 is `name`".
    *   **Action:** Add a "Debug Info Emission" task to Phase 3 or 4. Without this, "Data Science" users will struggle to debug logic errors.

### 5. The "Windows Path" Trap (OS Reality)
You defined `str` as UTF-8.
*   **The Reality:**
    *   **Linux/macOS:** Paths are arbitrary bytes (usually UTF-8).
    *   **Windows:** Paths are UTF-16 (UCS-2).
*   **The Conflict:** If `std.fs.open(path: str)` expects UTF-8, what happens when a user encounters a Windows file with invalid UTF-8 characters?
*   **Ryo Consideration:**
    *   **The "Go" Approach:** Treat paths as strings, but handle conversion errors at the syscall boundary. This is "DX-First."
    *   **The "Rust" Approach:** Create `OsString`. This is "Correct" but annoying.
    *   **Decision:** Stick to `str` (UTF-8) for DX. If a file path is not valid UTF-8, Ryo simply **cannot open it**. This is an acceptable trade-off for a general-purpose language in 2025 (most things are UTF-8 now), but it must be documented.

---

### Summary of Additions to Roadmap/Spec

1.  **Security:** Spec must define "Safe Package Installation" (No arbitrary code exec).
2.  **Observability:** Spec the `Runtime` to support OTel Context Propagation.
3.  **Debugging:** Compiler must eventually emit **DWARF** data.
4.  **Unicode:** Explicitly state that Ryo is **UTF-8 Only**. Non-UTF-8 file paths are unsupported (or require raw byte access via unsafe `std.sys`).