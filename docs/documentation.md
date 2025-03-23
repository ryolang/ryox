# Documentation

This document provides in-depth information about the Ryo programming language, covering its design philosophy, key features, and advanced usage.

## Table of Contents
- [Documentation](#documentation)
  - [Table of Contents](#table-of-contents)
  - [Language Design Philosophy](#language-design-philosophy)
  - [Object Model: Structs and Traits](#object-model-structs-and-traits)
    - [Structs](#structs)
    - [Traits](#traits)
    - [The `Drop` Feature](#the-drop-feature)
  - [Asynchronous Programming with `async/await`](#asynchronous-programming-with-asyncawait)
    - [Basic Async Functions](#basic-async-functions)
    - [Safety Rules for Async Code](#safety-rules-for-async-code)
  - [Error Handling with `Result[T, E]`](#error-handling-with-resultt-e)
    - [The `Result[T, E]` Type](#the-resultt-e-type)
    - [Handling `Result` Values with `match`](#handling-result-values-with-match)
    - [Error Propagation with `?` Operator](#error-propagation-with--operator)
    - [Error Handling in `async/await`](#error-handling-in-asyncawait)
  - [Package Manager (ryopkg)](#package-manager-ryopkg)
    - [Manifest File](#manifest-file)
    - [Essential Commands](#essential-commands)
  - [Language Specification Summary](#language-specification-summary)
    - [Target Audience and Use Cases](#target-audience-and-use-cases)

## Language Design Philosophy

Ryo is designed with three guiding principles in mind:

1. **Python-like Simplicity**: Ryo aims for a clean, readable, and approachable syntax inspired by Python. It seeks to be easy to learn and use, especially for developers familiar with Python. The focus is on reducing boilerplate and promoting code clarity.

2. **Rust-like Safety**: Ryo prioritizes safety, aiming to provide strong guarantees against data races and memory unsafety, drawing inspiration from Rust's safety model. The goal is to catch errors at compile time where possible and provide robust runtime behavior.

3. **Performance**: Ryo strives for good performance, particularly in concurrent and I/O-bound applications. It aims to target a level of performance comparable to Go in concurrent scenarios.

## Object Model: Structs and Traits

Ryo uses structs and traits rather than classes and interfaces. This design direction is deliberate and aims to balance simplicity, safety, and code reusability.

### Structs

Structs in Ryo serve a purpose similar to classes in Python in terms of data aggregation. They are primarily about grouping data fields together in a structured way.

```ryo
struct Point:
    x: int
    y: int

    # Methods associated with Point struct
    fn distance_from_origin(self) -> float:
        return sqrt(self.x*self.x + self.y*self.y)

p = Point {x: 3, y: 4}
dist = p.distance_from_origin()
print(f"Distance: {dist}")
```

### Traits

Traits define shared behavior that different types can implement. They achieve polymorphism and code reuse without the complexities of class inheritance.

```ryo
trait Drawable:
    fn draw(self) -> str # Method signature - returns a String representation

struct Circle:
    radius: float

impl Drawable for Circle:
    fn draw(self) -> str:
        return f"Drawing a circle with radius {self.radius}"

struct Square:
    side: float

impl Drawable for Square:
    fn draw(self) -> str:
        return f"Drawing a square with side {self.side}"

fn print_drawing(drawable: impl Drawable): # Polymorphic function - accepts any type that implements Drawable
    print(drawable.draw())

c = Circle{ radius: 5.0 }
s = Square{ side: 4.0 }

print_drawing(c) # Works because Circle implements Drawable
print_drawing(s) # Works because Square implements Drawable
```

### The `Drop` Feature

Ryo includes a mechanism similar to Rust's `Drop` trait for deterministic resource management:

```ryo
struct FileHandler:
    file_descriptor: int
    filename: str

# Syntax for Drop-like behavior in Ryo
impl Drop for FileHandler:
    fn drop(self): # Code to be executed when FileHandler instance is dropped
        print(f"Closing file: {self.filename}")
        # ... Code to close the file descriptor ...

fn process_file(filename: str):
    file = FileHandler{ file_descriptor: open_file(filename), filename: filename }
    print(f"Processing file: {file.filename}")
    # ... Use the file ...
    # file will be automatically dropped at the end of this function's scope,
    # and the drop() function in impl Drop for FileHandler will be executed,
    # ensuring the file is closed.
```

## Asynchronous Programming with `async/await`

Ryo supports asynchronous programming using `async` and `await`. This allows you to write code that can perform long-running operations, like network requests or file I/O, without blocking the program's execution.

### Basic Async Functions

To define an asynchronous function, use the `async fn` keywords:

```ryo
async fn fetch_data_from_web(url: str) -> str: # Define an async function
    print(f"Fetching data from {url}...")
    # Imagine 'http_get_async' is a function that performs an async network request
    data = await http_get_async(url)
    print("Data fetched!")
    return data
```

Inside an `async fn`, you use the `await` keyword to pause execution until a `Future` (representing an asynchronous operation) completes:

```ryo
async fn main():
    data1 = await fetch_data_from_web("https://example.com/api/data1") # Await the result
    print("Data 1:", data1)

    data2 = await fetch_data_from_web("https://example.com/api/data2") # Await another result
    print("Data 2:", data2)

    print("All data fetched and processed.")

run_async(main) # Run the async main function
```

### Safety Rules for Async Code

Ryo has a strict rule to help prevent data races in asynchronous code: **You cannot have mutable borrows that span across `await` points.**

```ryo
async fn problematic_function():
    let mut counter = 0
    let mutable_counter = &mut counter # Mutable borrow starts
    counter = counter + 1 # OK to use mutable_counter here

    await some_async_operation() # Suspension point

    # ERROR! Cannot use 'mutable_counter' here after 'await' because it's a mutable borrow that spanned across the 'await'
    # mutable_counter = mutable_counter + 1 # This will cause a compile error in Ryo
```

To avoid this error, restructure your code so that mutable borrows do not cross `await` points. You might need to perform mutations before the `await` or re-borrow after the `await` if needed.

## Error Handling with `Result[T, E]`

Ryo provides the `Result[T, E]` type for explicit error handling.

### The `Result[T, E]` Type

`Result` is a type that represents either a successful outcome with a value of type `T` (represented as `Ok(T)`) or a failure with an error value of type `E` (represented as `Err(E)`).

```ryo
fn divide(numerator: float, denominator: float) -> Result<float, str>:
    if denominator == 0.0:
        return Err("Cannot divide by zero") # Indicate an error as Err(string)
    else:
        return Ok(numerator / denominator) # Indicate success with Ok(result)
```

### Handling `Result` Values with `match`

To use the result of a function that returns `Result`, you need to check whether it was successful (`Ok`) or if it failed (`Err`) using the `match` expression:

```ryo
fn main():
    result1 = divide(10.0, 2.0) # Call divide - might succeed or fail
    match result1:
        Ok(value):
            print(f"Division successful, result: {value}")
        Err(error_message):
            print(f"Division failed: {error_message}")

    result2 = divide(5.0, 0.0) # Call divide with a zero denominator
    match result2:
        Ok(value): # This branch will NOT be executed for result2
            print(f"Division successful, result: {value}")
        Err(error_message): # This branch WILL be executed for result2
            print(f"Division failed: {error_message}")
```

### Error Propagation with `?` Operator

Ryo provides the `?` operator for convenient error propagation:

```ryo
fn process_data() -> Result[str, E]:
    data = fetch_data()? # Propagate error from fetch_data if it returns Err
    processed_data = transform_data(data)? # Propagate error from transform_data if it returns Err
    return Ok(f"Processed: {processed_data}")

fn fetch_data() -> Result[str, E]:
    # ... function that might return Ok(data) or Err(error) ...
    if success:
        return Ok("Raw Data")
    else:
        return Err("Failed to fetch data")

fn transform_data(raw_data: str) -> Result[str, E]:
    # ... function that might return Ok(transformed_data) or Err(error) ...
    if transformation_ok:
        return Ok(f"Transformed: {raw_data}")
    else:
        return Err("Failed to transform data")
```

The `?` operator works as follows:
- If the expression is `Ok(value)`, it unwraps to `value`.
- If the expression is `Err(error)`, it returns `Err(error)` from the current function.
- The function using `?` must return a compatible `Result<_, ErrorType>`.

### Error Handling in `async/await`

`Result` works seamlessly with `async/await`. Asynchronous functions can also return `Result`, and you can use `await` with functions that return `Future[Result[T, E]]`:

```ryo
async fn fetch_content_result(url: str) -> Result[str, E]: # Async function returning Result
    print(f"Fetching from {url}...")
    # Assume 'async_http_get_result' is an async function that returns Future[Result[str, E]]
    fetch_future_result = async_http_get_result(url)
    content_result = await fetch_future_result # Await the Future[Result[str, E]] - 'content_result' is now a Result[str, E]

    match content_result:
        Ok(content):
            print("Fetch successful.")
            return Ok(content) # Return Ok(content)
        Err(http_error):
            print(f"Fetch failed: {http_error}")
            return Err(f"Network error: {http_error}") # Propagate error as Err
```

## Package Manager (ryopkg)

Ryo includes a package manager called `ryopkg`, inspired by Cargo and Go Modules but tailored to Ryo's philosophy.

### Manifest File

Ryo projects use a TOML-based manifest file called `ryo.toml`:

```toml
# ryo.toml
[package]
name = "hello_ryo"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
edition = "2024"

[dependencies]
ryo-utils = "1.0"
fast-http = "^0.3"
```

### Essential Commands

1. **`ryopkg new <project_name>`**: Create a new Ryo project.
   ```bash
   ryopkg new hello_ryo
   ```

2. **`ryopkg add <package_name> [<version_constraint>]`**: Add a dependency to the current project.
   ```bash
   ryopkg add ryo-utils
   ryopkg add fast-http ^0.3
   ```

3. **`ryopkg install`**: Install project dependencies.

4. **`ryopkg build`**: Build the current project and its dependencies.

5. **`ryopkg run`**: Run the main executable of the current project.

6. **`ryopkg test`**: Run tests for the current project.

7. **`ryopkg publish`**: Publish a package to `ryopkgs.io`.

8. **`ryopkg update`**: Update dependencies.

9. **`ryopkg lock`**: Generate or refresh the `ryo.lock` file without updating dependencies.

## Language Specification Summary

| Feature Category      | Ryo Characteristic                                          | Inspiration Source | Key Aspect                                              |
| --------------------- | ----------------------------------------------------------- | ------------------ | ------------------------------------------------------- |
| **Simplicity**        | Python-like Syntax, Type Inference, Concise Features        | Python             | Readability, Ease of Learning, Reduced Boilerplate      |
| **Safety**            | Borrowing/Ownership, Static Typing, `Result` Error Handling | Rust               | Memory Safety, Data Race Freedom, Compile-Time Errors   |
| **Performance**       | `async/await` with Implicit Futures, Optimized Runtime      | Go                 | High Concurrency, Efficient I/O, Good Throughput        |
| **Concurrency Model** | Explicit `async/await`                                      | Python, Go, Rust   | Structured Asynchrony, Safe Communication               |
| **Error Handling**    | `Result[T, E]` Type, `match` Expressions                    | Rust               | Robust, Explicit, Compile-Time Checked Error Management |

### Target Audience and Use Cases

- **Target Audience**: Python developers seeking improved performance, enhanced safety, and stronger concurrency capabilities, while retaining a familiar and approachable syntax.
- **Suitable Use Cases**:
  - Web servers and backend services
  - Scripting
  - Networked applications and distributed systems
  - Concurrent and parallel processing tasks
  - Applications benefiting from asynchronous I/O 