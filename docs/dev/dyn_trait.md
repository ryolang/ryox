Here is a deep dive into **Dynamic Dispatch**: what it is, why your "General Purpose" language needs it, and where to put it in your roadmap.

---

### 1. What is it? (The Concept)

To understand **Dynamic Dispatch**, you first need to understand **Static Dispatch**.

#### Static Dispatch (Monomorphization)
This is what Ryo currently plans for v0.1 via Generics. The compiler knows *exactly* which function to call at compile time.

*   **Scenario:** You have a function that prints a shape.
*   **Code:** `fn print_area[T: Shape](s: T) ...`
*   **Compiler Action:** If you call this with a `Circle` and a `Square`, the compiler secretly **copy-pastes** the function twice:
    1.  `fn print_area_Circle(s: Circle)`
    2.  `fn print_area_Square(s: Square)`
*   **Pros:** Extremely fast (inlining).
*   **Cons:** Binary size bloat (code duplication).

#### Dynamic Dispatch (`dyn Trait`)
The compiler **does not know** what the concrete type is at compile time. It only knows that the object "implements the Shape trait."

*   **Scenario:** You want a list containing *both* Circles and Squares.
*   **Code:** `fn print_area(s: &dyn Shape) ...`
*   **Compiler Action:** The compiler generates **one** function. It expects a "Fat Pointer" containing two things:
    1.  A pointer to the data (the struct).
    2.  A pointer to a **vtable** (a lookup table of functions for that specific type).
*   **Runtime:** When `s.area()` is called, the program looks at the vtable to find the correct function address, then jumps to it.

---

### 2. Why is it needed? (The 3 Killer Use Cases)

If Ryo targets "Web Backend" and "CLI Tools," you effectively **cannot** skip this concept, or users will struggle.

#### A. Heterogeneous Collections
You want a list of different objects that share behavior.
*   **Without Dynamic Dispatch:** You cannot create `list[Shape]`. You can only create `list[Circle]` or `list[Square]`.
*   **With Dynamic Dispatch:** You can create `list[&dyn Shape]`. The list can hold a Circle, then a Square, then a Triangle.

#### B. Binary Size & Compilation Time
If you use Static Dispatch (Generics) for everything, your binary grows with every new type you use.
*   *Example:* `fn sort[T](list: [T])`. If you sort Integers, Floats, Strings, and Users, you get 4 copies of the sort logic in your binary.
*   *With Dynamic:* You generate the sort logic once.

#### C. Testing (Dependency Injection) — **The Critical One**
This is the "smell" we discussed earlier.
*   **Real World:** You have a `Service` that depends on a `Database`.
*   **Static Way:** `struct Service[DB: DatabaseTrait]`.
    *   This "infects" your entire codebase. Every struct that holds the Service must also be generic. This is called "Generic Soup."
*   **Dynamic Way:** `struct Service { db: &dyn DatabaseTrait }`.
    *   The Service doesn't care. At runtime, you pass `PostgresDB`. In tests, you pass `MockDB`. The struct definition looks clean.

---

### 3. What it allows (The User Experience)

It allows **Decoupling**.

It allows a plugin architecture where the main application defines a `Plugin` trait, and users load libraries at runtime that implement that trait. The main app doesn't know `MyCoolPlugin` exists when it is compiled, but it knows how to talk to it via dynamic dispatch.

---

### 4. Where should it be in the planning?

This is a hard implementation challenge. Implementing **Vtables** (Virtual Method Tables) requires careful memory layout design and ABI stability.

#### Option A: The Hard Way (True `dyn Trait`)
Implement full Vtables like Rust/C++.
*   **Roadmap:** Phase 5 (Post v0.1). It is too complex for the initial release.

#### Option B: The "Ryo" Way (Enum Dispatch) — **Recommended for v0.1**
Since Ryo is "Ownership Lite" and prioritizes DX, you can skip full Dynamic Dispatch in v0.1 by promoting **Enum Wrappers** as the official pattern.

**The Pattern:**
Instead of an interface `Database`, you define an Enum that holds the possibilities.

```ryo
# The "Dynamic" Type
enum Database:
    Postgres(PostgresDB)
    SQLite(SqliteDB)
    Mock(MockDB)

# The Dispatch Logic
impl Database:
    fn query(self, sql: str):
        match self:
            Database.Postgres(db): db.query(sql)
            Database.SQLite(db):   db.query(sql)
            Database.Mock(db):     db.record(sql)
```

**Why this works for v0.1:**
1.  **Solves the Collection problem:** You can have `list[Database]`.
2.  **Solves the Testing problem:** You can inject `Database.Mock`.
3.  **Solves the Implementation problem:** You already have Enums and Pattern Matching in the roadmap (Phase 2). You get this feature "for free" without writing complex Vtable logic in the compiler.

#### Recommendation for the Roadmap

1.  **Phase 2 (Enums):** Ensure Enums are robust enough to hold Structs (Algebraic Data Types).
2.  **Documentation:** Write a guide on "Polymorphism in Ryo" explaining the Enum Pattern for testing and collections.
3.  **Phase 5 (Future):** Add true `dyn Trait` (Dynamic Dispatch) later for cases where Enums are too rigid (e.g., user-defined plugins).

**Verdict:** Do **not** add `dyn Trait` / Vtables to the v0.1 roadmap. It will delay the release. Use **Enums** to solve the user need instead.
