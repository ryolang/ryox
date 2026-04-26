# Ryo v0.4 Concurrency Implementation Plan
> Task / Future / Channel — Green Thread M:N Runtime

---

## Overview

This document describes the phased implementation plan for Ryo's concurrency
model. The goal is a working, cross-platform (Linux, macOS, Windows) M:N green
thread runtime backing `task.run`, `future[T]`, and channels by the v0.4
release.

**Core stack:**
- `corosensei` — cross-platform stack switching
- `mio` — cross-platform I/O polling (epoll / kqueue / IOCP)
- `crossbeam-deque` — work-stealing task queues
- Custom scheduler — M:N green thread dispatch

**Design constraints inherited from spec:**
- No function coloring — tasks look like synchronous code to the user
- Ambient runtime via TLS — no runtime handle passed by caller
- Dropping a future cancels its task
- Cooperative cancellation at suspension points only
- RAII cleanup guaranteed on cancellation (stack unwind)

---

## Phase 1 — Foundation (Single-Threaded)

**Goal:** Get a single green thread switching correctly on all three platforms.
No scheduler, no I/O, no channels. Just prove the stack model works.

### 1.1 Stack Abstraction

- Wrap `corosensei` in a `RyoStack` type owned by the runtime
- Default stack size: `128KB` per task
- Guard page at the bottom of every stack
- On guard page hit: deliver `StackOverflow` error to the task, not a process
  crash
- Keep stack size configurable via a spawn option for the future

```
RyoStack {
    coroutine: corosensei::Coroutine,
    size: usize,          // default 128KB
    guard_page: bool,     // always true in v0.4
}
```

### 1.2 Task and Future Primitives

- Define `Task<T>` as an owned handle to a green thread
- Define `Future<T>` as the user-facing return value of `task.run`
- Implement `Drop` on `Future<T>` to request cancellation
- States: `Pending | Running | Completed(T) | Cancelled`

```
Future<T>:
    Drop  → sends CancelRequest to task
    .await → suspends caller until task completes, returns T
```

### 1.3 Minimal Cooperative Scheduler (Single OS Thread)

- A simple FIFO run queue of ready tasks
- `task.run` pushes a new task onto the queue
- `.await` on a future suspends the current task, yields to scheduler
- Scheduler loops: pop next ready task, resume it via `corosensei`
- No I/O yet — only `task.delay` (busy-wait stub for now)

### 1.4 TLS Runtime Handle

- Store the scheduler pointer in thread-local storage
- `task.run`, `.await`, channels all access it implicitly
- Establish the ambient runtime pattern that v0.4 will build on

### 1.5 Cancellation Delivery (Basic)

- When a future is dropped, set a `cancel_requested` flag on the task
- At every suspension point (`.await`, channel ops), check the flag
- If set, unwind the task's stack via normal Ryo drop semantics
- Deliver `task.Canceled` as an error into the task's error union

**Exit criteria:** A Ryo program can spawn 1,000 tasks, each doing simple
computation, switch between them cooperatively, and all complete correctly on
Linux, macOS, and Windows.

---

## Phase 2 — I/O Integration

**Goal:** Real non-blocking I/O via `mio`. Tasks suspend on I/O, not busy-wait.

### 2.1 Integrate `mio` Event Loop

- Create a `mio::Poll` instance owned by the runtime
- Scheduler loop becomes: run all ready tasks → poll I/O events → wake waiting
  tasks → repeat
- `mio` handles epoll (Linux), kqueue (macOS), IOCP (Windows) transparently

```
Scheduler loop:
    while true:
        drain ready_queue          // run all runnable tasks
        poll = mio.poll(timeout)   // block until I/O event or timeout
        for event in poll:
            wake_task(event.token) // move waiting task to ready_queue
```

### 2.2 Async File and Network I/O

- Wrap `mio`-backed TCP/UDP sockets as Ryo's standard `net` types
- `.send()` / `.recv()` on sockets suspend the task, register with `mio`,
  resume when ready
- File I/O on Linux via thread pool (files are not pollable on most OSes)
- Establish the `#[blocking]` FFI attribute — blocking calls go to thread pool

### 2.3 Real `task.delay`

- Replace the busy-wait stub with a timer wheel
- Design reference: Tokio's `HashedWheelTimer` (MIT licensed — study freely)
- Resolution: 1ms minimum, sufficient for scripting use cases
- `task.delay(duration)` suspends the task, wakes it after the timer fires

### 2.4 `task.timeout` Implementation

- Wraps any `future[!T]` with a timer
- On expiry: deliver `task.Timeout` to the waiting task
- Uses the same timer wheel as `task.delay`

**Exit criteria:** A Ryo HTTP server can handle concurrent connections. Tasks
correctly suspend on I/O without blocking OS threads. `task.delay(100ms)` is
accurate to within ~5ms.

---

## Phase 3 — Multi-Threading and Work Stealing

**Goal:** Spread tasks across multiple OS threads (M:N scheduling). Full
work-stealing.

### 3.1 Multi-Threaded Scheduler

- Spawn N OS worker threads (default: number of logical CPU cores)
- Each worker thread has its own local run queue and its own TLS runtime handle
- Each worker thread runs its own `mio` poll loop

### 3.2 Work Stealing via `crossbeam-deque`

- Each worker owns a `Worker<Task>` deque (push/pop local end)
- Each worker holds `Stealer<Task>` handles to all other workers
- When a worker's local queue is empty: steal from a random other worker
- Stealing is the standard Chase-Lev algorithm — `crossbeam-deque` implements
  this correctly

```
Worker loop:
    task = local_queue.pop()
         ?? steal_from_others()
         ?? park_until_woken()
    execute(task)
```

### 3.3 Task Affinity and Pinning

- By default tasks migrate freely between workers
- Consider `task.pin()` for tasks holding OS-thread-local resources (e.g. some
  C FFI libraries)
- Do not implement for v0.4 — design the hook, implement later

### 3.4 `#[blocking]` Thread Pool

- Separate fixed-size thread pool for blocking FFI calls
- When a `#[blocking]` function is called from a green thread:
  1. Move the blocking call to the thread pool
  2. Suspend the green thread
  3. Resume when the thread pool call completes
- Pool size: configurable, default 64 threads (matches Go's default)

**Exit criteria:** A CPU-bound workload across 10,000 tasks scales linearly
with core count. Work stealing keeps all cores busy. Blocking FFI calls do not
stall the scheduler.

---

## Phase 4 — Channels and High-Level Primitives

**Goal:** Implement all concurrency primitives described in the spec.

### 4.1 Channels

```
std.channel.create[T]() -> (sender[T], receiver[T])
```

- Bounded and unbounded variants
- `tx.send(value)` — moves value into channel, suspends if buffer full
- `rx.recv()` — suspends until message available, returns value
- Closing a channel delivers an error to all waiting receivers
- Both `sender[T]` and `receiver[T]` are owned types — drop = close that end

Internal implementation:
- `VecDeque<T>` as the ring buffer
- Intrusive wait lists for suspended senders and receivers
- No `Mutex` held during task resumption — only during queue manipulation

### 4.2 `task.scope` (Structured Concurrency)

- All tasks spawned inside a scope must complete before the scope exits
- If any task panics or the scope exits early: cancel all remaining tasks
- Scope holds a `JoinHandle` list; on exit, awaits all of them
- This is the recommended default over `task.spawn_detached`

### 4.3 `select`

- Waits on multiple concurrency primitives simultaneously
- First ready case wins; all others are cancelled
- `default` branch makes the select non-blocking
- Implementation: register all cases as wakers, first waker to fire wins,
  deregister the rest

```
select:
    case msg = rx.recv():     // channel receive
        handle(msg)
    case res = fut.await:     // future completion
        handle(res)
    case task.delay(1s).await: // timer
        print("timed out")
    default:                   // non-blocking
        print("nothing ready")
```

### 4.4 `task.gather` / `task.join` / `task.any`

- `task.join([futures])` — homogeneous, waits for all, returns `list[T]`
- `task.gather([futures])` — heterogeneous, waits for all, returns tuple
- `task.any([futures])` — returns first to complete, cancels the rest
- All three implemented on top of `select` internally

### 4.5 `fut.cancel()` and Cancellation Sources

Implement all cancellation sources from the spec:

| Source | Mechanism |
|---|---|
| `drop(future)` | Sets cancel flag, wakes task |
| `task.scope` exit | Cancels all child futures |
| `select` losing case | Cancels non-winning operations |
| `task.timeout` expiry | Delivers `Timeout` at next suspension |
| `fut.cancel()` | Explicit cancel call |

**Exit criteria:** All spec examples in §9.4 run correctly. `task.scope`
prevents resource leaks under cancellation. `select` with `default` is
non-blocking. Channels transfer ownership correctly under the borrow checker.

---

## Phase 5 — Hardening and Performance

**Goal:** Production-ready runtime. Correct under adversarial conditions,
tuned for Ryo's scripting workloads.

### 5.1 Stack Size Tuning

- Profile real Ryo programs to find p99 stack depth
- Consider adaptive starting size (start at 32KB, grow to 128KB on overflow)
- Implement stack caching — reuse freed stacks rather than returning to
  allocator immediately (reduces allocation pressure at high task churn)

### 5.2 Deadlock Detection

- In debug mode: detect when all tasks are suspended and no I/O is pending
- Report the cycle with task IDs and suspension points
- Matches the `mutex` deadlock detection described in §9.2.4

### 5.3 Scheduler Fairness

- Add a task age counter — tasks that have been in the queue longest get
  priority
- Prevents starvation in high-throughput workloads
- Consider a `task.yield_now()` hint for CPU-bound tasks (equivalent to
  Go's `runtime.Gosched()`)

### 5.4 Windows IOCP Hardening

- IOCP is completion-based, not readiness-based — different from epoll/kqueue
- `mio` abstracts this, but test explicitly:
  - Stack unwinding through IOCP callbacks
  - `#[blocking]` thread pool interaction with IOCP
  - SEH (Structured Exception Handling) compatibility via `corosensei`
- Do not ship v0.4 without a Windows CI run on every PR

### 5.5 Observability Hooks

- Task IDs visible in debug output and panic messages
- `task.current_id()` for user-facing introspection
- Runtime stats in debug mode: active tasks, queue depth, steal count
- Foundation for a future `ryo profile` tool

---

## Dependency Summary

| Crate | Purpose | Alternatives considered |
|---|---|---|
| `corosensei` | Stack switching, all platforms | `context`, raw `ucontext` |
| `mio` | I/O polling abstraction | `tokio` (rejected — wrong model) |
| `crossbeam-deque` | Work-stealing queues | Manual Chase-Lev |
| `crossbeam-channel` | Internal runtime messaging | `std::sync::mpsc` |

---

## What Is Explicitly Out of Scope

| Feature | Reason |
|---|---|
| Generator-style `yield` | Channels cover all use cases more idiomatically |
| WASM target | Stack swapping not available in standard WASM — tracked separately, see WasmFX note below |
| Stackless coroutines | Reintroduces function coloring, wrong for Ryo |
| Tokio as scheduler | Stackless-first, conflicts with stack-swapping model |
| `io_uring` direct | Linux only, use `mio` for cross-platform |
| Preemptive scheduling | Cooperative is sufficient for scripting; adds significant complexity |

---

## Future: WASM Target via WasmFX

> **Not in scope for v0.4.** This section documents the path forward for a
> future WASM backend once the platform matures sufficiently.

### Why WASM Is Deferred

Standard WASM has no accessible execution stack — it is a stack machine at the
bytecode level, but user code cannot swap or inspect stacks. Ryo's green thread
model depends entirely on stack swapping via `corosensei`, which has no
equivalent in standard WASM today. The only alternative — compiling tasks into
stackless state machines — reintroduces function coloring, which directly
contradicts Ryo's core design goal.

### WasmFX — The Right Future Primitive

[WasmFX](http://wasmfx.dev/) (formally: the WebAssembly Typed Continuations
proposal) is the upcoming WASM feature that would enable Ryo's concurrency
model on WASM without compromising its semantics.

**What it provides:**
- First-class, typed continuations — snapshots of an execution stack that can
  be suspended and resumed
- A general stack-switching instruction set sufficient to implement green
  threads, async/await, generators, and coroutines at the WASM level
- No whole-program transformation required — unlike CPS or state machine
  approaches

**Current status (as of 2025–2026):**
- Phase 3 of the W3C WebAssembly standardisation process
- Chrome and Firefox are actively implementing it
- Safari status: a tracking ticket exists but timeline is unclear
- The proposal is known as `stack-switching` / WasmFX in the
  [WebAssembly proposals repository](https://github.com/WebAssembly/proposals)

### What a Ryo WASM Backend Would Look Like

The key design principle: **Ryo's language semantics do not change**. Only the
runtime backend swaps.

```
v0.4 (native):          v0.x (WASM, future):
corosensei              WasmFX continuations
    +                       +
mio                     WASI 0.3 async I/O
    +                       +
crossbeam-deque         Single-threaded event loop (browser)
                        or WASI threads (server)
```

The runtime abstraction introduced in Phase 1 (`RyoStack`, TLS scheduler
handle) should be designed so that a WASM backend can be dropped in without
touching the scheduler interface. Concretely:

- `RyoStack` becomes a WasmFX continuation handle instead of a `corosensei`
  coroutine
- The scheduler loop drives WasmFX `resume` / `suspend` instead of
  `corosensei` yield
- `mio` is replaced by WASI 0.3 I/O (server) or the browser's event loop
  (browser target)
- Work stealing is replaced by a single-threaded cooperative loop in the
  browser context (no `SharedArrayBuffer` requirement)

### WASM Target Variants

| Target | I/O backend | Threading | Notes |
|---|---|---|---|
| Browser | Browser event loop | Single-threaded | No `SharedArrayBuffer` needed |
| WASI server | WASI 0.3 async | WASI threads (if available) | Wasmtime has experimental support |
| Edge functions | WASI 0.3 async | Single-threaded | Cloudflare Workers, Fastly Compute |

### Prerequisites Before Starting

Do not begin WASM work until all of the following are true:

1. WasmFX reaches Phase 4 (standardised) or has stable support in at least
   two major runtimes (V8 + SpiderMonkey, or Wasmtime + V8)
2. Ryo's native runtime (Phases 1–5) is stable and well-tested
3. The `RyoStack` abstraction has been validated as genuinely swappable by
   writing a mock backend for testing purposes first
4. WASI 0.3 is stable (expected late 2026 / early 2027 for 1.0)

### Risk

The single biggest risk is Safari. WasmFX browser support requires all three
major engines. If Safari lags significantly, the browser WASM target may need
to fall back to a stackless state machine compilation mode for Safari only —
effectively a per-engine codegen path. This is significant engineering work and
should only be tackled if the browser target is a stated product priority.

---

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Windows IOCP + stack unwinding edge cases | Medium | High | CI on Windows from Phase 1, not Phase 5 |
| Stack overflow in recursive user code | Medium | Medium | Guard pages + `StackOverflow` error delivered to task |
| Work stealing causing cache thrashing | Low | Medium | Profile before tuning; start with random steal |
| `corosensei` platform gap | Low | High | Check supported targets before committing; it covers x86_64 + aarch64 on all three OSes |
| Cancellation during `select` leaving zombie wakers | Medium | High | Strict ownership of waker registration; cancel deregisters atomically |

---

## Milestone Summary

| Phase | Deliverable | Unlocks |
|---|---|---|
| 1 | Single-threaded green threads, cooperative switch | Basic `task.run` + `.await` |
| 2 | `mio` I/O, timer wheel | Non-blocking I/O, `task.delay`, `task.timeout` |
| 3 | Work stealing, `#[blocking]` pool | True M:N parallelism, safe FFI |
| 4 | Channels, `select`, `task.scope` | Full spec compliance |
| 5 | Hardening, observability, Windows CI | Production readiness |
