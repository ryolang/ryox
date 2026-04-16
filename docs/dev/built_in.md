For Ryo's "General Purpose" + "DX-First" goals, the distinction is critical.

*   **Built-in:** Things the **Compiler** must know to generate machine code (Grammar, Primitives, Memory Layout).
*   **Standard Library:** Things the **Runtime** provides (I/O, OS interaction, Complex logic).

Since you are writing the Runtime in Rust, you should aim to keep the **Built-in set as small as possible** to keep the compiler simple, while making the **Standard Library feel "magic"** via the Prelude (auto-imports).

Here is the breakdown:

---

### 1. Built-in (The Compiler's Domain)
These cannot be implemented in user code. The lexer/parser/codegen handles these directly.

#### A. Primitive Types
The compiler maps these directly to CPU registers or Cranelift types.
*   `int`, `float`, `bool`
*   `void` (Unit), `never` (Bottom type)
*   `char` (Unicode Scalar)

#### B. Memory Primitives ("Ownership Lite" Mechanics)
The compiler needs these to run the Borrow Checker and Layout generation.
*   `&T` (Immutable Reference)
*   `&mut T` (Mutable Reference)
*   `*void` / `*T` (Unsafe C Pointers)
*   `?T` (Optional/Nullable - Logic for `none` and `orelse` is hardcoded in codegen).

#### C. "Magic" Structs (Language Lang Items)
These are technically structs defined in the library, but the **Compiler knows their internal layout** to support literals.
*   **`&str` (String Slice):** The compiler creates these when it sees a literal `"hello"`. It builds the fat pointer (ptr + len).
*   **`Error`:** The compiler needs to generate the `!T` (Error Union) layout automatically.
*   **`list` & `map` (v0.1 only):** Because generics are hardcoded in v0.1, the compiler treats `list[...]` syntax as a compiler intrinsic, not a user struct.

---

### 2. Standard Library (The Runtime's Domain)
These are normal modules (written in Ryo or Rust-via-FFI).

#### A. The "Core" Module (`std.core` / `std.mem`)
Low-level machinery.
*   **`struct str` (Owned String):** This is just a struct wrapping a pointer/len/cap. The compiler doesn't *need* to know it's special, except for the implicit coercion rule.
*   **`fn panic(msg)`:** Wraps the runtime's abort logic.
*   **`trait Drop`:** Defined here, but the compiler looks for it to insert destructor calls.
*   **`trait Iterator`:** Defined here, used by `for` loops.

#### B. The "System" Module (`std.sys` - Hidden)
The unsafe glue layer (as discussed previously).
*   `libc_malloc`, `libc_write`, `libc_open`.

#### C. The User-Facing Modules
*   **`std.io`:** `print`, `input`, `File`.
*   **`std.fs`:** `read_file`, `path_join`.
*   **`std.env`:** `args`, `vars`.
*   **`std.net`:** `fetch` (High level), `TcpStream` (Low level).
*   **`std.task`:** `spawn`, `sleep`, `yield` (Hooks into the Green Thread runtime).
*   **`std.simd`:** `f32x4`, `i32x4` (Wraps Cranelift intrinsics).

---

### 3. The "Prelude" (The DX Bridge)
To make Ryo feel like Python, you don't want users typing `import std.io` just to print.
The **Prelude** is a set of items that are **automatically imported** into every file.

**What goes in the Prelude?**
1.  **Core Types:** `int`, `float`, `str`, `bool`, `list`, `map`, `void`.
2.  **Core Functions:** `print`, `println`, `panic`, `assert`.
3.  **Core Traits:** `Drop`, `Error` (so you can return `!T` without imports).
4.  **Constructors:** `Some`, `None` (if you use enums for optionals, though Ryo uses `?T`).

---

### 4. Decision Matrix: Where does it go?

Use this checklist when implementing a feature:

| Feature | Is it syntax? | Does it need CPU Registers? | Is it OS specific? | **Verdict** |
| :--- | :--- | :--- | :--- | :--- |
| `if / else` | Yes | Yes | No | **Built-in** |
| `x + y` | Yes | Yes | No | **Built-in** |
| `"foo"` (Literal) | Yes | Yes (Data Section) | No | **Built-in** |
| `s.len()` | No | Yes (Read memory) | No | **Stdlib (Method)** |
| `print()` | No | No | Yes (Syscall) | **Stdlib (Prelude)** |
| `list[T]` | Yes (`[]`) | Yes (Layout) | No | **Magic Type (Built-in v0.1)** |
| `File.open` | No | No | Yes | **Stdlib** |
| `task.spawn` | No | No | No (Runtime) | **Stdlib** |

### 5. Roadmap Adjustment

For **Phase 1**, your goal is to implement the **Built-ins**.
For **Phase 2**, your goal is to implement the **Stdlib** (using the Rust Runtime strategy).

**Example: The `list` implementation**
1.  **Compiler (Built-in):** Parses `[1, 2, 3]`. Allocates memory for 3 integers. Generates a `list` struct.
2.  **Runtime (Rust):** `extern "C" fn ryo_list_push(...)`.
3.  **Stdlib (Ryo):** `impl list { fn append() { unsafe { call_rust } } }`.