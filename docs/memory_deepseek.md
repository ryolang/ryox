### **Memory Management System Design for Ryo **

To create a memory-safe, easy-to-understand system with compile-time guarantees, we’ll blend concepts from Rust (ownership), Kotlin (null safety), and ryo (simplicity). Here’s the design:

---

#### **1. Ownership Model with Move Semantics**
- **Core Rule**: Each value has a single owner. Assigning a variable transfers ownership, invalidating the original.
  ```ryo
  a = [1, 2, 3]  # `a` owns the list
  b = a           # Ownership moves to `b`; `a` is now invalid
  print(b)        # OK: [1, 2, 3]
  print(a)        # Compile error: "Value moved to `b`"
  ```
- **Explicit Cloning**: Copying requires an explicit `.clone()`, preventing accidental duplication.
  ```ryo
  c = b.clone()   # `c` gets a copy; `b` remains valid
  ```

---

#### **2. Borrowing with Lifetime Inference**
- **Immutable Borrows**: Allow temporary read-only references without ownership transfer.
  ```ryo
  func sum(lst: &[int]) -> int:  # Borrows `lst` immutably
      total = 0
      for num in lst:
          total += num
      return total

  nums = [1, 2, 3]
  result = sum(&nums)  # `nums` remains valid after the call
  ```
- **Mutable Borrows**: Allow temporary mutable references, enforced as non-overlapping.
  ```ryo
  func append(lst: &mut [int], val: int):
      lst.push(val)

  append(&mut nums, 4)  # `nums` is now [1, 2, 3, 4]
  ```
- **Lifetimes Inferred**: Compiler tracks reference validity without explicit annotations.

---

#### **3. Null Safety with Optional Types**
- **No Implicit Nulls**: Variables can’t be `null` unless declared as `Optional[T]`.
  ```ryo
  name: str = "Alice"    # Guaranteed non-null
  maybe_name: Optional[str] = None  # Must be checked before use
  ```
- **Compile-Time Checks**: Force unwrapping of `Optional` values safely.
  ```ryo
  if maybe_name is not None:
      print(maybe_name.unwrap())
  else:
      print("No name")
  ```

---

#### **4. Automatic Compile-Time Reference Counting (CRC)**
- **No Runtime Overhead**: Compiler statically ensures each allocation has exactly one owner or valid borrows.
- **Cycle Detection**: Compiler rejects code with ownership cycles (e.g., two structs owning each other).

---

#### **5. Bounds and Type Safety**
- **Fixed-Size Arrays**: Sizes known at compile time enable static bounds checks.
  ```ryo
  arr: [int; 3] = [1, 2, 3]
  print(arr[5])  # Compile error: "Index 5 out of bounds (size 3)"
  ```
- **Dynamic Collections**: Use runtime checks with compiler-inserted guards.
  ```ryo
  dynamic_list = List[int]()
  dynamic_list.push(10)
  print(dynamic_list[1])  # Runtime error: "Index out of bounds"
  ```

---

#### **6. Memory Cleanup via Scopes**
- **Deterministic Destruction**: Values are dropped when their owner goes out of scope.
  ```ryo
  def process_data():
      data = load_data()  # Owned by `data`
      # ... use `data` ...
  # `data` is automatically freed here
  ```

---

### **Example Program**
```ryo
# Ownership and borrowing
def main():
    scores = [90, 85, 95]  # Owned by `scores`
    total = sum(&scores)    # Immutable borrow
    print(total)            # 270

    add_extra(&mut scores, 5)  # Mutable borrow
    print(scores)           # [95, 90, 100]

# Compiler ensures `list` is valid for the borrow duration
def add_extra(list: &mut [int], bonus: int):
    for i in 0..len(list):
        list[i] += bonus
```

---

### **Key Features**
| **Aspect**          | **Mechanism**                                     | **Safety**                            |
| ------------------- | ------------------------------------------------- | ------------------------------------- |
| **Ownership**       | Move semantics + explicit cloning                 | Prevents use-after-free, double frees |
| **Borrowing**       | Immutable/mutable references + lifetime inference | No data races, dangling pointers      |
| **Null Safety**     | `Optional[T]` + compile-time checks               | Eliminates `null` dereferences        |
| **Bounds Checking** | Static (fixed-size) + dynamic (runtime) guards    | Prevents buffer overflows             |
| **Cleanup**         | Scope-based destruction                           | No memory leaks                       |

---

### **Why It Works**
1. **Familiar Syntax**: ryo-like structure with minimal annotations.
2. **Compiler Guidance**: Clear error messages explain ownership violations, null checks, or bounds issues.
3. **No Runtime Overhead**: Most checks (ownership, nulls, static bounds) are compile-time.
4. **Gradual Learning**: Developers can start with basic ownership and adopt borrowing as needed.

This system prioritizes **ease of use** while enforcing **memory safety** and **compile-time guarantees**, making it ideal for developers transitioning from ryo or similar languages.