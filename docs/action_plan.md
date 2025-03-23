# Action Plan

This document outlines the actions to begin developing the Ryo programming language, focusing on building a functional core, REPL, compiler, and essential safety features.

## Phase 1: Core Language Foundations & Tooling (Enhanced)

**1. Formalize Language Grammar & Lexical Rules (Precise Specification)**

*   **Action:** Define a complete and formal grammar for Ryo syntax, including lexical rules.
    *   Specify lexical grammar (tokens) using regular expressions or similar. Be detailed about whitespace handling, comments, identifier rules, etc.
    *   Specify syntactic grammar (rules for combining tokens) in EBNF or a similar notation. Cover all language constructs (functions, expressions, statements, control flow, async/await).
    *   Consider edge cases and ambiguities in the grammar and resolve them.
*   **Output:**  A detailed, unambiguous grammar specification document (Markdown/Text). This will be the *reference* for parsing.

**2. Lexer (Tokenizer) Implementation (Robust Tokenization & Error Handling)**

*   **Action:** Implement a robust lexer that reads Ryo source and produces a stream of tokens.
    *   Choose a lexer tool/library or implement manually (consider performance implications even for initial lexer).
    *   Implement token recognition for all defined tokens in the grammar (keywords, identifiers, operators, literals, punctuation, etc.).
    *   **Implement basic lexer-level error handling**: Detect and report invalid tokens or lexical errors (e.g., unterminated strings, invalid characters). Provide informative error messages with source code location (line, column).
*   **Output:** Lexer code, and initial tests for tokenization and error reporting.

**3. Parser Implementation (AST Construction & Syntax Error Handling)**

*   **Action:** Implement a parser that takes tokens and builds a detailed Abstract Syntax Tree (AST).
    *   Choose a parsing technique (Recursive Descent likely simpler to start).
    *   Implement parsing logic for all grammar rules, ensuring correct operator precedence and associativity.
    *   **Implement parser-level error handling**: Detect and report syntax errors (grammar violations) during parsing. Error messages should be clear and point to the location of the syntax error in the source code.
*   **Output:** Parser code, AST node definitions, and tests for parsing valid and invalid syntax, including syntax error reporting.

**4. Abstract Syntax Tree (AST) Design (Comprehensive Representation)**

*   **Action:** Design a comprehensive AST structure that accurately represents all Ryo language constructs and necessary information for later stages.
    *   Refine `ASTNodeKind` enum and `ASTNode` structure. Ensure it can represent *all* elements defined in the grammar.
    *   Define detailed data structures for each node type (e.g., `FunctionDefinitionNode`, `AsyncFunctionDefinitionNode`, `IfStatementNode`, `BinaryExpressionNode`, `VariableDeclarationNode`, `TypeAnnotationNode`, `BlockNode` etc.). Include source location information in each node for better error reporting.
*   **Output:**  Complete AST node definitions in code.

**5. Basic Interpreter Setup (Execution Framework & Value Representation)**

*   **Action:** Set up a robust interpreter framework.
    *   Implement a well-structured AST walker/visitor pattern for interpretation.
    *   **Design and implement a basic type system representation in the interpreter** (even if initially simple, it needs to represent types for variables and expressions during execution).
    *   Implement robust environment/scope management for variables and functions (consider nested scopes, function scopes).
    *   **Define a clear value representation** for Ryo data types (integers, floats, strings, booleans, `Result`, `Future` - even if `Future` is initially a placeholder). Choose how to represent these values in your host language.
    *   **Implement error handling within the interpreter itself:**  Handle runtime errors (e.g., division by zero, type errors during operations) gracefully and provide informative error messages.
*   **Output:** Interpreter framework code, type system representation, value representation, runtime error handling mechanism.

**6. Implement Core Language Features (Feature-Rich Runnable Core)**

*   **Action:** Implement a more feature-rich set of core language features.
    *   Data Types: `int`, `float`, `str`, `bool`, **`Result[T, E]`**.
    *   Operators: Arithmetic, Comparison, Logical, Modulo.
    *   Variable Assignment (Implicit Declaration initially).
    *   Function Definition & Call (`fn`, **with return type annotations**).
    *   `print` function (and potentially other basic I/O).
    *   `if`, `elif`, `else` statements.
    *   `for` loop (`range`).
    *   **`match` expression for `Result` handling.**
*   **Output:** Interpreter code implementing these extended core features, including `Result` and `match`.

**7. Implement REPL (Advanced Features & Error Handling)**

*   **Action:** Build a user-friendly and feature-rich REPL.
    *   **Enhance REPL error handling**: Display detailed error messages (lexing, parsing, runtime errors) with source code context in the REPL.
    *   **Implement line editing and history (for user convenience).**  Use a library for REPL functionality if available in your host language.
    *   **Consider adding basic debugging features to the REPL** (e.g., inspect variable values).
*   **Output:**  Enhanced REPL with better error reporting and user-friendliness.

**8. Basic Compiler/Build Process Setup (Error Reporting & Source Mapping)**

*   **Action:** Set up a basic compiler/build process that can process Ryo source files and provide good error reporting.
    *   For an interpreter: Enhance CLI to take `.ryo` files, lex, parse, and interpret them.
    *   **Implement source code location tracking throughout the compilation/interpretation pipeline.** Ensure error messages (lexing, parsing, runtime, borrow checker later) include accurate line and column numbers from the original source code. This is crucial for debugging.
    *   **(Optional - later):** If moving towards compilation: Begin outlining compilation stages (frontend, IR, backend), even if initial output is still interpreted or bytecode.
*   **Output:** CLI Ryo executable capable of running `.ryo` files with good error reporting.

**9. Basic Memory Management Strategy (Outline - Even for Interpreter)**

*   **Action:** Define a basic memory management strategy for your Ryo runtime, even if starting with a simple interpreter.
    *   Decide how objects and data will be allocated and managed in memory.
    *   For an interpreter: How are values stored and passed around? How is memory reclaimed? (Even simple interpreters need some form of memory management, even if it's host language's GC).
    *   **(If considering compilation later):** Outline potential memory management strategies for compiled Ryo code (e.g., manual memory management with borrow checking enforcement, garbage collection, or a hybrid approach).
*   **Output:** Document outlining the basic memory management approach for the Ryo runtime (interpreter initially).

**10. Comprehensive Testing & Iteration (Robustness & Error Paths)**

*   **Action:**  Significantly expand testing to ensure robustness and cover error handling thoroughly.
    *   **Test error conditions extensively**: Write tests specifically for lexer errors, parser errors, runtime errors (division by zero, type mismatches in operations, `match` expression failures, etc.).
    *   **Test `Result` and `match` error handling patterns thoroughly.**
    *   **Increase code coverage**: Aim for good test coverage of all language features implemented so far, including error paths and edge cases.
    *   Use REPL extensively for manual testing and exploration.
*   **Output:**  Comprehensive test suite covering valid code, error conditions, and edge cases.

## Phase 2: Safety & Concurrency (Borrow Checker & Async/Await - As Previously Defined)

**10. Implement Borrow Checker (Async-Aware - Simplified Rules)**

*   **Action:** Implement the simplified borrow checker with the rule: "No mutable borrows across `await` points."
    *   Add a semantic analysis pass to the compiler/interpreter.
    *   Implement borrow tracking logic (scopes, borrow types, `await` points).
    *   Implement compile-time error reporting for borrow violations.
*   **Output:** Borrow checker implementation code integrated into the compiler/interpreter.

**11. Implement `async/await` Runtime (Basic)**

*   **Action:** Implement a basic `async/await` runtime.
    *   Define `Future` representation.
    *   Implement a simple scheduler/executor (event loop).
    *   Implement `run_async` entry point function.
*   **Output:** `async/await` runtime implementation code.

**12. Implement `async func` and `await` Parsing & Compilation/Interpretation**

*   **Action:** Extend parser and compiler/interpreter to handle `async func` and `await` syntax and semantics.
*   **Output:** Parser and compiler/interpreter code updates for `async/await` support.

**13. Minimal Standard Library (Essential Functions)**

*   **Action:** Implement a minimal standard library.
    *   `print`, `range`, and potentially basic I/O functions (if desired for early async testing).
*   **Output:** Standard library code for essential functions.

**14. Comprehensive Testing & Iteration (For Safety & Async)**

*   **Action:** Expand testing to cover borrow checker and `async/await` functionality.
    *   Write unit tests specifically for borrow checker rules.
    *   Write integration tests for `async/await` scenarios.
    *   Test error handling with `Result` and `match` (if implemented later).
    *   Run all tests and debug/refine.
*   **Output:** Expanded test suite covering safety and concurrency features.

## Phase 3: Documentation & Community (Initial Steps - For Later)

*   **(Placeholder - For future phases):** Start planning for initial documentation (tutorials, language reference), and think about how to potentially build a small Ryo community (website, forums - much later stage).

**Tools & Technologies (Refined):**

*   **Host Language:** Choose implementation language (Rust, Go, C++, Python - consider performance and safety if aiming for a compiler later).
*   **Parser/Lexer Tools (Optional but Recommended):** Parser and lexer generators (Yacc/Bison, ANTLR, Lex/Flex, PLY) can significantly speed up lexer and parser development and improve robustness.
*   **Version Control:** Git (essential).
*   **Testing Framework:** Choose a robust testing framework in your host language.
*   **REPL Library (Optional):**  Explore REPL libraries in your host language to simplify REPL implementation.
*   **Consider a Build System/Automation:** Even for an interpreter, using a build system (like Make, CMake, Cargo, Go modules, etc.) can help organize the build process, testing, and potentially packaging.

**Focus on Iteration, Testing, and Error Handling from the Start!  Good error messages and a robust, well-tested foundation are crucial for a usable and enjoyable programming language.**

