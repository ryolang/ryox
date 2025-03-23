# Ryo Cheat Sheet 

This cheat sheet provides a concise overview of the Ryo programming language syntax and core features. It is intended as a quick reference for developers already familiar with programming concepts.

**Note:** Ryo is a simplified language focusing on readability and memory safe. Its concurrency model is limited to `async/await` with strict borrow checking. Ryo collections (lists and maps), structs and enums are **homogeneous** and do not support mixed data types where it's not explicitly defined for enums.

---

## 1. Basic Syntax

**Comments:**

```ryo
# This is a single-line comment
```

**Functions:**

```ryo
fn function_name(parameter1: type, parameter2: type) -> return_type:
	# function body (indented with tabs)
	statement1
	statement2
	return value  # Optional, depending on return_type
```

*   `fn` keyword to define a function.
*   `parameter: type` for typed parameters.
*   `-> return_type` to specify the return type (optional if no explicit return or Unit type - implicit for now).
*   `:` colon starts the tab-indented function body.
*   Indentation (**tabs**) defines code blocks. **Use a single tab for each indentation level.**

**`main` Function (Entry Point):**

```ryo
fn main():
	# program starts execution here (tab-indented)
	statement
	# ...
```

**Implicit Variable Declaration (within functions):**

```ryo
variable_name = value # Type inferred from 'value'
```

**Print Statement:**

```ryo
print(value)
print(f"Formatted string with {variable}") # f-strings
```

---

## 2. Data Types

*   **`int`**: Integer numbers (e.g., `10`, `-5`, `0`).
*   **`float`**: Floating-point numbers (e.g., `3.14`, `-2.5`, `0.0`).
*   **`bool`**: Boolean values: `true`, `false`.
*   **`str`**: Strings (e.g., `"Hello"`, `"Ryo"`).
*   **`Result[T, E]`**: Represents either success (`Ok(T)`) or failure (`Err(E)`).
*   **`list[T]`**: Ordered collection of items of type `T`. (e.g., `list[int]`, `list[str]`). **Lists are homogeneous - all elements must be of type `T`.**
*   **`map[K, V]`**: Collection of key-value pairs, where keys are of type `K` and values are of type `V`. (e.g., `map[str, int]`, `map[str, list[str]]`). **Maps are homogeneous - keys must all be of type `K`, and values must all be of type `V`.**
*   **`struct StructName`**: User-defined data structure to group fields of different types (but each field within a struct has a specific type).
*   **`enum EnumName`**: User-defined type that can be one of several named variants, each variant potentially holding data.

---

## 3. Operators

**Arithmetic:**

*   `+` (addition), `-` (subtraction), `*` (multiplication), `/` (division), `%` (modulo)

**Comparison:**

*   `==` (equal), `!=` (not equal), `<` (less than), `>` (greater than), `<=` (less or equal), `>=` (greater or equal)

**Logical:**

*   `and`, `or`, `not`

**List/Map Specific (Illustrative - Subject to Refinement):**

*   **List Indexing:** `list_variable[index]` (for accessing elements)
*   **Map Indexing (Key-based Access):** `map_variable[key]` (for accessing values by key), `map_variable[key] = value` (for setting/updating values)
*   **Struct Field Access:** `struct_instance.field_name` (to access fields of a struct instance)

---

## 4. Control Flow

**`if` / `elif` / `else` Statements:**

```ryo
if condition1:
	# code to execute if condition1 is true (tab-indented)
elif condition2:
	# code to execute if condition1 is false AND condition2 is true (tab-indented)
else:
	# code to execute if all conditions are false (tab-indented)
```

**`for` Loop (range-based and List Iteration):**

```ryo
for variable in range(start, end): # Iterates from 'start' up to (but not including) 'end'
	# code to execute for each value in the range (tab-indented)
	print(variable)

for item in list_variable: # Iterates over each item in a list
    # code to execute for each item (tab-indented)
    print(item)
```

*   **List Iteration:**  The `for...in` loop now also supports iterating directly over elements in a `list`.

---

## 5. Error Handling (`Result[T, E]`)

**Function Returning `result`:**

```ryo
fn fallible_function() -> result<return_type, error_type>:
	# ... code that might fail ... (tab-indented)
	if success_condition:
		return Ok(success_value)
	else:
		return Err(error_value)
```

**Handling `result` with `match`:**

```ryo
result_value = fallible_function()
match result_value:
	Ok(value):
		# Code to execute if successful ('value' is the successful result) (tab-indented)
		print(f"Success: {value}")
	Err(error):
		# Code to execute if there was an error ('error' is the error value) (tab-indented)
		print(f"Error: {error}")
```

**`match` for Enums:**

```ryo
enum_variable = ... # some enum value
match enum_variable:
    Variant1:
        # Code for Variant1 (tab-indented)
        print("It's Variant1")
    Variant2(data): # Matching a variant with associated data
        # Code for Variant2, 'data' holds the associated value (tab-indented)
        print(f"It's Variant2 with data: {data}")
    # ... more variants ...
    _ : # Optional wildcard/default case
        print("It's some other variant")
```

*   **`match` with Enum Variants:** The `match` expression can also be used to handle different variants of an `enum`. You can match specific variants and extract data associated with variants. The `_` pattern acts as a wildcard to match any variant not explicitly listed.

---

## 6. Asynchronous Programming (`async`/`await`)

**`async func` (Asynchronous Function):**

```ryo
async func async_function_name() -> return_type: # or -> Future[return_type> if explicit
	# Asynchronous function body (tab-indented)
	result = await some_async_operation() # Suspend execution until 'some_async_operation' completes
	return result
```

*   `async func` keyword to define asynchronous functions.
*   Implicitly returns a `Future[return_type>`.

**`await` (Suspension Point):**

```ryo
value = await future_expression # Pauses execution until 'future_expression' (Future) resolves.
```

**Running `async main`:**

```ryo
async func main() -> result<(), str>: # main can also return result
	# ... async code ... (tab-indented)
	return Ok(()) # or Err(...) for error
run_async main()
```

**Important Safety Rule (Borrow Checker):**

*   **No Mutable Borrows Across `await`:**  Mutable borrows **cannot** span across `await` points. The compiler will prevent this to avoid potential data races.
*   **Shared Immutable Borrows OK:** Immutable borrows are allowed across `await` points.

**Limitations of Concurrency in Ryo (Simplified Model):**

*   **`async/await` only**: No channels, no mutexes.
*   Primarily for asynchronous I/O within a single task.
*   Not designed for complex parallel processing or shared mutable state concurrency.

---

## 7. Data Type Literals & Initialization (Illustrative - Syntax to be Refined)

**List Literals:**

```ryo
my_list: list[int] = [1, 2, 3, 4, 5] # List of integers
string_list: list[str] = ["apple", "banana", "cherry"] # List of strings
empty_list: list[int] = [] # Empty list (explicit type annotation recommended for empty collections)
```

*   Square brackets `[]` are used to define list literals, similar to Python. **Lists in Ryo are homogeneous - all elements must be of the same type.** Explicit type annotation `list[T]` is recommended, especially for empty lists.

**Map Literals:**

```ryo
string_int_map: map[str, int] = {"Alice": 30, "Bob": 25, "Charlie": 40} # Map with string keys and integer values
string_string_map: map[str, str] = {"name": "Ryo", "creator": "Bard", "type": "Language"} # Map with string keys and string values
empty_map: map[str, str] = {} # Empty map (explicit type annotation recommended for empty collections)
```

*   Curly braces `{}` are used to define map literals, with key-value pairs separated by colons `:`, similar to Python and JavaScript. **Maps in Ryo are homogeneous - all keys must be of the same type, and all values must be of the same type.**  **Map literals should have explicit type annotations like `map[K, V]` for clarity, especially for empty maps.**

**Struct Definition & Instantiation:**

```ryo
struct Point: # Define a struct named Point
    x: int  # Field 'x' of type int (tab-indented)
    y: int  # Field 'y' of type int (tab-indented)

my_point: Point = Point { x: 10, y: 20 } # Create an instance of Point (struct literal)
another_point: Point = Point { x: -5, y: 0 }

# Struct field access:
x_coordinate = my_point.x # Access the 'x' field
my_point.y = 25          # Modify the 'y' field (structs are mutable in this initial version)
```

*   `struct StructName:` keyword to define a struct.
*   Indented lines within the `struct` block define fields with `field_name: type`.
*   Struct instantiation using `StructName { field1: value1, field2: value2 }` (struct literal).
*   Field access using dot notation: `instance.field_name`. **Structs are mutable in this version.**

**Enum Definition & Usage:**

```ryo
enum Option[T]: # Define a generic enum Option, parameterized by type T
    Some(T) # Variant 'Some' holding data of type T (tab-indented)
    None    # Variant 'None' (tab-indented)

some_value: Option[int] = Some(42) # Create an Option with a value
no_value: Option[int] = None        # Create an Option representing no value

result_option: result<Option[str], str> = fallible_operation() # Example returning result containing an Option

match result_option:
    Ok(option_value):
        match option_value: # Nested match to handle the Option inside Result
            Some(text):
                print(f"Operation successful, Option contains: {text}")
            None:
                print("Operation successful, Option is None (no value)")
    Err(error_message):
        print(f"Operation failed: {error_message}")
```

*   `enum EnumName[TypeParameters]:` keyword to define an enum. Type parameters (like `[T]`) are optional for enums that are not generic.
*   Indented lines within the `enum` block define variants.
    *   `VariantName`: Simple variant with no associated data.
    *   `VariantName(DataType)`: Variant holding data of `DataType`.
*   Enum variants are used as constructors: `VariantName` or `VariantName(value)`.
*   Use `match` expression to handle different enum variants.

---


## 8.  Important Notes

*   **Indentation Matters:** Code blocks are defined by **tabs**. Use a single tab for each indentation level. **Do not use spaces for indentation.**
*   **Static Typing:** Ryo is statically typed. Type annotations are used in function signatures, for list/map/enum/struct types (using `list[T]`, `map[K, V]`, `enum EnumName[T]`, `struct StructName`), and for struct fields and enum variant data. Variable types are often inferred.
*   **Error Handling is Explicit:** Use **`Result[T, E]`** for functions that can fail and `match` to handle `result` values.
*   **Simplified Concurrency:** Ryo's concurrency is limited to `async/await` with borrow checker restrictions.  It is not intended for highly concurrent or parallel applications in this simplified version.
*   **Ryo collections (lists and maps) are homogeneous and cannot contain elements of mixed types within a single instance.** This applies to both lists and maps (values within a map, as well as elements within a list).


---

**This cheat sheet is a starting point.  Refer to the full Ryo documentation (when available) for complete details.**



