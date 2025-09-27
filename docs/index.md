---
title: Ryo Programming Language - Productive, Safe, Fast
hide:
  - navigation # Optional: Hide navigation on the main page for a cleaner look
  - toc        # Optional: Hide table of contents on the main page
---

<style>
  .md-typeset h1 {
    text-align: center;
    font-weight: bold;
    margin-bottom: 0.5em;
  }
  .md-typeset .hero-subtitle {
    text-align: center;
    font-size: 1.2em;
    color: var(--md-typeset-color);
    margin-bottom: 1.5em;
  }
  .md-typeset .pronunciation {
    font-style: italic;
    color: var(--md-accent-fg-color); /* Use theme's accent color */
  }
  .md-typeset .feature-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5em;
    margin-top: 2em;
  }
  .md-typeset .feature-item {
    text-align: center;
    padding: 1em;
    border: 1px solid var(--md-default-fg-color--lightest);
    border-radius: 4px;
  }
  .md-typeset .feature-item .icon {
    font-size: 3em;
    margin-bottom: 0.5em;
  }
  .md-typeset .cta-buttons {
    text-align: center;
    margin-top: 2em;
    margin-bottom: 2em;
  }
  .md-typeset .cta-buttons .md-button {
    margin: 0.5em;
  }
  .md-typeset .example {
    margin-bottom: 2em;
  }
</style>


<p style="text-align: center;"><img src="assets/ryo-logo-dark.svg" alt="Ryo Logo" width="150"></p>

# ⚡ Ryo Programming Language

<p class="hero-subtitle">
Productive, Safe, and Fast.<br>
<span class="pronunciation">/ˈraɪoʊ/ (Rye-oh)</span>
</p>

Ryo is a modern, statically-typed, compiled programming language designed for developers who love the **simplicity of Python** but need the **performance and memory safety** guarantees of languages like Rust or Go, without the steep learning curve.

Build reliable and efficient **web backends, CLI tools, and scripts** with an approachable syntax, powerful compile-time checks, and a familiar async/await concurrency model. Ryo manages memory safely via ownership and borrowing (simplified, no manual lifetimes) **without a garbage collector**, ensuring predictable performance and eliminating entire classes of bugs.

!!! warning "Development Status"

    Ryo is currently in the **early stages of development** (pre-alpha). The language design is stabilizing, but the compiler and standard library are under active construction. It is **not yet ready for production use**. We welcome feedback and contributions!

<div class="cta-buttons">
    <a href="getting-started/" class="md-button md-button--primary">
      Get Started
    </a>
    <a href="specification/" class="md-button">
      Read the Spec
    </a>
    <a href="https://github.com/ryolang/ryo/" target="_blank" rel="noopener" class="md-button">
      View on GitHub
    </a>
</div>

---

## Why Ryo?

<div class="feature-grid">
  <div class="feature-item">
    <div class="icon">🐍</div>
    <h3>Simple & Productive</h3>
    <p>Write clear, readable code with a clean syntax inspired by Python. Reduce boilerplate with features like f-strings, tuples, built-in `print`/`len`, and implicit package management.</p>
  </div>
  <div class="feature-item">
    <div class="icon">🛡️</div>
    <h3>Safe & Reliable</h3>
    <p>Compile-time memory safety via "Ownership Lite" prevents common errors without a GC. Explicit `Result` and `Optional` types ensure robust error and null handling.</p>
  </div>
  <div class="feature-item">
    <div class="icon">🚀</div>
    <h3>Fast & Efficient</h3>
    <p>Compiled to native code (or Wasm) using Cranelift. No GC pauses mean predictable speed. Familiar async/await concurrency for scalable applications with excellent Python developer ergonomics.</p>
  </div>
</div>

---

## Quick Look

Get a feel for Ryo's syntax with this simple example:

```ryo title="src/main.ryo"
# Import necessary standard library packages
import net.http
import io.files

struct User:
    id: int
    name: str
    email: str

#: Async function to fetch user data from API
async fn fetch_user(user_id: int) -> Result[User, HttpError] {
    response = await http.get(f"https://api.example.com/users/{user_id}")
    user = await response.json[User]()
    return Ok(user)
}

#: Async function to save user to file
async fn save_user_profile(user: &User) -> Result[(), IoError] {
    content = f"User Profile:\nID: {user.id}\nName: {user.name}\nEmail: {user.email}\n"
    await files.write_text(f"profiles/{user.id}.txt", content)
    return Ok(())
}

#: Async function to fetch multiple users
async fn fetch_multiple_users() -> Result[List[User], Error] {
    # Concurrent API requests - very familiar to Python developers
    user_tasks = [
        fetch_user(1),
        fetch_user(2), 
        fetch_user(3)
    ]
    
    users = await async.gather(user_tasks)?
    return Ok(users)
}

#: Main application entry point  
fn main() -> Result[(), Error] {
    print("Fetching user profiles...")
    
    # Start async runtime and run async code
    users = async_runtime.run(fetch_multiple_users())?
    print(f"Fetched {users.len()} users successfully!")
    
    # Process users 
    for user in users {
        # Run async operations via runtime
        async_runtime.run(save_user_profile(&user))?
        print(f"Saved profile for {user.name}")
    }
    
    return Ok(())
}

```

---

## Core Features

*   **Memory Safety without GC:** Simplified Ownership & Borrowing ("Ownership Lite") prevents memory errors at compile time. Deterministic cleanup via `Drop`.
*   **Async/Await Concurrency:** Simple and safe concurrency using familiar async/await patterns with a high-performance runtime. Perfect for I/O-bound applications.
*   **Python-Inspired Syntax:** Clean, readable, tab-indented code. Includes f-strings, tuples, built-in `print`/`len`.
*   **Static Typing:** Catch type errors at compile time. Type inference for local variables keeps code concise.
*   **Modern Tooling:** Integrated package manager (`ryo`), fast compiler (Cranelift), REPL (JIT), built-in testing.
*   **Compile-Time Execution (`comptime`):** Run code during compilation for metaprogramming, configuration, and optimization.
*   **Explicit Error Handling:** `Result[T, E]` and `Optional[T]` with the `?` operator ensure robust handling of errors and absence of values.
*   **C Interoperability:** Standard C FFI allows leveraging existing native libraries (requires `unsafe`).
*   **Future Concurrency Extensions:** CSP-style channels (`chan[T]`, `select`) planned as optional additions for specialized use cases like actor systems and data pipelines.

---

## Get Involved

*   **[Get Started](getting-started.md){ .md-button }:** Install Ryo and run your first program.
*   **[Read the Docs](specification.md){ .md-button }:** Dive deeper into the language specification and standard library.
*   **[Community Chat](link-to-discord-or-forum){ .md-button }:** Join the discussion, ask questions, and connect with other developers.
*   **[Contribute on GitHub](link-to-github-repo){ .md-button }:** Report issues, suggest features, or contribute code.
