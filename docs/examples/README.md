# Ryo Code Examples

This directory contains practical code examples demonstrating various Ryo language features and programming patterns.

## Examples Overview

### [hello_world.ryo](hello_world.ryo)
**Concepts:** Basic syntax, functions, string interpolation
- Simple "Hello World" program
- Function definitions and calls
- String formatting with f-strings

### [temperature_converter.ryo](temperature_converter.ryo)
**Concepts:** Enums, structs, pattern matching, methods
- Enum definitions with variants
- Struct definitions with fields
- Pattern matching with `match` expressions
- Function organization and code structure

### [error_handling.ryo](error_handling.ryo)
**Concepts:** Error types, error propagation, error handling with `try`/`catch`
- Error type definitions with the `error` keyword
- Error unions (`ErrorType!SuccessType`)
- The `try` keyword for error propagation
- The `catch` operator with pattern matching
- Handling empty collections and edge cases

### [ownership_borrowing.ryo](ownership_borrowing.ryo)
**Concepts:** Memory management, ownership, borrowing, references
- Ownership transfer and move semantics
- Immutable borrows (`&T`)
- Mutable borrows (`&mut T`)
- Scope-based lifetime management
- Safe memory access patterns

### [task_spawn_run.ryo](../../examples/task_spawn_run.ryo)
**Concepts:** Concurrent task execution, futures
- `task.spawn` for fire-and-forget execution
- `task.run` for returning futures
- `.await` for suspending tasks
- `task.delay` for simulating long operations

### [channel_communication.ryo](../../examples/channel_communication.ryo)
**Concepts:** Channel-based communication, ownership transfer
- Creating typed channels with `std.channel.create`
- Sending and receiving messages
- Ownership transfer via channels
- Safe concurrent communication

### [select_example.ryo](../../examples/select_example.ryo)
**Concepts:** Non-deterministic waiting, racing operations
- `select` statement for waiting on multiple operations
- Racing futures and channels
- Timeout mechanisms with `task.delay`
- Non-deterministic execution patterns

### [task_join.ryo](../../examples/task_join.ryo)
**Concepts:** Task coordination, error handling in concurrent contexts
- Launching multiple concurrent tasks
- Using `task.join` to wait for all tasks
- Error propagation with `future[!T]`
- List comprehensions with tasks

### [data_structures.ryo](data_structures.ryo)
**Concepts:** Collections, custom types, methods, iteration
- Built-in collections (`List`, `Map`)
- Custom struct and enum definitions
- Method implementations with `impl` blocks
- Complex data structures and nested types
- Iteration patterns and data processing

## Running the Examples

Once the Ryo compiler is available, you can run these examples using:

```bash
# Compile and run an example
ryo run hello_world.ryo

# Or compile to a binary
ryo build temperature_converter.ryo -o temp_converter
./temp_converter
```

## Learning Path

For beginners, we recommend following this order:

1. **hello_world.ryo** - Basic syntax and structure
2. **temperature_converter.ryo** - Data types and pattern matching
3. **error_handling.ryo** - Error handling patterns
4. **ownership_borrowing.ryo** - Memory management concepts
5. **data_structures.ryo** - Working with collections and custom types
6. **task_spawn_run.ryo** - Basic concurrent tasks
7. **channel_communication.ryo** - Channel-based communication
8. **select_example.ryo** - Non-deterministic concurrent operations
9. **task_join.ryo** - Task coordination and error handling

## Key Concepts Demonstrated

### Memory Safety
- Ownership and borrowing prevent memory leaks and use-after-free bugs
- Compile-time checking ensures memory safety without runtime overhead
- Clear rules about when data can be accessed and modified

### Error Handling
- Explicit error handling with error types and error unions
- No hidden exceptions or null pointer errors
- Ergonomic error propagation with `try` and error handling with `catch`
- Safe optional handling with `?T` types and the `orelse` operator

### Performance
- Zero-cost abstractions - high-level code compiles to efficient machine code
- No garbage collector overhead
- Predictable memory usage and performance characteristics

### Developer Experience
- Python-like syntax that's easy to read and write
- Strong type system that catches errors at compile time
- Clear error messages that help you fix problems quickly

## Additional Resources

- [Getting Started Guide](../getting_started.md) - Comprehensive introduction
- [Language Specification](../specification.md) - Detailed language reference
- [Language Comparison](../language_comparison.md) - How Ryo compares to other languages
- [Implementation Roadmap](../implementation_roadmap.md) - Development progress and plans

## Contributing Examples

If you have ideas for additional examples that would help other developers learn Ryo, please consider contributing! Good examples should:

- Demonstrate specific language features clearly
- Include helpful comments explaining the concepts
- Be practical and realistic (not just toy examples)
- Follow Ryo best practices and idioms
- Include error handling where appropriate