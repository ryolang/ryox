# Ryo's Memory Management

Ryo's memory management provides memory safety with compile-time guarantees while maintaining an approachable syntax. It bridges the gap between Python's ease of use and Rust's safety guarantees.

## Glossary of Key Terms

**Ownership**: The concept that exactly one variable "owns" a value and is responsible for its cleanup.

**Move**: Transferring ownership of a value from one variable to another, invalidating the original variable.

**Copy**: Duplicating a value so both the original and new variables have independent copies.

**Reference**: A way to access a value without taking ownership of it.

**Shared reference**: A read-only (immutable) reference that allows multiple simultaneous readers.

**Mutable reference**: A reference that allows changing the referenced value, limited to one at a time.

**Weak reference**: A non-owning reference that doesn't prevent cleanup of the referenced value.

**Drop**: Automatic cleanup of resources when a value goes out of scope.

**Borrowing**: Temporarily accessing a value via a reference without taking ownership.

## Design Goals

1. **Memory Safety**: Prevent common memory errors at compile time
2. **Python-like Simplicity**: Maintain ergonomics similar to Python
3. **Rust-like Safety**: Compile-time guarantees without full complexity
4. **Web Service & Scripting Focus**: Optimized for these use cases rather than systems programming
5. **Python-Rust Interoperability**: Fill the gap between these languages

## Reference Semantics Summary

*This table summarizes how different types are handled in function parameters and return values:*

| Parameter Type                | default  | Explicit Options | Note                          |
| ----------------------------- | -------- | ---------------- | ----------------------------- |
| Primitives (int, float, etc.) | By value | `shared`, `mut`  | Copied by default             |
| Collection types (list, dict) | `shared` | `mut`, `owned`   | Read-only by default          |
| Custom structs                | `shared` | `mut`, `owned`   | Read-only by default          |
| Return values                 | `owned`  | `shared`, `mut`  | Transfer ownership by default |

## 1. Ownership Model and Value Semantics

### 1.1 Basic Ownership Rules

Every value in Ryo has exactly one owner at any given time. When ownership is transferred, the previous variable can no longer be used.

```ryo
# Ownership example
data = [1, 2, 3]        # data owns this list
processed = data        # ownership transferred to processed (a move operation)
print(data)             # Error: data no longer owns anything (it was moved)
```

This prevents use-after-free errors and ensures deterministic cleanup when the owner goes out of scope.

### 1.2 Value Types and Reference Types

Ryo classifies types based on their size and complexity to optimize memory operations:

- **Value types**: Small, simple types that are copied by default
  - Primitives (int, float, bool)
  - Small strings (under 24 bytes)
  - Small structs (under 64 bytes with only primitive fields)
  - Enums (without complex fields)
- **Reference types**: Larger, complex types that follow ownership rules by default
  - Collections (lists, dictionaries, sets)
  - Large strings (24+ bytes)
  - Structs containing reference types
  - Any type over 64 bytes in size

```ryo
# Value types are copied implicitly
struct Point:
    x: int
    y: int

p1 = Point(1, 2)
p2 = p1           # Point is small (8 bytes), so p1 is COPIED to p2 (both exist independently)

# Reference types follow ownership rules
struct Image:
    pixels: list[int]
    width: int
    height: int

img1 = Image(...)
img2 = img1       # Ownership MOVED, img1 is invalidated
img3 = img1.clone()  # Explicit clone required for reference types
```

> **Implementation Note**: The 64-byte threshold represents a practical balance between performance and memory usage. Types smaller than this threshold are typically cheaper to copy than to track with references, while larger types benefit from reference semantics to avoid expensive copies. This threshold is an implementation detail that may be adjusted in future versions based on performance analysis.

The distinction between value and reference types is purely a performance optimization - the same fundamental memory safety rules apply to all types. The compiler handles this distinction automatically, so developers rarely need to think about it except when writing performance-critical code.

### 1.3 Copy vs Move Behavior

For clarity, Ryo follows these guidelines about when values are copied versus moved:

```ryo
# Automatic COPY for:
# - Primitive types (int, float, bool)
# - Small strings (under 24 bytes)
# - Small structs (under 64 bytes with primitive fields)
x = 42
y = x  # Copy: both x and y are valid and independent

# Automatic MOVE for:
# - Collections (list, dict, set)
# - Large strings
# - Complex structs
# - Any type containing references
data = [1, 2, 3]
new_data = data  # Move: data is no longer valid, ownership transferred

# When in doubt, be explicit with clone()
safe_copy = data.clone()  # Explicit copy: both original and copy valid
```

This predictable behavior makes it easy to understand when data is copied vs moved without complex rules.

## 2. Reference System

### 2.1 Reference Types and Their Uses

Ryo uses three primary reference types, with intelligent defaults to minimize annotation burden:

```ryo
# Three types of references, shown with explicit annotations
fn read_name(person: shared Person):  # Shared (read-only) reference
    return person.name

fn update_age(person: mut Person):  # Mutable reference
    person.age += 1
    
fn consume_data(data: owned Data):  # Taking ownership
    transform(data)
    # data is consumed here
```

default reference behaviors:
- **Primitives** (int, float, bool, small strings): By value (copied)
- **Collections and custom structs**: Shared (read-only) by default
- **Return values**: Owned by default

> **Cross-reference**: See the Reference Semantics Summary table above for a complete overview.

### 2.2 Automatic Borrowing for Function Calls

For most function calls, Ryo automatically manages borrowing without requiring explicit annotations:

```ryo
# The compiler automatically handles these as temporary borrows
data = [1, 2, 3]

# No explicit borrowing syntax needed for function calls
sum(data)         # Compiler creates temporary shared (read-only) reference
data.append(4)    # Compiler creates temporary mutable reference
process(data)     # Compiler infers from how process uses its parameters

# Method chaining works naturally
result = data.filter(is_even).map(double).sum()
```

> **Important**: Even with automatic borrowing, the core safety rules in section 2.3 are still enforced by the compiler. You cannot have simultaneous mutable access from multiple places.

This preserves Python's clean function call syntax while ensuring memory safety behind the scenes.

> **IDE Integration**: Ryo's tooling provides hover information to show what type of reference (shared, mutable, or owned) is being inferred for each function call. This gives you immediate feedback without requiring explicit annotations.

### 2.2.1 Deterministic Borrowing Inference Rules

The compiler follows these specific rules when inferring reference types:

1. **Shared (read-only) References** are inferred when:
   - The function only reads from a parameter (no modification)
   - Multiple simultaneous references to the same data are needed
   - The parameter is used in a read-only context (e.g., in an equality check)

2. **Mutable References** are inferred when:
   - The function modifies the parameter in place (e.g., appending to a list)
   - The parameter is passed to a function that requires a mutable reference
   - Methods that modify the object are called on the parameter

3. **Ownership Transfer** is inferred when:
   - The function stores the parameter for later use beyond the function's scope
   - The function returns the parameter or a derived value containing it
   - The parameter is moved into a data structure that takes ownership

```ryo
# Examples of inference in action:

# Inferred as: fn calculate_total(items: shared list[int]) -> int
fn calculate_total(items: list[int]) -> int:
    total = 0
    for item in items:  # read-only operation
        total += item
    return total
    
# Inferred as: fn add_item(items: mut list[int], item: int) -> none
fn add_item(items: list[int], item: int):
    items.append(item)  # modifies items in place
    
# Inferred as: fn create_service(config: owned Config) -> Service
fn create_service(config: Config) -> Service:
    service = Service()
    service.config = config  # config is stored in service (ownership transfer)
    return service
```

When the compiler cannot confidently determine the appropriate reference type (for example, if a function is overloaded with different reference requirements), you'll need to provide explicit annotations.

### 2.3 Safety Rules for References

Ryo enforces these safety rules at compile time:

1. At any time, you can have **either**:
   - One mutable reference (`mut`)
   - Any number of shared (read-only) references (often implicit)

2. References cannot outlive the data they refer to

3. References must be valid for their entire scope

These rules prevent data races, dangling references, and use-after-free errors.

```ryo
# Example enforcing the safety rules
data = [1, 2, 3]

# This works - multiple shared (read-only) references
r1 = shared data  # Explicit shared reference
r2 = shared data  # Another shared reference
print(r1[0], r2[0])  # Can use both simultaneously

# This fails - cannot have mutable and shared references simultaneously
m = mut data      # Mutable reference
r = shared data   # ERROR: cannot have shared reference while mutable exists

# This also fails - cannot have multiple mutable references
m1 = mut data     # First mutable reference
m2 = mut data     # ERROR: cannot have second mutable reference
```

### 2.4 Null Safety with Optional Types

Ryo eliminates null reference errors through compile-time Optional types:

```ryo
# No implicit nulls - every variable has a valid value of its type
name: str = "Alice"             # Cannot be null
maybe_name: ?str = None         # Optional type with ? shorthand (can be None)

# Safe unwrapping with compiler enforcement
if maybe_name:                  # Compiler tracks this null check
    print(maybe_name)           # Automatically unwrapped in this scope
else:
    print("No name")            # This branch handles the None case

# Cannot use an Optional without checking
print(maybe_name.upper())       # Compile error: maybe_name might be None

# Pattern matching for optionals
match maybe_name:
    case Some(value):           # When value exists
        print(f"Name: {value}")
    case None:                  # When it's None
        print("No name provided")
```

#### When to use if-checks vs. pattern matching

- **Use if-checks when**:
  - You need a simple presence check
  - The unwrapped value is used in only one branch
  - The check is part of a larger condition

- **Use pattern matching when**:
  - You need to handle multiple cases or extract inner values
  - You want to destructure complex optional types
  - You prefer more explicit naming of the unwrapped value

Both approaches provide the same compile-time safety guarantees.

## 3. Resource Management

### 3.1 Automatic Cleanup with Drop

Resources are automatically cleaned up when they go out of scope through the `__drop__` method:

```ryo
struct File:
    _handle: FileHandle
    
    # Constructor
    fn __new__(path: str) -> File:
        return File(_handle=open_file_handle(path))
    
    # Automatic cleanup when File goes out of scope
    fn __drop__(self):
        close_file_handle(self._handle)

fn process_file(path: str):
    file = File(path)       # File is created here
    # Work with file...
    # No need for explicit close - file.__drop__() called automatically
    # when file goes out of scope at function end
    # <-- file.__drop__() called here automatically
```

This provides deterministic cleanup without garbage collection or manual resource management.

### 3.2 Error Handling and Resource Safety

Ryo ensures resources are properly cleaned up even when errors occur:

```ryo
# Error handling with automatic resource cleanup
fn process_file(path: str) -> Result[str, Error]:
    file = File(path)  # Automatically cleaned up when function exits
    
    # The ? operator unwraps Results or returns the error to the caller
    data = file.read()?  # If read() returns Err, function returns immediately
                         # Even when returning early, file is still cleaned up
    
    # Process data...
    if invalid_data(data):
        return Err("Invalid data format")  # Early return with error
                                           # file still cleaned up
    
    return Ok(process(data))  # Successful return
    # file.__drop__() called automatically on all exit paths
```

This ensures no resource leaks occur, even in complex error handling scenarios, without requiring explicit try/finally blocks.

> **Related concept**: The `?` operator is syntactic sugar for unwrapping `Result` types, similar to Rust. It extracts the value on success or returns the error if one occurs.

## 4. Data Structures and Collections

### 4.1 Memory-Safe Collections

Ryo ensures array accesses are always safe, eliminating out-of-bounds errors:

```ryo
# Fixed-size arrays with compile-time bounds checking
fixed: [int; 3] = [1, 2, 3]
print(fixed[2])                 # OK - within bounds
print(fixed[5])                 # Compile error: out of bounds

# Dynamic arrays with optimized runtime checks
dynamic = [1, 2, 3]
print(dynamic[safe_index])      # Runtime check, optimized when possible
```

### 4.2 Collection Views for Efficient Access

Ryo provides efficient views into collections that allow working with a portion of a collection without copying the data:

```ryo
# Slicing creates efficient VIEWS, not copies
data = [1, 2, 3, 4, 5]
window = data[1:3]  # Creates a VIEW into elements 1-2, not a copy
                    # Changes to data will be visible through window

# Views can be passed to functions efficiently
fn process_window(view: list[int]):
    # Works with the original data without copying
    for item in view:
        print(item)
        
process_window(data[1:3])  # No copy needed
```

#### View Semantics and Behavior

Views represent a bounded window into a collection with these characteristics:

1. **Mutation Behavior**:
   - When the original collection is modified, changes may be visible through the view
   - If elements are added/removed from the original collection that affect the viewed range, 
     views follow specific rules:
     - If elements are added/removed before the view's range, the view adjusts its indexing
     - If elements within the view's range are removed, the view's size is reduced
     - If new elements are inserted within the view's range, they become part of the view

   ```ryo
   data = [1, 2, 3, 4, 5]
   window = data[1:3]  # window views [2, 3]
   
   data.insert(0, 0)   # data is now [0, 1, 2, 3, 4, 5]
                       # window still views [2, 3] but at indices 2-3
   
   data.remove(2)      # data is now [0, 1, 3, 4, 5]
                       # window now views [3] at index 2
   ```

2. **Safety Guarantees**:
   - Views follow the same safety rules as references
   - A view cannot outlive the collection it references
   - Mutable operations on the original collection may invalidate views if they would make the view unsafe

3. **View vs. Reference**:
   - A view is a specialized form of reference that includes bounds information
   - Like a reference, a view doesn't own the data it points to
   - Unlike a general reference, a view only accesses a specific range of the collection
   - Views apply the same ownership and borrowing rules as regular references

4. **Performance Characteristics**:
   - Views have minimal overhead (typically just start/end indices)
   - No data copying occurs when creating or using a view
   - Compiler optimizations can eliminate bounds checks when safe to do so

This approach preserves Python's easy slicing syntax while ensuring memory efficiency and safety.

### 4.3 Data Sharing Patterns

Ryo makes it easy to efficiently share data between functions:

```ryo
# Simple patterns for efficient data sharing
fn process_large_data(data: list[int]) -> list[int]:
    # For read-only access, data is automatically borrowed
    total = sum(data)  # No copy made, uses shared (read-only) reference
    
    # For creating derived data without modifying original
    results = []
    for item in data:  # No copy of data is made during iteration
        results.append(item * 2)
    
    return results  # Ownership of results transferred to caller
```

The compiler automatically optimizes data sharing patterns, avoiding unnecessary copies while maintaining memory safety.

### 4.4 Handling Circular References

Ryo provides weak references to handle circular relationships without creating ownership cycles:

```ryo
# Pattern for parent/child relationships with circular references
struct TreeNode:
    value: int
    parent: weak TreeNode  # Non-owning reference that doesn't affect cleanup
    children: list[TreeNode]  # Owning references (each child owned by this node)
    
# Creating a tree with parent references
root = TreeNode(value=1)
child = TreeNode(value=2)
root.children.append(child)    # root owns child
child.parent = weak(root)      # weak() creates a weak reference to root
                               # that doesn't prevent root from being cleaned up

# Using weak references safely
if let parent = child.parent:  # Safely checks if reference is valid
    print(parent.value)        # Only executes if the parent still exists
```

#### Weak Reference Details

Weak references have these key characteristics:

1. **Validity Checking**: Weak references must be checked before use
   - `if let` syntax provides a scoped, safe access pattern
   - Returns `None` if the target has been cleaned up

2. **Performance**: Weak references add minimal overhead
   - Single atomic reference count check
   - No impact on the performance of the target object

3. **Error Prevention**: Accessing an invalid weak reference directly causes a compile error
   ```ryo
   print(child.parent.value)  # ERROR: must check weak reference first
   ```

4. **Explicit Weakening**: Use the `weak()` function to create weak references
   ```ryo
   weak_ref = weak(object)  # Creates a weak reference
   ```

> **Cross-reference**: See section 5.4 for how the compiler helps prevent accidental ownership cycles.

The `weak` reference type allows back-references without creating ownership cycles, solving common problems in tree and graph structures without complex lifetime annotations.

## 5. Compiler Optimizations

### 5.1 Escape Analysis

The compiler determines when data can be stack-allocated instead of heap-allocated:

```ryo
fn process_local():
    # The compiler detects that data doesn't escape this function
    data = [1, 2, 3]  # Allocated on stack since it doesn't escape
                      # No heap allocation or reference counting needed
    total = sum(data)
    return total      # Only the scalar result escapes
   # data automatically cleaned up when it goes out of scope
```

### 5.2 Static Reference Counting

The compiler eliminates runtime reference counting when possible:

```ryo
fn update_counter(counter: mut Counter):
    # The compiler can statically determine when counter is no longer needed
    counter.value += 1  # No runtime reference counting overhead
    # counter is automatically cleaned up here if it was moved into this function
```

### 5.3 Automatic Memory Optimization

The compiler automatically optimizes memory allocation without requiring developer intervention:

```ryo
fn process():
    # Compiler makes these optimizations automatically:
    
    # 1. Stack allocation for non-escaping data
    local = [1, 2, 3]  # Stack allocated when confined to this function
    
    # 2. In-place updates when safe
    local.append(4)    # No reallocation if capacity exists
    
    # 3. Eliminating unnecessary copies
    result = sum(local)  # No copy of local is made for sum()
    
    return result
```

These optimizations happen automatically without special annotations, providing performance benefits while maintaining Python-like simplicity.

### 5.4 Cycle Detection in the Compiler

The compiler prevents accidental reference cycles that would cause memory leaks:

```ryo
# Compiler prevents accidental ownership cycles
struct Node:
    next: Node  # Ownership field - will own whatever is assigned to it

a = Node()
b = Node()
a.next = b                  # Fine - a owns b
b.next = a                  # ERROR: would create ownership cycle
                            # The compiler detects this and prevents it
```

> **Solution**: To create circular structures, use weak references as shown in section 4.4.

### 5.5 Memory Layout and Performance Considerations

Ryo's memory management system is designed to optimize both safety and performance. Understanding these details can help developers write more efficient code:

#### Memory Layout

- **Value Types**: Stored directly in variables or inline within structs
  - Primitives and small types are stored directly on the stack when possible
  - When part of a larger struct, they're stored inline within that struct
  - This improves cache locality and reduces indirection

- **Reference Types**: Stored as a pointer to heap-allocated data
  - The pointer itself may be on the stack or inline within another struct
  - The actual data is on the heap with appropriate memory management
  - This enables efficient ownership transfer without large copies

```ryo
struct User:
    id: int        # stored inline (value type)
    name: str      # pointer to heap data (reference type)
    active: bool   # stored inline (value type)
```

In this example, `id` and `active` are stored directly within the `User` struct, while `name` is a pointer to heap-allocated string data.

#### Performance Implications

1. **Stack vs. Heap Allocation**:
   - Stack allocations are faster than heap allocations
   - The compiler automatically prefers stack allocation when safe to do so
   - Variables that don't escape their function scope are typically stack-allocated

2. **Memory Locality**:
   - Value types stored inline improve cache locality
   - Related data stored together enables better performance
   - The compiler optimizes struct layouts to maximize locality benefits

3. **Copy vs. Move Costs**:
   - Value types have predictable, fixed copy costs
   - Reference types have constant-time move operations
   - Explicit `.clone()` calls have costs proportional to data size

4. **Compiler Optimizations**:
   - Copy elision: Avoiding unnecessary copies when safe
   - In-place updates: Modifying data in place when possible
   - Reference coalescing: Combining multiple references to the same data

```ryo
# The compiler applies these optimizations:

fn process_data(data: list[int]) -> int:
    # Compiler uses shared reference, no copy needed
    length = len(data)
    
    # In-place update where possible
    data.sort()
    
    # Compiler avoids copying result
    return data[0]
```

Understanding these performance characteristics allows developers to write idiomatic code that's both safe and efficient, without needing to manually optimize memory management in most cases.

## 6. Special Cases

### 6.1 Thread Safety Model

Ryo provides comprehensive thread safety guarantees through a combination of ownership rules and specialized thread-safe types:

```ryo
# Thread-safe counter without complex annotations
struct Counter:
    value: atomic[int]  # Thread-safe integer type
    
# Multiple threads can safely access/modify
fn increment(counter: shared Counter):  # Shared reference works across threads
    counter.value += 1  # Automatically thread-safe increment
    
# For more complex shared state
struct SharedState:
    data: lockable[dict[str, int]]  # Thread-safe collection
    
    fn update(self, key: str, value: int):
        with self.data:  # Automatic locking
            self.data[key] = value
```

#### Comprehensive Thread Safety Model

TBD

Ryo's thread safety is built on three complementary mechanisms:

1. **Ownership and Borrowing Rules Across Threads**:
   - The compiler enforces ownership rules across thread boundaries
   - Data can only be accessed by multiple threads if properly protected
   - Data moved to a thread is owned exclusively by that thread
   - Compiler prevents data races at compile time

   ```ryo
   fn spawn_worker(data: list[int]):  # Takes ownership of data
       thread.spawn(
           process(data)  # This thread now exclusively owns data
       )
       # Cannot use data here - ownership was transferred to the thread
   ```

2. **Thread-Safe Types**:
   - `atomic[T]`: Thread-safe versions of primitive types with atomic operations
      - Supports operations like increment, compare-and-swap, etc.
      - Prevents data races for simple values
   
   - `lockable[T]`: Automatically adds mutex protection to any type
      - Used with `with` statement for automatic locking/unlocking
      - Prevents data races for complex types
   
   - `sharedref[T]`: Thread-safe reference-counted shared access
      - Allows multiple threads to safely access the same data
      - Similar to Rust's Arc but with simpler syntax

   ```ryo
   # Thread-safe shared state
   state = sharedref[AppState]()  # Reference-counted thread-safe state
   
   # Multiple threads can safely access the same state
   thread.spawn(|| {
       with state.config:  # Automatic locking when needed
           print(state.config.debug_mode)
   })
   ```

3. **Concurrency Patterns**:
  TBD


These patterns provide thread-safety without requiring complex ownership annotations, while still maintaining memory safety guarantees.

### 6.2 Async Memory Management

Ryo provides special handling for async code to ensure resources are properly managed across await points:

```ryo
# Async/await with ownership awareness
struct Response:
    body: bytes
    
    fn __drop__(self):
        # Automatic cleanup logic (releases network resources, etc.)
        print("Response resources freed")

async fn fetch_data(url: str) -> bytes:
    resp = await http.get(url)  # resp is created here
    
    # Processing can happen here
    
    result = resp.body.clone()  # Clone the body data to return it
    return result
    # resp is automatically cleaned up here, even across await points
```

Asynchronous operations automatically track ownership across suspend points, ensuring resources are properly cleaned up regardless of the execution path.

### 6.3 Error Handling Patterns

Ryo provides comprehensive error handling approaches that maintain memory safety:

```ryo
# Result type for expected errors
fn divide(a: int, b: int) -> Result[int, DivisionError]:
    if b == 0:
        return Err(DivisionError("Division by zero"))
    return Ok(a / b)

# Using the ? operator for propagation
fn calculate(a: int, b: int) -> Result[int, CalculationError]:
    # ? unwraps or returns the error to the caller
    result = divide(a, b)?  # Returns DivisionError if divide fails
    
    return Ok(result * 2)

# Pattern matching for detailed error handling
fn handle_result(result: Result[int, Error]):
    match result:
        case Ok(value):
            print(f"Success: {value}")
        case Err(DivisionError(msg)):
            print(f"Division error: {msg}")
        case Err(err):
            print(f"Other error: {err}")

# Try expressions for local error handling
fn safe_operation():
    result = try divide(10, 0) else 0  # Use 0 if division fails
    print(result)  # Always succeeds, prints 0 if division failed
```

#### Error Handling Best Practices

1. Use `Result` for expected errors that should be handled
2. Use `?` to propagate errors up the call stack
3. Use pattern matching for detailed error handling
4. Use `try` expressions for simple fallback values
5. All error handling patterns maintain resource safety

### 6.4 Robust External Resource Management

When working with external resources like file handles, network connections, or database connections, Ryo provides robust cleanup mechanisms:

```ryo
struct DatabaseConnection:
    _conn_handle: NativeHandle
    
    fn __new__(connection_string: str) -> Result[DatabaseConnection, ConnectionError]:
        handle = try_connect(connection_string)
        if handle:
            return Ok(DatabaseConnection(_conn_handle=handle))
        return Err(ConnectionError("Failed to connect"))
    
    fn __drop__(self):
        # Properly releases the connection even if in error state
        safe_disconnect(self._conn_handle)
```

#### Handling `__drop__` Failures

When a `__drop__` method encounters an error, Ryo provides several ways to handle it safely:

1. **Logging in `__drop__`**: Errors in `__drop__` can be logged but cannot be propagated
   ```ryo
   fn __drop__(self):
       result = self._conn_handle.close()
       if result.is_err():
           log.error(f"Failed to close connection: {result.unwrap_err()}")
   ```

2. **Pre-emptive Cleanup**: For critical resources, provide an explicit cleanup method
   ```ryo
   struct File:
       _handle: FileHandle
       _closed: bool = false
       
       fn close(self) -> Result[None, IOError]:
           if not self._closed:
               result = close_handle(self._handle)
               self._closed = true
               return result
           return Ok(None)
       
       fn __drop__(self):
           if not self._closed:
               # Best-effort cleanup, errors logged but not propagated
               _ = self.close()
   ```

3. **Deferred Cleanup**: For resources that might fail during cleanup
   ```ryo
   struct CleanupManager:
       _pending: list[Cleanable]
       
       fn add(self, resource: Cleanable):
           self._pending.append(resource)
       
       fn cleanup(self) -> list[CleanupError]:
           errors = []
           for resource in self._pending:
               result = resource.cleanup()
               if result.is_err():
                   errors.append(result.unwrap_err())
           return errors
   ```

These patterns ensure resources are cleaned up properly, even in error conditions, without compromising Ryo's deterministic resource management.

## 7. Language Interoperability

Ryo seamlessly bridges between Python's garbage collection and Rust's ownership system:

```ryo
# Python interoperability
from python import numpy  # Import from Python ecosystem

# Automatic reference management for Python objects
struct PyObject:
    _ptr: PythonPtr  # Internal pointer to Python object
    
    fn __drop__(self):
        # Properly decrements Python's reference count
        release_python_reference(self._ptr)

# Rust interoperability
from rust import serde_json  # Import from Rust ecosystem

fn parse_json(data: str) -> dict:
    # Memory-safe bridging between languages
    # Ryo handles the different memory models automatically
    return serde_json.from_str(data)
```

#### Interoperability Details

1. **Python Integration**:
   - Automatic reference counting for Python objects
   - Zero-copy data sharing when possible
   - Type conversions with minimal overhead
   - Support for Python's asyncio integration

2. **Rust Integration**:
   - Direct mapping of Ryo's ownership to Rust's
   - Zero-cost abstractions for Rust libraries
   - Support for Rust's async/await system
   - Native performance for critical sections

#### Advanced Rust Lifetime Patterns
Some Rust patterns involving complex lifetime relationships require wrapper types in Ryo:

```ryo
# Rust lifetime-parameterized types need wrappers
from rust import serde_json

# Instead of direct lifetime parameters, use Ryo's type system
struct JsonValue:
    _ptr: RustPtr[serde_json.Value]  # Internal pointer to Rust value
    
    # Safe wrapper methods
    fn get_field(self, name: str) -> ?JsonValue:
        if field_ptr := self._ptr.get_field(name):
            return Some(JsonValue(_ptr=field_ptr))
        return None
```

Complex Rust lifetime patterns are handled through:
1. Safe wrapper types that hide lifetime complexity
2. Ryo's ownership system which maps to Rust's where possible
3. Runtime checks where compile-time guarantees aren't possible

#### Python Integration Limitations

When working with Python's memory model, certain limitations apply:

```ryo
# Python object integration with explicit lifecycle
struct PyDataFrame:
    _obj: PythonObject  # Reference to Python object
    
    # Explicit cleanup to handle Python's GIL requirements
    fn __drop__(self):
        with gil_lock():  # Acquire Python's Global Interpreter Lock
            release_python_reference(self._obj)
```

Key considerations for Python interoperability:
1. Ryo respects Python's GIL when accessing Python objects
2. Large data transfers between Ryo and Python may require serialization
3. Python's dynamic nature may require runtime type checking for safety

## Real-World Example: Web Service Handler

Here's an example showing Ryo's memory management in a web service context:

```ryo
# A web service endpoint
async fn handle_request(request: Request) -> Response:
    # STEP 1: Parse JSON payload (automatic memory management)
    payload = request.json()
    
    # STEP 2: Database connection with automatic cleanup
    struct DBConnection:
        conn: NativeConnection
        
        fn __new__(uri: str) -> DBConnection:
            return DBConnection(conn=connect_to_db(uri))
        
        fn __drop__(self):
            self.conn.disconnect()  # Connection automatically closed
    
    # Create connection that will be cleaned up when function exits
    db = DBConnection("postgres://localhost/users")
    
    # STEP 3: Query database (using shared/read-only reference)
    user = db.find_user(payload.user_id)
    if not user:
        return Response(status=404)  # Early return - db still cleaned up
    
    # STEP 4: Process data
    result = process_user_data(user)  # Automatic borrowing for function call
    
    # STEP 5: Fetch from external API (async with resource management)
    data = await fetch_external_data(user.id)
    
    # STEP 6: Combine results and return
    return Response(
        status=200,
        body={"user": user, "data": data, "result": result} # TBD this is a python dict but ryo do not support it, use struct instead
    )
    # db connection automatically closed here via __drop__
```

Key memory management features demonstrated:
1. Automatic resource cleanup for database connection
2. Safe references for database queries
3. Proper cleanup on early returns and error paths
4. Async resource management for API calls
5. No memory leaks or use-after-free bugs possible

## Benefits of Ryo

1. **Simplicity**: Minimal annotations with smart defaults
2. **Deterministic Cleanup**: Resource management via Drop trait
3. **Performance**: Compile-time guarantees with minimal runtime overhead
4. **Python-like Ergonomics**: Familiar syntax with intelligent inference
5. **Web-Service Ready**: Features designed for typical web and script workloads
6. **Interoperability**: Bridges the gap between Python and Rust
7. **Null Safety**: Built-in protection against null reference errors
8. **Progressive Learning**: Start simple, add advanced features as needed

This memory management provides the safety of Rust with ergonomics closer to Python, making it ideal for web services, scripting, and other applications where both safety and productivity are important.

## Comparison: Ryo vs. Rust vs. Python

| Feature                 | Ryo                                          | Rust                                           | Python                                     |
| ----------------------- | -------------------------------------------- | ---------------------------------------------- | ------------------------------------------ |
| **Ownership Model**     | Single owner with intelligent defaults       | Single owner with explicit lifetimes           | Reference counting with garbage collection |
| **Memory Safety**       | Compile-time checks with minimal annotations | Compile-time checks with explicit annotations  | Runtime checks with GC                     |
| **Reference Types**     | Shared, mutable, owned with smart defaults   | Shared, mutable, owned with explicit lifetimes | All references are mutable by default      |
| **Null Safety**         | Compile-time Optional type checking          | Compile-time Option type checking              | Runtime None checks                        |
| **Resource Management** | Automatic deterministic cleanup (Drop)       | Automatic deterministic cleanup (Drop)         | Finalizers with GC (\_\_del\_\_)           |
| **Error Handling**      | Result types with ? operator                 | Result types with ? operator                   | Exceptions with try/except                 |
| **Performance**         | Near-Rust with Python-like ergonomics        | Maximum performance with explicit control      | Ease of use over performance               |
| **Memory Overhead**     | Low, with compiler optimizations             | Minimal                                        | Higher due to GC and boxing                |
| **Concurrency Safety**  | Thread-safe types + borrowing rules          | Send/Sync traits + borrowing rules             | GIL + manual synchronization               |
| **Initialization**      | All values properly initialized              | All values properly initialized                | Attributes can be added dynamically        |
| **Collection Safety**   | Bounds checking, optimized when possible     | Bounds checking, optimized when possible       | Runtime bounds checking                    |
| **Learning Curve**      | Moderate, progressive disclosure             | Steep learning curve                           | Gentle learning curve                      |
| **Interoperability**    | Native with both Python and Rust             | FFI for C/C++, wrappers for others             | C extensions, FFI through ctypes           |
| **Weak References**     | First-class with safety checks               | Available through Rc/Arc with Weak             | Available but can cause leaks              |
| **Memory Leaks**        | Prevented at compile time                    | Prevented at compile time                      | Possible with reference cycles             |

**Key Takeaways:**

1. **Ryo** offers Rust's memory safety with Python's ergonomics
2. **Rust** provides maximum control and performance at cost of complexity
3. **Python** emphasizes developer productivity over memory efficiency
4. **Ryo's intelligent defaults** reduce annotation burden compared to Rust
5. **Ryo's compile-time guarantees** prevent many runtime errors found in Python
6. **Ryo's progressive learning** model makes safety accessible to Python developers

This comparison highlights Ryo's position as a bridge between Python's ease of use and Rust's safety guarantees, offering a practical middle ground for developers who need memory safety without sacrificing productivity.
