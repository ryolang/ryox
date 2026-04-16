Based on the "System Package" architecture and the need to validate the "Ownership Lite" model, the first Standard Library implementation should be **`std.io`** (Console Input/Output), backed by a hidden **`std.sys`** (C Bindings).

Here is why this is the correct starting point and how to implement it to validate your architecture.

### 1. The Choice: `std.io` (The Console)

You should implement `std.io` first because it forces you to solve the **three hardest infrastructure problems** immediately, before the language gets complex:

1.  **The Unsafe Boundary:** You must call C functions (`write`, `read`) safely.
2.  **String Layout:** You must decide how `str` (Ryo) maps to `char*` (C).
3.  **Error Handling:** I/O is the first place where `!T` (Error Unions) matters.

### 2. The Architecture (Layered Implementation)

You will actually create **two** modules. This validates your `kind="system"` proposal from the previous discussion.

#### Layer 1: `std.sys` (The Hidden Foundation)
*   **Location:** `src/std/sys.ryo`
*   **Config:** `kind = "system"` (Allows `unsafe`).
*   **Role:** Raw C bindings. No safety guarantees.

```ryo
# std/sys.ryo
package fn libc_write(fd: i32, buf: *u8, count: usize) -> isize:
    extern "C":
        fn write(fd: i32, buf: *u8, count: usize) -> isize

package fn libc_malloc(size: usize) -> *void:
    extern "C":
        fn malloc(size: usize) -> *void

package fn libc_free(ptr: *void):
    extern "C":
        fn free(ptr: *void)
```
*Note: Use `package` visibility so only the Standard Library can see these.*

#### Layer 2: `std.io` (The Safe Facade)
*   **Location:** `src/std/io.ryo`
*   **Config:** `kind = "application"` (Standard safety rules).
*   **Role:** The user-facing API.

```ryo
import std.sys

# Define the Error Type (Milestone 9)
pub error WriteError(code: int)

# Safe Wrapper
pub fn print(s: &str) -> WriteError!void:
    # 1. Access the raw parts of the string slice (Milestone 10)
    ptr = s.ptr()
    len = s.len()
    
    # 2. Call the system layer
    # (Note: In a real implementation, this 'unsafe' block would live 
    # inside a helper in std.sys to keep std.io clean, but for v0.1 
    # we might allow std to use unsafe internally)
    unsafe:
        res = sys.libc_write(1, ptr, len) # 1 = stdout
    
    if res < 0:
        return WriteError(res)
        
    return void

pub fn println(s: &str) -> WriteError!void:
    try print(s)
    try print("\n")
    return void
```

### 3. Why this validates your Roadmap

Implementing just `print` and `println` proves that:

1.  **Milestone 3 (CodeGen):** You can link against `libc`.
2.  **Milestone 10 (Strings):** You have successfully defined the memory layout of `&str` (Fat Pointer) so that `s.ptr()` and `s.len()` work correctly.
3.  **Milestone 21 (FFI Gatekeeper):** Your compiler correctly allows `extern "C"` in `std.sys` but would forbid it in a user's `main.ryo`.

### 4. The Next Step: `std.mem` (The Allocator)

Once `std.io` works for *literals* (`print("hello")`), you immediately hit a wall: **You cannot print dynamic text** (`print("Hello " + name)`).

This dictates the **Second** stdlib module: **`std.mem`**.

*   **Role:** Wraps `malloc` and `free`.
*   **Enables:** The implementation of the owned `str` type (concatenation) and `list` (arrays).
*   **Safety:** This is where you implement the **Concurrent Allocator** requirement (Milestone 30 prep) by linking `mimalloc` or just using thread-safe `libc malloc` initially.

### Summary
1.  **First:** `std.sys` (Private, Unsafe C bindings).
2.  **Second:** `std.io` (Public, Safe Console Output).
3.  **Third:** `std.mem` (Public/Private, Allocator for Strings/Lists).

This order gives you "Hello World" first (high morale), then immediately forces you to solve memory management (the core architecture).