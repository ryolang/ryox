# Deep Guide


## Imports

1. **Agrupar Módulos Relacionados** 
   ```python
   import utils.{math, strings}  # Importa math y strings del paquete utils
   ```
   - **Para qué**: Evitar repetición al importar submódulos de un mismo paquete. 
   - **Ejemplo**: 
     ```python
     math.suma(2, 3)
     strings.capitalizar("hola")
     ```

2. **Reflejar la Estructura del Proyecto** 
   ```python
   import api.v1.users  # Corresponde a api/v1/users.ryo
   ```
   - **Para qué**: Mantener jerarquías claras y acceder directamente a submódulos profundos. 
   - **Ejemplo**: 
     ```python
     usuarios = users.obtener_todos()
     ```

3. **Dependencias Externas con Prefijo** 
   ```python
   import pkg:requests  # Paquete externo instalado (no local)
   ```
   - **Para qué**: Diferenciar código local de paquetes externos y evitar colisiones. 
   - **Ejemplo**: 
     ```python
     respuesta = requests.get("https://api.com")
     ```

### **Ejemplo Completo** 

```python
# main.ryo
import utils.{math, strings}
import api.v1.users
import pkg:requests

func main():
    print(math.suma(2, 3))                # 5
    nombre = strings.capitalizar("juan")   # "Juan"
    usuarios = users.obtener_todos()       # Función desde api/v1/users.ryo
    respuesta = requests.get("https://...")
```

---

#### **Estructura del Proyecto** 
```
mi_app/
├── ryo.toml
└── src/
    ├── main.ryo
    ├── utils/
    │    ├── math.ryo        # package utils
    │    └── strings.ryo     # package utils
    └── api/
        └── v1/
            └── users.ryo    # package api.v1.users
```

## Structs and Traits, but no Classes or Interfaces.

This design direction is deliberate and aims to balance several key objectives for Ryo:

*   **Python-like Simplicity and Data Focus:** Structs in Ryo will serve a purpose similar to classes in Python in terms of data aggregation. Structs are primarily about grouping data fields together in a structured way. This aligns with Python's emphasis on data and relatively simple object creation for data holding.
*   **Rust-like Safety and Code Reusability:** Traits are adopted from Rust precisely for their powerful mechanism for code reuse, abstraction, and ensuring type safety. Traits allow us to define shared behaviors that can be implemented by different data types (structs and potentially enums in Ryo), promoting code modularity and polymorphism without the complexities of traditional class inheritance.
*   **Avoiding Class Inheritance Complexities:**  The decision to *omit classes* is a conscious choice to avoid the complexities and potential pitfalls often associated with class-based inheritance hierarchies. Class inheritance, while a cornerstone of traditional object-oriented programming, can lead to:
    *   **Fragile Base Class Problem:** Changes in base classes can unexpectedly break derived classes.
    *   **Deep and Inflexible Hierarchies:** Inheritance can lead to rigid and difficult-to-refactor class hierarchies.
    *   **"Gorilla/Banana Problem":** "You wanted a banana but what you got was a gorilla holding the banana and the entire jungle." - Inheritance can sometimes pull in more functionality than you actually need, leading to complexity.

*   **Favoring Composition over Inheritance:** Modern language design trends (and Rust's successful example) often favor **composition over inheritance**. Traits are a prime example of a composition-based approach. Instead of inheriting from a base class, structs can *compose* behaviors by implementing traits. This leads to more flexible and modular code.
*   **Simplified Object Model:** By having just structs for data and traits for behavior, Ryo aims for a simpler and more predictable object model compared to languages with both classes and interfaces.  This can make the language easier to learn, reason about, and maintain.


**Structs in Ryo:**

*   **Purpose:**  Primarily for data aggregation.  Think of them as custom data structures.
*   **Similar to:**  Python classes (in terms of data holding), C structs, Rust structs.
*   **Features:**  Will contain fields/members to hold data of various types.  Likely will have methods associated with structs (functions that operate on struct instances).
*   **Example (Conceptual Ryo):**

```ryo
struct Point:
    x: Int
    y: Int

    # Methods associated with Point struct
    fn distance_from_origin(self) -> float:
        return sqrt(self.x*self.x + self.y*self.y)

p = Point{x: 3, y: 4 }
dist = p.distance_from_origin()
print("Distance: {dist}")
```

**Traits in Ryo:**

*   **Purpose:**  To define shared behavior (methods) that different types can implement.  To achieve polymorphism and code reuse. To define contracts.
*   **Similar to:** Rust traits, Java interfaces, Go interfaces, TypeScript interfaces.
*   **Features:** Define a set of method signatures (names and types). Structs (and potentially enums) can implement traits.
*   **Example (Conceptual Ryo):**

```ryo
trait Drawable:
    fn draw(self) -> str # Method signature - returns a String representation

struct Circle:
    radius: float

impl Drawable for Circle:
    fn draw(self) -> str:
        return "Drawing a circle with radius {self.radius}"

struct Square:
    side: float

impl Drawable for Square:
    fn draw(self) -> str:
        return "Drawing a square with side {self.side}"

fn print_drawing(drawable: impl Drawable): # Polymorphic function - accepts any type that implements Drawable
    print(drawable.draw())

c = Circle{ radius: 5.0 }
s = Square{ side: 4.0 }

print_drawing(c) # Works because Circle implements Drawable
print_drawing(s) # Works because Square implements Drawable
```

**Ryo `Drop` Feature (Deterministic Resource Management):**

Ryo have a mechanism similar to Rust's `Drop` trait.  The key principles will likely be:

*   **Deterministic Cleanup:** Ryo will provide a way to define code that is guaranteed to be executed when an object is no longer needed (goes out of scope, is explicitly dropped). This is crucial for deterministic resource management.
*   **Resource Safety:** This feature will enable safe management of resources like:
    *   Memory (though Ryo's borrowing system will largely handle memory safety, `Drop` can be used for manual memory management in very specific scenarios, if needed in the future or for advanced use cases).
    *   File handles (closing files reliably).
    *   Network connections (closing sockets).
    *   External system resources.
*   **Automatic Invocation (Likely tied to Scope):**  Similar to Rust's `Drop`, the cleanup code will likely be automatically invoked when an object goes out of scope (the indentend block where it was defined ends).
*   **Preventing Resource Leaks:**  Guarantees that resources are released promptly and predictably, preventing resource leaks, which can be especially important in performance-sensitive applications or long-running processes.
*   **No Reliance Solely on Garbage Collection (for resource cleanup):** Unlike languages that rely solely on garbage collection for resource finalization, Ryo will offer deterministic control, which is beneficial for performance predictability and managing resources that are not just memory (like file handles).

**Example of a `Drop` feature in Ryo :**

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
    print("Processing file: {file.filename}")
    # ... Use the file ...
    # file will be automatically dropped at the end of this function's scope,
    # and the drop() function in impl Drop for FileHandler will be executed,
    # ensuring the file is closed.

```

**Summary of Ryo's Object Model and Resource Management:**

Ryo's object model is designed to be:

*   **Simple:** Structs for data, Traits for behavior.  No classes or interfaces to simplify the language.
*   **Safe:**  Static typing, borrowing system, and a `Drop`-like feature contribute to memory safety and resource safety.
*   **Performant:**  Compiled language, borrowing system (no GC pauses), deterministic resource management.
*   **Modern:**  Embraces composition over inheritance, traits for code reuse, deterministic resource cleanup – all are modern language design principles.

By combining structs and traits, and incorporating a `Drop`-like feature, Ryo aims to provide a powerful, safe, and efficient programming model that is both approachable and capable of handling complex software development tasks.  This approach draws valuable lessons from both Python's data-centric simplicity and Rust's safety and performance focus.

## Enums
Here's a recap of **Ryo's Algebraic Data Types (ADTs)** using enums, based on our previous design discussions:

---

### **Ryo Enum Syntax**
```ryo
# Basic definition
enum Result[T, E]:
    Ok(T)
    Err(E)

# Complex variant example TDB
enum Expression:
    Literal(int)
    BinaryOp:
        lhs: Box<Expression>
        op: Op
        rhs: Box<Expression>
    Variable(String)

```

### **Key ADT Features**
1. **Variants**:
   - Unit: `VariantName`
   - Tuple: `VariantName(T1, T2)`
   - Struct-like: `VariantName { field: Type }`

2. **Pattern Matching**:
   ```ryo
   fn handle_result(res: Result[int, str]) -> str:
       match res:
           Ok(value) => f"Success: {value}",
           Err(msg) => f"Error: {msg}",
   ```

3. **Type Safety**:
   - Compile-time exhaustiveness checks
   - No implicit fallthrough
   - Nested destructuring:
     ```ryo
     match expr:
         Expression.BinaryOp { lhs, op, rhs } => # ...
     ```

4. **Generic Support**:
   ```ryo
   enum Option[T]:
       Some(T)
       None
   ```

---

### **Standard Library ADTs**
1. **`Result[T, E]`**  
   ```ryo
   enum Result[T, E]:
       Ok(T)
       Err(E)
   ```

2. **`Option[T]`**  
   ```ryo
   enum Option[T]:
       Some(T)
       None
   ```

3. **`Err` Hierarchy**  
   ```ryo
   enum Err:
       Io:
            path: str
            code: int
       Parse:
            line: int
            message: str
       Runtime(str)
   ```

### **Key Design Choices**
1. **Exhaustive Matching**:  
   ```ryo
   # Compiler error if any variant is unhandled
   match option_val:
       Some(x): 
            # ...,
       None:
            # ...,
   ```

2. **No Nulls**:  
   - `Option[T]` replaces null checks
   - Compiler-enforced handling of `None`


## Error Handling with `Result[T, E]`

Robust programs need to handle errors gracefully. Ryo provides the `Result[T, E]` type for explicit error handling.  `Result` is a type that represents either a successful outcome with a value of type `T` (represented as `Ok(T)`) or a failure with an error value of type `E` (represented as `Err(E)`).

### The `Result[T, E]` Type

Think of `Result` as a container that can hold one of two things:

*   **`Ok(value)`:**  Indicates success. `value` is the successful result of the operation.
*   **`Err(error_value)`:** Indicates failure. `error_value` is a value representing the error that occurred.

`T` represents the type of the successful value, and `E` represents the type of the error value. You need to specify these types when you use `Result`.

### Functions that can Fail: Returning `Result`

When you define a function that might encounter errors (like reading a file, making a network request, or parsing data), you should make it return a `Result` type.

**Example: A function that might fail to divide**

```ryo
fn divide(numerator: float, denominator: float) -> Result[float, Err]:
    if denominator == 0.0:
        return Err("Cannot divide by zero") # Indicate an error as Err(string)
    else:
        return Ok(numerator / denominator) # Indicate success with Ok(result)
```

*   **`-> Result[float, Err]`:**  This specifies the return type as `Result`.
    *   `float` is the type of the successful value (the result of division).
    *   `Err` is the type of the error value (an error message string).
*   **`return Err("Cannot divide by zero")`**:  If division by zero is attempted, the function returns `Err` containing an error message string.
*   **`return Ok(numerator / denominator)`**: If the denominator is not zero, the function returns `Ok` containing the result of the division.


### Try Expressions for Local Error Handling

Ryo provides a convenient `try` expression for handling errors locally without having to use a full `match` statement. The `try` expression allows you to attempt an operation that returns a `Result` and provide a fallback value if it fails.


```ryo
fn safe_operation():
    result = try divide(10, 0) else 0  # Use 0 if division fails
    print(result)  # Always succeeds, prints 0 if division failed
```

#### **Bajo el Capó (Equivalente en Ryo)**

La expresión `try X or Y` se traduciría a:

```python
temp = X
match temp:
    Ok(v) -> v
    Err(_) -> Y
```

#### **Reglas Clave**
1. **No captura errores críticos**: Si necesitas manejar errores específicos, usa `catch` + `match`.
2. **No permite ignorar errores silenciosamente**: El error se consume, pero puedes registrar advertencias.
```ryo
user = try get_user(id) or:
    print("User not found, using guest")
    return guest_user()

```

#### **Implementación Detallada**
##### 1. **Tipado Estricto**
El compilador debe asegurar que el tipo después de `or` coincida con `T` en `Result[T, E]`:
```python
# ✅ Válido: "backup" es str (mismo que Ok)
mensaje = try leer_archivo("msg.txt") or "backup"

# ❌ Error: 0 es int, pero se esperaba str
mensaje = try leer_archivo("msg.txt") or 0
```

##### 2. **Uso con Bloques**
Si el valor alternativo requiere lógica compleja, usa un bloque `:`:
```ryo
port = try parsear_puerto(entrada) or:
    print("Puerto inválido, usando 8080")
    return 8080

```

##### 3. **Interacción con `Drop`**
Los recursos dentro del `try` se liberan correctamente (RAII):
```ryo
conn = try conectar_db() or conexion_alternativa()
# Si conectar_db() falla, cualquier recurso intermedio se libera via __drop__
```

---

#### **Código de Ejemplo Completo**
```ryo
error ParseError:
    InvalidFormat
    Overflow

def parsear_entero(s: str) -> Result[int, ParseError]:
    if not s.isdigit():
        return Err(ParseError.InvalidFormat)
    return Ok(int(s))

def main():
    entrada = "123a"
    numero = try parsear_entero(entrada) or:
        print("Entrada inválida, usando 0")
        return 0
    print(numero)  # Imprime 0
```

### Handling `Result` Values with `match`

To use the result of a function that returns `Result`, you need to check whether it was successful (`Ok`) or if it failed (`Err`).  The `match` expression is the perfect way to do this in Ryo.

**Example: Handling the result of `divide`**

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

*   **`match result1:`**:  The `match` expression takes a `Result` value (`result1` in this case) and checks its variant (either `Ok` or `Err`).
*   **`Ok(value):`**: This is a **pattern** that matches if `result1` is an `Ok` variant.
    *   `Ok(value)` also **binds** the successful value inside the `Ok` to the variable `value`.  You can then use `value` in the code block under this pattern.
    *   `print(f"Division successful, result: {value}")`: This code block is executed if `result1` is `Ok`.
*   **`Err(error_message):`**: This is a pattern that matches if `result1` is an `Err` variant.
    *   `Err(error_message)` binds the error value inside `Err` to the variable `error_message`.
    *   `print(f"Division failed: {error_message}")`: This code block is executed if `result1` is `Err`.

When you run this code, you'll see output like:

```
Division successful, result: 5.0
Division failed: Cannot divide by zero
```

### Error Propagation

Error Propagation with `try`:


```ryo
fn process_data() -> Result[str, E]:
    data = try fetch_data() # Propagate error from fetch_data if it returns Err
    processed_data = try transform_data(data) # Propagate error from transform_data if it returns Err
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


operation_result = process_data()
match operation_result:
    Ok(final_result):
        print("Operation successful:", final_result)
    Err(final_error):
        print("Operation failed:", final_error)
```
* `try expresion`: If expression is `Result[T, E]`:
  * If Ok(value), unwraps to value.
  * If Err(error), returns Err(error) from the current function.
* Reduces verbosity: Simplifies error propagation compared to using match every time.
* Function return type: The function using `try` must return a compatible Result[_, ErrorType].

**Example: Function that uses `divide` and propagates errors**

```ryo
fn process_division(num1: float, num2: float) -> Result[str, Err]:
    division_result = divide(num1, num2) # Call divide, which returns a Result
    match division_result:
        Ok(result_value):
            return Ok(f"Processed division result: {result_value}") # Success, return Ok with a message
        Err(division_error):
            return Err(f"Error in process_division: {division_error}") # Propagate the error as Err
```

If `divide` returns `Err`, `process_division` will also return `Err`, effectively passing the error up the call stack.  If `divide` returns `Ok`, then `process_division` will perform its processing and return `Ok` indicating its own success.

### Error Handling in `async/await`

`Result` works seamlessly with `async/await`.  Asynchronous functions can also return `Result`, and you can use `await` with functions that return `Future[Result[T, E]]`.  The `await` expression will then return a `Result[T, E]`.

**Example: Asynchronous function returning `Result`**

```ryo
async fn fetch_content_result(url: str) -> Result[str, E]: # Async function returning Result
    print(f"Fetching from {url}...")
    # Assume 'async_http_get_result' is an async function that returns Future[Result[str, E]]
    fetch_future_result = async_http_get_result(url)
    content_result = await fetch_future_result # Await the Future - 'content_result' is now a Result[str, E]

    match content_result:
        Ok(content):
            print("Fetch successful.")
            return Ok(content) # Return Ok(content)
        Err(http_error):
            print(f"Fetch failed: {http_error}")
            return Err(f"Network error: {http_error}") # Propagate error as Err
```

**Example: Handling `Result` from an `async` function in `main`** TDB

```ryo
async fn main() -> Result[(), Err]: # main can also return Result to indicate program success or failure
    fetch_result = await fetch_content_result("https:#example.com") # await async function that returns Result
    match fetch_result:
        Ok(content):
            print("Fetched content:", content.substring(0, 50), "...") # Print first 50 chars
            return Ok(()) # Indicate main function success with Ok(()) - Unit type
        Err(fetch_err):
            print("Error fetching content in main:", fetch_err)
            return Err(fetch_err) # Propagate error from main - program ends in failure

run_async main() # Run async main function
```

In `async fn main() -> Result[(), Err] : ... `,  `main` itself is defined to return a `Result`.  `Ok(())` is used to indicate successful completion of the `main` function (returning a Unit type `()` within `Ok`). `Err(fetch_err)` propagates an error from `main`, indicating that the program ended with a failure.

By using `Result[T, E]`, you can make your Ryo programs more robust by explicitly handling potential errors and making error management a core part of your code. Remember to use `match` to properly handle `Result` values and think about how errors should be propagated and handled in your programs.

## Asynchronous Programming with `async/await` in Ryo

Ryo supports asynchronous programming using `async` and `await`. This allows you to write code that can perform long-running operations, like network requests or file I/O, without blocking the program's execution.

*   **`async fn`**:  To define an asynchronous function, use the `async fn` keywords. These functions implicitly return a `Future` (though you don't have to write `Future` explicitly in the return type annotation in most cases).

    ```ryo
    async fn fetch_data_from_web(url: str) -> str: # Define an async function
        print(f"Fetching data from {url}...")
        # Imagine 'http_get_async' is a function that performs an async network request
        data = await http_get_async(url)
        print("Data fetched!")
        return data
    ```

*   **`await`**:  Inside an `async fn`, you use the `await` keyword to pause execution until a `Future` (representing an asynchronous operation) completes.

    ```ryo
    async fn main():
        data1 = await fetch_data_from_web("https:#example.com/api/data1") # Await the result
        print("Data 1:", data1)

        data2 = await fetch_data_from_web("https:#example.com/api/data2") # Await another result
        print("Data 2:", data2)

        print("All data fetched and processed.")

    run_async(main) # Run the async main function
    ```
    Note: run_async is performing an implicit, top-level "await" on the Future returned by the async main function. It's what makes the asynchronous program actually run and complete within the synchronous environment where the program execution begins.

*   **Important Safety Rule: No Mutable Borrows Across `await`**

    Ryo has a strict rule to help prevent data races in asynchronous code: **You cannot have mutable borrows that span across `await` points.**

    This means if you borrow a variable mutably, and then you use `await`, you cannot continue using that mutable borrow after the `await`. The Ryo compiler will catch this as an error.

    **Example of an Error:**

    ```ryo
    async fn problematic_function():
        let mut counter = 0
        let mutable_counter = &mut counter # Mutable borrow starts
        counter = counter + 1 # OK to use mutable_counter here

        await some_async_operation() # Suspension point

        # ERROR! Cannot use 'mutable_counter' here after 'await' because it's a mutable borrow that spanned across the 'await'
        # mutable_counter = mutable_counter + 1 # This will cause a compile error in Ryo
    ```

    **How to Fix it:**  To avoid this error, restructure your code so that mutable borrows do not cross `await` points. You might need to perform mutations before the `await` or re-borrow after the `await` if needed, being very careful about data access.  In simpler scenarios, just work with values directly and avoid mutable references spanning `await` calls.

*   **Limitations of Concurrency in Ryo (Simplified Model):**

    It's important to understand that in this simplified version of Ryo, concurrency is primarily focused on asynchronous I/O within a single task. Ryo **does not provide channels or mutexes** for more complex forms of concurrency, like safe sharing of mutable data between concurrent tasks or parallel processing.

    For simple asynchronous operations, `async/await` in Ryo provides a clean and relatively safe way to write non-blocking code. However, for applications needing more advanced concurrency patterns, Ryo in this simplified form is **not designed to be a robust solution.** Fibers will be implemented in future versions of Ryo.