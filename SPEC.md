# Obsidian Language — Full Design Specification
### Version 0.1 — Complete Reference

---

> "Say what you mean. Mean what you say. Pay only for what you use."

---

# TABLE OF CONTENTS

1. Philosophy & Guarantees
2. File & Package System
3. Primitive Types
4. Variables — `const` / `var`
5. Functions — `fn`
6. Classes — `class`
7. Behavior — `impl`
8. Interfaces — `interface`
9. Enums — `enum`
10. Generics
11. Control Flow — `if` `else` `for` `while` `loop` `match` `break` `continue`
12. Pattern Matching — `match`
13. Error Handling — `!` `fail` `pass` `else err` `guard` `!!`
14. Memory Management — `const` `var` `alloc` `defer` `~` `&` `*`
15. Closures & Lambdas
16. The Spark Feed — `-:`
17. String System — `str` `<>` `${}` `"""`
18. Async & Concurrency — `async` `await` `spawn` `scope`
19. Comptime — `@comptime` `comptime if`
20. Decorators — `@`
21. Generics — Constraints & Bounds
22. Access Modifiers — `pub` `priv`
23. Mutability — `mut`
24. The `self` Keyword
25. Namespaces & the `::` Operator
26. Imports & Exports
27. Optional Types — `T?` `some` `none`
28. Collections — `[T]` `[T;N]` `Map<K,V>` `Set<T>`
29. The `_` Discard Identifier
30. Operator Reference
31. Compiler Modes & Flags
32. Standard Library Surface
33. Complete Keyword Index

---

## 1. Philosophy & Guarantees

Obsidian makes four unconditional guarantees:

**1. No hidden costs.**
Every allocation, every heap usage, every mutable operation is visible at the call site.
There is no background GC, no hidden vtable dispatch, no implicit copy.

**2. No hidden control flow.**
Errors travel as values or visible `else err` branches.
`fail` is the only way to signal failure. `pass` is the only way to propagate.
There are no exceptions that silently unwind the stack.

**3. Immutability by default.**
`const` is the default. Mutation requires `var` or `mut self`.
Every mutable thing in a codebase is findable with a grep for `var` or `mut`.

**4. Readability is architecture.**
A file's structure — package, imports, data, behavior, interfaces — is always
in the same order. Reading top to bottom tells you everything.

---

## 2. File & Package System

Every `.obs` file declares exactly one package at the top.
The package name must mirror the folder path relative to `src/`.

```obsidian
package game::player          // file lives at src/game/player.obs
package std::io               // file lives at src/std/io.obs
package main                  // entry point — file is src/main.obs
```

**Rules:**
- One `package` declaration per file. Required. Must be line 1.
- Package names use `::` as a separator, matching folder separators.
- Circular imports are a compile-time error.
- Nothing is exported unless marked `pub`.
- No wildcard imports. Every imported name is explicit.

**Import syntax:**
```obsidian
import std::io          { print, println, read }
import std::math        { sqrt, clamp, abs }
import game::player     { Player, Entity }
import game::errors     { GameError, ERR_NOT_FOUND }
import game::world              // imports the module — access via world::Thing
```

**File structure order (enforced by formatter):**
```
1. package declaration
2. imports
3. constants
4. enums
5. interfaces
6. classes (data only)
7. impl blocks (behavior)
8. free functions
9. entry fn main() if applicable
```

---

## 3. Primitive Types

### Integer Types
| Type | Size | Range |
|------|------|-------|
| `i8`  | 8-bit  | -128 to 127 |
| `i16` | 16-bit | -32,768 to 32,767 |
| `i32` | 32-bit | -2,147,483,648 to 2,147,483,647 |
| `i64` | 64-bit | -9.2×10¹⁸ to 9.2×10¹⁸ |
| `i128`| 128-bit| very large |
| `u8`  | 8-bit unsigned | 0 to 255 |
| `u16` | 16-bit unsigned | 0 to 65,535 |
| `u32` | 32-bit unsigned | 0 to 4,294,967,295 |
| `u64` | 64-bit unsigned | 0 to 1.8×10¹⁹ |
| `int` | platform native | alias for i64 on 64-bit |
| `uint`| platform native | alias for u64 on 64-bit |
| `byte`| alias for u8 | 0 to 255 |

### Float Types
| Type  | Size | Precision |
|-------|------|-----------|
| `f32` | 32-bit | ~7 decimal digits |
| `f64` | 64-bit | ~15 decimal digits |
| `float` | platform native | alias for f64 |

### Other Primitives
| Type   | Description |
|--------|-------------|
| `bool` | `true` or `false` |
| `str`  | UTF-8 string — immutable by default |
| `void` | absence of a value — used in `-> void` returns |
| `never`| a type that never produces a value (infinite loops, always-fail fns) |
| `any`  | escape hatch — dynamically typed, use sparingly |

### Literals
```obsidian
42          // int
42_000_000  // int with separators — readable
3.14        // f64
3.14f32     // f32 literal
true false  // bool
"hello"     // str
'A'         // byte / char (single quotes = single byte)
none        // absence — used with T? optionals
```

---

## 4. Variables — `const` and `var`

### `const` — Immutable Binding

```obsidian
const x = 5                   // type inferred as int
const y: f32 = 3.14           // explicit type
const name = "Arin"           // str, inferred
const player = Player::new()  // immutable reference to a Player
```

- Cannot be reassigned after declaration.
- Cannot call `mut self` methods on a `const` binding.
- Compiler error if you attempt mutation.
- Preferred for everything. Use `var` only when you need to change it.

### `var` — Mutable Binding

```obsidian
var score = 0                  // mutable int
var name: str = "Arin"        // explicit type, mutable
var pos = Vec2::zero()         // mutable Vec2
```

- Can be reassigned with `=`.
- Can call `mut self` methods.
- The formatter **visually aligns** `var` declarations to make mutation scannable.
- In `strict` mode: a `var` that is never mutated after declaration → compiler warning.

### Type Inference Rules
- The compiler infers type from the right-hand expression.
- Explicit annotation is always allowed and preferred for public API boundaries.
- Integer literals default to `int`. Float literals default to `f64`.
- Use suffix to override: `42u8`, `3.14f32`.

### Shadowing
```obsidian
const x = 5
const x = x + 1        // shadows previous x — allowed, creates new binding
```

Shadowing is allowed but triggers a compiler hint in strict mode.

---

## 5. Functions — `fn`

### Basic Declaration

```obsidian
fn greet(name: str) -> str {
    "Hello, ${name}!"           // last expression = return value
}

fn add(a: int, b: int) -> int {
    a + b
}

fn log_message(msg: str) {      // void return — no -> needed
    println(msg)
}
```

### Explicit Return

```obsidian
fn clamp_hp(hp: int) -> int {
    if (hp > 100) { return 100 }
    if (hp < 0)   { return 0   }
    hp
}
```

`return` exits early. The last expression is the implicit return.
Both can be used in the same function.

### Named Arguments

All functions support named arguments at the call site:

```obsidian
fn connect(host: str, port: int, timeout: int) -> Connection! { ... }

// positional — allowed but discouraged for 3+ args
connect("localhost", 8080, 5000)

// named — idiomatic Obsidian for 3+ arguments
connect(host: "localhost", port: 8080, timeout: 5000)
```

Named arguments can be passed in any order:

```obsidian
connect(timeout: 5000, port: 8080, host: "localhost")   // valid
```

### Default Parameters

```obsidian
fn connect(host: str, port: int = 8080, timeout: int = 5000) -> Connection! {
    ...
}

connect(host: "localhost")                    // port=8080, timeout=5000
connect(host: "localhost", port: 9090)        // timeout=5000
```

### Fallible Functions — `-> T!`

The `!` suffix on a return type declares that the function can fail.
A function without `!` is guaranteed to never fail:

```obsidian
fn safe(x: int) -> int { x * 2 }          // cannot fail
fn risky(path: str) -> str! { ... }        // can fail — caller must handle
```

See Section 13 for full error handling.

### Free Functions vs Methods

Free functions live at the package level. Methods live inside `impl` blocks.
Free functions cannot access class state — they are pure operations on their arguments.

```obsidian
// free function — package-level
fn clamp(val: int, lo: int, hi: int) -> int {
    if (val < lo) { return lo }
    if (val > hi) { return hi }
    val
}
```

### Entry Point

Every executable Obsidian program has exactly one `main` function:

```obsidian
fn main() { ... }                           // synchronous entry
async fn main() { ... }                     // async entry
fn main() -> void! { ... }                 // fallible entry — error printed to stderr
```

---

## 6. Classes — `class`

A `class` declares a named data structure. It holds fields and nothing else.
Behavior lives in `impl` blocks — always separate.

```obsidian
class Player {
    pub       name:   str
    pub   var hp:     int = 100
    pub   var score:  int = 0
    priv  var secret: str = "hunter2"
}
```

### Field Rules

| Syntax | Meaning |
|--------|---------|
| `name: str` | immutable field, package-visible |
| `var name: str` | mutable field, package-visible |
| `pub name: str` | immutable field, public |
| `pub var name: str` | mutable field, public |
| `priv name: str` | immutable field, private to this file |
| `priv var name: str` | mutable field, private to this file |

### Default Values

```obsidian
class Config {
    host:    str  = "localhost"
    port:    int  = 8080
    debug:   bool = false
    timeout: int  = 5000
}
```

Fields with default values are optional in constructors.

### Instantiation

Classes are instantiated via `::new()` (by convention) or named constructors:

```obsidian
const p = Player { name: "Arin", hp: 100, score: 0 }   // struct-literal
const p = Player::new(name: "Arin")                      // constructor (defined in impl)
```

Struct-literal syntax requires all fields without defaults to be provided.
Fields with defaults can be omitted.

### Stack vs Heap

By default, all class instances are **stack-allocated**:

```obsidian
const p = Player::new("Arin")      // stack — freed when scope ends
```

Heap allocation requires explicit allocator:

```obsidian
var p = alloc(arena) Player::new("Arin")
defer free(arena, p)
```

---

## 7. Behavior — `impl`

`impl` blocks attach methods to a class. They are always separate from the class declaration.

```obsidian
impl Player {

    // static constructor — no self
    pub fn new(name: str) -> Self {
        Self { name: name, hp: 100, score: 0, secret: "hunter2" }
    }

    // pure method — reads state, guarantees no mutation
    pub fn is_alive(self) -> bool {
        self.hp > 0
    }

    // mutating method — must declare mut self
    pub fn take_damage(mut self, amount: int) -> void! {
        if (amount < 0) { fail GameError::new("negative damage") }
        self.hp = (self.hp - amount).clamp(0, 100)
    }

    // static utility — no self at all
    pub fn max_hp() -> int { 100 }

}
```

### Method Types

| Signature | Type | Can mutate? |
|-----------|------|-------------|
| `fn foo(self)` | instance, pure | No — compiler enforced |
| `fn foo(mut self)` | instance, mutating | Yes |
| `fn foo()` | static | No self at all |
| `fn foo() -> Self` | static constructor | Returns new instance |

### `Self` Keyword

Inside `impl` blocks, `Self` refers to the class being implemented:

```obsidian
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }         // Self = Vec2
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
```

### Multiple `impl` Blocks

A class can have multiple `impl` blocks — useful for grouping:

```obsidian
// core behavior
impl Player { ... }

// interface implementation
impl Entity for Player { ... }

// serialization
impl Serializable for Player { ... }
```

---

## 8. Interfaces — `interface`

An interface is a named collection of method signatures.
It defines a contract — nothing more.

```obsidian
pub interface Entity {
    fn update(mut self, dt: f32)
    fn is_active(self) -> bool
    fn describe(self) -> str
}

pub interface Serializable {
    fn to_json(self) -> str
    fn from_json(data: str) -> Self!
}
```

### Rules

- Interfaces contain **only signatures** — no fields, no default implementations.
- A class satisfies an interface **structurally** — no `implements` keyword.
- If the methods exist with matching signatures, the interface is satisfied.
- The compiler checks interface satisfaction at the `impl InterfaceName for Class` site.

### Implementing an Interface

```obsidian
impl Entity for Player {
    fn update(mut self, dt: f32) {
        self.pos.x = self.pos.x + self.vel.x * dt
        self.pos.y = self.pos.y + self.vel.y * dt
    }
    fn is_active(self) -> bool { self.is_alive() }
    fn describe(self)  -> str  { self.status_bar() }
}
```

### Using Interfaces as Types

```obsidian
fn print_all(entities: [Entity]) {
    for (e in entities) {
        println(e.describe())
    }
}

fn update_world(entity: mut Entity, dt: f32) {
    entity.update(dt)
}
```

### Interface Composition

```obsidian
interface Printable {
    fn to_str(self) -> str
}

interface Loggable {
    fn log_line(self) -> str
}

// compose interfaces
interface Debuggable : Printable, Loggable {
    fn dump(self)
}
```

---

## 9. Enums — `enum`

Enums declare a closed set of named variants. Variants can carry data.

```obsidian
// simple enum
enum Direction { North, South, East, West }

// enum with payload
enum Shape {
    Circle   { radius: f32 },
    Rect     { width: f32, height: f32 },
    Triangle { base: f32, height: f32 },
    Point,
}

// enum as result carrier
enum NetStatus {
    Connected { address: str, port: int },
    Timeout   { after_ms: int },
    Refused,
}
```

### Instantiation

```obsidian
const dir   = Direction.North
const shape = Shape.Circle { radius: 5.0 }
const net   = NetStatus.Connected { address: "127.0.0.1", port: 8080 }
```

### Matching Enums

```obsidian
const area = match (shape) {
    Shape.Circle   { radius }        => 3.14159 * radius * radius,
    Shape.Rect     { width, height } => width * height,
    Shape.Triangle { base, height }  => 0.5 * base * height,
    Shape.Point                      => 0.0,
}
```

### Impl on Enums

```obsidian
impl Shape {
    pub fn area(self) -> f32 {
        match (self) {
            Shape.Circle   { radius }        => 3.14159 * radius * radius,
            Shape.Rect     { width, height } => width * height,
            Shape.Triangle { base, height }  => 0.5 * base * height,
            Shape.Point                      => 0.0,
        }
    }
}
```

---

## 10. Generics

### Generic Functions

```obsidian
fn first<T>(list: [T]) -> T? {
    if (list.len() == 0) { return none }
    list[0]
}

fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}
```

### Generic Classes

```obsidian
class Box<T> {
    pub var value: T
}

impl<T> Box<T> {
    pub fn new(val: T) -> Self   { Self { value: val } }
    pub fn get(self) -> T        { self.value }
    pub fn set(mut self, val: T) { self.value = val }
}
```

### Constrained Generics

Use `:` to require an interface:

```obsidian
fn print_all<T: Printable>(items: [T]) {
    for (item in items) { println(item.to_str()) }
}

fn largest<T: Comparable>(a: T, b: T) -> T {
    if (a > b) { a } else { b }
}

// multiple constraints with +
fn process<T: Printable + Serializable>(item: T) {
    println(item.to_str())
    save(item.to_json())
}
```

### Generic Enums

```obsidian
enum Option<T> {
    Some { value: T },
    None,
}

enum Pair<A, B> {
    Both { first: A, second: B }
}
```

---

## 11. Control Flow

### `if` / `else if` / `else`

Conditions always in parentheses:

```obsidian
if (score > 100) {
    println("High score!")
} else if (score > 50) {
    println("Good score")
} else {
    println("Keep trying")
}
```

`if` is also an expression:

```obsidian
const label = if (hp > 50) { "healthy" } else { "injured" }
```

### `for` — Iterator Loop

```obsidian
for (item in collection) {
    println(item.name)
}

// with index
for (i, item in collection.enumerate()) {
    println("${i}: ${item.name}")
}
```

### Range Loops

```obsidian
for (i in 0..10)   { println(i) }      // 0, 1, ..., 9  (exclusive)
for (i in 0..=10)  { println(i) }      // 0, 1, ..., 10 (inclusive)
for (i in 10..0)   { println(i) }      // 10, 9, ..., 1 (reverse)
for (i in 0..100 step 5) { println(i) } // 0, 5, 10, ..., 95
```

### `while` — Condition Loop

```obsidian
while (player.is_alive()) {
    game_tick(delta)
}

while (queue.len() > 0) {
    const task = queue.pop()!!
    task.run()
}
```

### `loop` — Infinite Loop

```obsidian
loop {
    const input = read_input()
    if (input == "quit") { break }
    process(input)
}
```

### `break` and `continue`

```obsidian
for (i in 0..100) {
    if (i == 50) { break }          // exit loop entirely
    if (i % 2 == 0) { continue }   // skip to next iteration
    println(i)
}
```

Named breaks for nested loops:

```obsidian
outer: for (i in 0..10) {
    for (j in 0..10) {
        if (i + j > 15) { break outer }
    }
}
```

---

## 12. Pattern Matching — `match`

`match` is Obsidian's primary branching tool. It replaces `switch`.
The compiler enforces **exhaustion** — every possible case must be handled.

### Basic Match

```obsidian
match (direction) {
    Direction.North => println("Going north"),
    Direction.South => println("Going south"),
    Direction.East  => println("Going east"),
    Direction.West  => println("Going west"),
}
```

### Match as Expression

```obsidian
const label = match (hp) {
    n if (n > 75) => "healthy",
    n if (n > 40) => "wounded",
    n if (n > 0)  => "critical",
    _             => "dead",
}
```

### Destructuring in Match

```obsidian
match (shape) {
    Shape.Circle   { radius }        => println("Circle r=${radius}"),
    Shape.Rect     { width, height } => println("Rect ${width}x${height}"),
    Shape.Triangle { base, height }  => println("Triangle"),
    Shape.Point                      => println("Point"),
}
```

### Guards

```obsidian
match (code) {
    n if (n >= 200 && n < 300) => println("Success"),
    n if (n >= 400 && n < 500) => println("Client error"),
    n if (n >= 500)            => println("Server error"),
    _                          => println("Unknown"),
}
```

### Multi-Arm (same result for multiple patterns)

```obsidian
match (direction) {
    Direction.North | Direction.South => println("Vertical"),
    Direction.East  | Direction.West  => println("Horizontal"),
}
```

### Wildcard `_`

`_` matches anything. Must come last:

```obsidian
match (status) {
    NetStatus.Connected { address, port } => println("Connected to ${address}:${port}"),
    NetStatus.Timeout   { after_ms }      => println("Timed out after ${after_ms}ms"),
    _                                     => println("Not connected"),
}
```

---

## 13. Error Handling

Error handling in Obsidian is built on one principle:
**errors are flow — not a parallel system.**

### `T!` — Fallible Return Type

Any function that can fail appends `!` to its return type:

```obsidian
fn load(path: str) -> str!  { ... }    // returns str or a failure
fn save(data: str) -> void! { ... }    // returns nothing or a failure
fn count() -> int           { ... }    // cannot fail — guaranteed
```

The `!` is part of the type. `str!` and `str` are different types.
A function without `!` cannot contain `fail`. Compiler enforced.

### `fail` — Raise a Failure

`fail` returns a failure from the current function. It is the only way to signal failure.

```obsidian
fn divide(a: f32, b: f32) -> f32! {
    if (b == 0.0) {
        fail MathError::new("Division by zero")
    }
    a / b
}
```

- `fail` takes any value — typically a class instance
- `fail` exits the function immediately (like `return`)
- A function with `!` in its signature must have at least one `fail` path

### `else err { }` — Inline Handling

The primary error handling pattern. An `else err` branch runs only on failure:

```obsidian
const data = load_file("config.obs") else err {
    println("Failed: ${err.message}")
    return                              // bail out
}
// data is clean here
```

`err` inside the block is automatically bound to the failure value.
Its type is inferred from the function's failure type.

Provide a fallback value to continue past the error:

```obsidian
const player = find_player("Arin") else err {
    log.warn("Not found: ${err.message}")
    Player::guest()                     // fallback — execution continues
}
// player is always valid here
```

### `else { }` — Handle Without Inspecting

When you don't need the error details:

```obsidian
const cfg = load_config(path) else {
    Config::default()                   // silently fall back
}
```

### `pass` — Propagate to Caller

Propagates a failure up to the calling function.
The calling function must also be `-> T!`:

```obsidian
fn load_and_parse(path: str) -> Config! {
    const raw    = pass load_file(path)     // if failed — propagate up
    const config = pass parse_config(raw)   // same
    config
}
```

`pass` is explicit. You can grep a file for every `pass` to see every propagation point.

### `guard` — All-or-Nothing Block

When several calls must all succeed before you continue:

```obsidian
fn setup() -> void! {
    guard {
        pass load_assets()
        pass init_world()
        pass connect_db()
    } else err {
        println("Setup failed at: ${err.message}")
        fail err                          // re-propagate
    }

    println("All systems ready")
}
```

If any call inside `guard` fails, execution jumps to `else err`.
The `err` binding is the first failure encountered.

### `!!` — Assert Success

For situations you are certain cannot fail:

```obsidian
const player = find_player("Arin")!!    // panic if not found
const head   = list.first()!!           // panic if list is empty
```

A panic prints the location, the failure value, and exits.
Use `!!` only for things that would be bugs, not cases.

### `try / catch` — For True Panics Only

`try/catch` exists only for unexpected runtime crashes.
It is **not** for normal error handling:

```obsidian
try {
    const result = ffi::dangerous_call()
    process(result)
} catch (p: Panic) {
    log.critical("Crash: ${p.message}")
    graceful_shutdown()
}
```

**Rule:** If you are writing `try/catch` for logic you control — use `else err` instead.

### Error Types

Errors are plain classes. There is no base `Error` class to extend.
Use interfaces if you want grouping:

```obsidian
pub class IoError {
    pub message: str
    pub path:    str?
    pub code:    int
}

pub class NetworkError {
    pub message:   str
    pub status:    int
    pub endpoint:  str
}

// optional grouping via interface
pub interface AppError {
    fn message(self) -> str
    fn code(self) -> int
}

impl AppError for IoError     { ... }
impl AppError for NetworkError { ... }
```

---

## 14. Memory Management

### Stack (Default)

All class instances and primitives are stack-allocated by default.
No annotation required. Freed automatically when scope ends:

```obsidian
fn process() {
    const player = Player::new("Arin")  // stack
    const vec    = Vec2::new(1.0, 2.0)  // stack
}   // both freed here — zero cost
```

### Heap — `alloc` / `free`

Heap allocation requires an explicit allocator argument.
The allocator is a class like any other — passed around explicitly:

```obsidian
fn load_level(arena: Arena, path: str) -> Level! {
    var buf = alloc(arena, 4096)        // allocate 4096 bytes on heap
    defer free(arena, buf)              // always freed when scope ends

    const data = pass fs::read_into(buf, path)
    pass Level::parse(data)
}
```

`alloc(allocator, size_or_type)` — explicit, visible, greppable.
`free(allocator, ptr)` — explicit cleanup.

### `defer` — Deterministic Cleanup

`defer` schedules a statement to run when the current scope exits —
whether by normal return, `fail`, or `pass`:

```obsidian
fn open_and_read(path: str) -> str! {
    var file = pass fs::open(path)
    defer file.close()                  // always runs, even on failure

    pass file.read_all()
}
```

Multiple `defer` statements run in **reverse order** (LIFO):

```obsidian
defer cleanup_a()    // runs third
defer cleanup_b()    // runs second
defer cleanup_c()    // runs first
```

### Ownership Hints

Ownership hints are optional annotations that communicate intent.
They generate compiler warnings on misuse but are not a hard borrow-checker:

| Hint | Syntax | Meaning |
|------|--------|---------|
| Own  | `~T`   | Takes ownership — caller cannot use value after this call |
| Borrow | `&T` | Borrow — read-only, does not take ownership |
| Mut-borrow | `*T` | Mutable borrow — caller's value is changed |

```obsidian
fn consume(~data: [byte]) { ... }       // takes ownership
fn inspect(&data: [byte]) { ... }       // borrows read-only
fn mutate(*data: [byte])  { ... }       // mutates caller's data
```

---

## 15. Closures & Lambdas

### Lambda Syntax

```obsidian
|args| => expression

|x: int| => x * 2                      // single arg, explicit type
|x, y| => x + y                        // multiple args, inferred types
|| => println("no args")               // no arguments
|x: int| => {                          // block body
    const doubled = x * 2
    doubled + 1
}
```

### First-Class Functions

Functions and lambdas are values. They can be stored and passed:

```obsidian
const double: fn(int) -> int = |x| => x * 2

fn apply(f: fn(int) -> int, val: int) -> int {
    f(val)
}

apply(double, 5)                        // 10
apply(|x| => x * x, 4)                 // 16
```

### Capturing

Lambdas capture their enclosing scope:

```obsidian
const multiplier = 3
const triple = |x: int| => x * multiplier      // captures multiplier

triple(5)           // 15
triple(10)          // 30
```

Captured `var` bindings are captured by reference. Captured `const` by value.

---

## 16. The Spark Feed — `-:`

`-:` is Obsidian's unique pipeline operator.
It feeds the left-hand value into the first argument of the right-hand function.

```obsidian
data -: transform()
// equivalent to: transform(data)

data -: map(f) -: filter(p) -: reduce(0, acc)
// equivalent to: reduce(filter(map(data, f), p), 0, acc)
```

### Rules

- `-:` works with any function whose first parameter matches the left-hand type.
- Named arguments after the first are written normally: `data -: join(sep: ", ")`
- `-:` chains are formatted one-per-line by the formatter for readability.
- `-:` compiles to a direct function call — zero overhead.

### Common Patterns

```obsidian
const result = users
    -: filter(|u| => u.active)
    -: map(|u| => u.name)
    -: sort()
    -: join(sep: ", ")

const total = orders
    -: filter(|o| => o.status == "complete")
    -: map(|o| => o.amount)
    -: reduce(0.0, |acc, x| => acc + x)
```

---

## 17. String System

### String Type — `str`

`str` is an immutable UTF-8 string. It is not null-terminated.
It carries a length. It is safe to index by byte or codepoint.

### Concatenation — `<>`

```obsidian
const greeting = "Hello, " <> name <> "!"
const path = dir <> "/" <> filename <> ".obs"
```

`<>` is Obsidian's string join operator. It does not work on non-strings.
Use explicit conversion: `int.to_str()` before `<>`.

### Interpolation — `${}`

```obsidian
const msg  = "Score: ${score} — Rank: ${rank}"
const info = "Player ${player.name} has ${player.hp}HP"
```

`${}` accepts any expression. The expression is converted via `.to_str()` automatically.
Interpolation is syntax sugar for `<>` chains and compiles identically.

### Multi-line Strings — `"""`

```obsidian
const help = """
    Usage: obsidian [command]

    Commands:
      build    Compile the project
      run      Build and execute
      test     Run @test functions
      fmt      Format all source files
"""
```

Leading whitespace is stripped to the indent level of the closing `"""`.

### String Methods (stdlib)

```obsidian
str.len()                       // byte length
str.chars()                     // iterator over unicode codepoints
str.trim()                      // strip leading/trailing whitespace
str.to_upper() / .to_lower()   // case conversion
str.starts_with(prefix: str)
str.ends_with(suffix: str)
str.contains(needle: str)
str.split(sep: str) -> [str]
str.replace(from: str, to: str) -> str
str.parse<T>() -> T!            // parse to any type
```

---

## 18. Async & Concurrency

### `async fn`

Mark a function as asynchronous:

```obsidian
async fn fetch(url: str) -> str! {
    const res = await http::get(url)
    pass res.body()
}
```

An `async fn` returns a `Future<T>` that resolves when awaited.

### `await`

`await` suspends the current async function until a future resolves:

```obsidian
async fn main() {
    const page = await fetch("https://example.com") else err {
        println("Fetch failed: ${err.message}")
        return
    }
    println(page)
}
```

`await` is only valid inside `async fn`.

### `spawn` — Concurrent Tasks

`spawn` launches an async function concurrently:

```obsidian
const handle = spawn fetch("https://api.example.com")
// ... do other work ...
const result = await handle else { "timeout fallback" }
```

### `scope` — Structured Concurrency

All tasks spawned inside a `scope` block **must complete** before the scope exits.
This is a compiler-enforced guarantee. There are no leaked tasks:

```obsidian
scope {
    spawn process_chunk(a)
    spawn process_chunk(b)
    spawn process_chunk(c)
}
// ← all three are guaranteed complete here
// ← this line cannot be reached while any task is alive
```

### Channels (stdlib)

```obsidian
const ch = Channel<str>::new(capacity: 32)

spawn {
    ch.send("hello")
    ch.send("world")
    ch.close()
}

for (msg in ch) {
    println(msg)
}
```

---

## 19. Comptime — `@comptime`

Comptime is Obsidian evaluated at compile time.
The same syntax, the same language, running earlier.

```obsidian
@comptime
fn type_name<T>() -> str {
    T::name
}

@comptime
const MAX_PLAYERS: int = 64

@comptime
const PLATFORM: str = env("TARGET_PLATFORM")
```

### `comptime if`

Branches evaluated and eliminated at compile time:

```obsidian
comptime if (PLATFORM == "wasm") {
    import platform::wasm { log, alloc }
} else if (PLATFORM == "native") {
    import platform::native { log, alloc }
} else {
    import platform::stub { log, alloc }
}
```

Dead branches are fully eliminated — zero runtime cost.

### Comptime Functions as Code Generators

```obsidian
@comptime
fn make_vec_type(name: str, dims: int) -> type {
    // generates a Vec2, Vec3, Vec4 type at compile time
}

const Vec3 = make_vec_type("Vec3", 3)
```

---

## 20. Decorators — `@`

Decorators annotate declarations. They run at compile time.

### Built-in Decorators

| Decorator | Target | Effect |
|-----------|--------|--------|
| `@inline` | `fn` | Force inline at every call site |
| `@cold` | `fn` | Hint: rarely executed — move out of hot path |
| `@test` | `fn` | Mark as test — runs with `obsidian test` |
| `@deprecated(msg)` | any | Compiler warning at every usage |
| `@derive(...)` | `class` `enum` | Auto-generate interface implementations |
| `@packed` | `class` | No padding between fields |
| `@align(n)` | `class` `field` | Force alignment to n bytes |
| `@extern("sym")` | `fn` | Link to external C symbol |
| `@comptime` | `fn` `const` | Evaluated at compile time |
| `@allow(warning)` | any | Suppress a specific compiler warning |
| `@doc("...")` | any | Attach documentation string |

### `@derive`

Automatically generate `impl` blocks for common interfaces:

```obsidian
@derive(Printable, Comparable, Hashable, Serializable)
class Score {
    pub value: int
    pub name:  str
}
```

Generated code is visible in the build output. Not magic — just automation.

### Custom Decorators

A decorator is a `@comptime` function that takes a declaration and returns a modified one:

```obsidian
@comptime
fn logged(func: fn) -> fn {
    // wraps func with entry/exit logging
}

@logged
fn process(data: str) { ... }
```

---

## 21. Access Modifiers — `pub` and `priv`

| Modifier | Visibility |
|----------|-----------|
| (none) | Package-visible — accessible within the same package |
| `pub` | Public — accessible from any package that imports this one |
| `priv` | Private — accessible only within the declaring file |

```obsidian
class Config {
    pub  host: str          // any package can read
    pub  port: int          // any package can read
         debug: bool        // only game::config package
    priv secret: str        // only this file
}
```

`pub` on an `impl` block makes all methods public:

```obsidian
pub impl Player {           // all methods are pub
    fn new() -> Self { ... }
    fn update() { ... }
}
```

Or selectively:

```obsidian
impl Player {
    pub fn new() -> Self { ... }    // public
    pub fn is_alive() -> bool { ... }
        fn internal() { ... }       // package-visible
    priv fn secret_calc() { ... }   // file-private
}
```

---

## 22. Mutability — `mut`

`mut` appears in three places:

### 1. `mut self` — Mutating Methods

```obsidian
impl Player {
    fn read(self) -> int { self.hp }              // cannot change state
    fn change(mut self) { self.hp = self.hp - 1 } // can change state
}
```

`self` without `mut` is a read-only view. The compiler rejects mutation attempts.

### 2. `var` fields in classes

```obsidian
class Player {
    pub var hp: int = 100       // mutable from outside (pub) — needs mut self to set
    priv var score: int = 0    // only settable inside impl
}
```

### 3. Interface mut requirements

```obsidian
interface Entity {
    fn update(mut self, dt: f32)    // implementors must accept mut self
    fn describe(self) -> str        // read-only
}
```

---

## 23. The `self` Keyword

`self` refers to the current instance inside an `impl` block.

```obsidian
impl Player {
    fn greet(self) -> str {
        "I am " <> self.name        // self.field access
    }

    fn heal(mut self, amount: int) {
        self.hp = self.hp + amount  // mut self allows field mutation
        self.update_level()         // can call other methods via self
    }
}
```

`self` is never implicit. Field access always requires `self.field`.
There is no bare `hp` — always `self.hp`. This is intentional.
It makes every state access visible and greppable.

---

## 24. Namespaces & `::`

`::` accesses type-level and module-level members.

```obsidian
Player::new("Arin")         // static method on Player
Vec2::zero()                // static constructor
Math::PI                    // constant on Math module
Direction::North            // enum variant (alternative to Direction.North)
std::io::println            // fully qualified function path
List<int>::with_capacity(32)
```

### `::` vs `.`

| Operator | Used for |
|----------|---------|
| `::` | Type-level (static methods, module paths, enum variants, constants) |
| `.` | Instance-level (fields, instance methods, chained calls) |

This distinction is always clear. `::` = you're talking to a **type or module**.
`.` = you're talking to an **object**.

---

## 25. Optional Types — `T?`

`T?` is a type that can hold either a value of type `T` or `none`.
Null does not exist in Obsidian without `T?`. Period.

```obsidian
const name: str?  = none            // no name yet
const name: str?  = "Arin"         // has a name
const name: str   = "Arin"         // can never be none — compiler enforced
```

### Producing Optional Values

```obsidian
fn find(id: int) -> Player? {
    // ... search ...
    if (found) { return player }
    return none
}
```

### Consuming Optional Values

```obsidian
// match — exhaustive
match (find(42)) {
    some(player) => println(player.name),
    none         => println("Not found"),
}

// safe unwrap with fallback
const player = find(42) or Player::guest()

// assert non-none (panic if wrong)
const player = find(42)!!

// chain — only runs if value is present
find(42)?.heal(amount: 10)      // ? propagates none through the chain
```

### Optional Chaining — `?.`

```obsidian
const hp = world.find_player("Arin")?.hp        // hp: int? — none if player not found
const len = config.tags?.len()                  // len: int? — none if tags is none
```

---

## 26. Collections

### Dynamic Array — `[T]`

```obsidian
var names: [str] = []
names.push("Arin")
names.push("Rex")

const first = names[0]              // index — panics if out of bounds
const safe  = names.get(0)         // returns T? — safe
const len   = names.len()
names.pop()                         // removes and returns last element
names.remove(index: 1)
```

### Fixed Array — `[T; N]`

Stack-allocated, fixed size, zero overhead:

```obsidian
const points: [Vec2; 4] = [
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
]
```

### Map — `Map<K, V>`

```obsidian
var scores: Map<str, int> = Map::new()
scores.set(key: "Arin", value: 100)
const val = scores.get(key: "Arin")    // returns V?
scores.remove(key: "Arin")
const exists = scores.has(key: "Arin")

for (key, value in scores) {
    println("${key}: ${value}")
}
```

### Set — `Set<T>`

```obsidian
var seen: Set<str> = Set::new()
seen.add("Arin")
const has = seen.contains("Arin")    // true
seen.remove("Arin")
```

### Collection Pipeline Methods

All collections support the `-:` pipeline:

```obsidian
const result = users
    -: filter(|u| => u.active)
    -: map(|u| => u.email)
    -: sort()
    -: take(10)
    -: join(sep: "\n")
```

| Method | Description |
|--------|-------------|
| `.map(f)` | Transform each element |
| `.filter(p)` | Keep elements matching predicate |
| `.reduce(init, f)` | Fold to single value |
| `.find(p)` | First matching element → `T?` |
| `.any(p)` | True if any matches |
| `.all(p)` | True if all match |
| `.sort()` | Sort in place |
| `.sort_by(f)` | Sort by key function |
| `.take(n)` | First n elements |
| `.skip(n)` | Skip first n elements |
| `.join(sep)` | Join strings with separator |
| `.flatten()` | Flatten `[[T]]` to `[T]` |
| `.zip(other)` | Pair elements together |
| `.enumerate()` | Pair with index |
| `.first()` | First element → `T?` |
| `.last()` | Last element → `T?` |
| `.len()` | Element count |
| `.is_empty()` | True if length is 0 |

---

## 27. The `_` Discard Identifier

`_` discards a value explicitly. Using `_` signals intent to the reader and compiler.

```obsidian
const _, err = risky_call()       // discard value, keep error
const val, _ = risky_call()       // discard error — compiler warns in strict mode

for (_ in 0..10) { ... }          // discard loop variable

fn unused(_ : str) { }            // parameter intentionally unused
```

---

## 28. Operator Reference

### Arithmetic
| Op | Meaning |
|----|---------|
| `+` `-` `*` `/` | Standard arithmetic |
| `%` | Modulo |
| `**` | Exponentiation |

### Comparison
| Op | Meaning |
|----|---------|
| `==` `!=` | Equality |
| `<` `>` `<=` `>=` | Ordering |

### Logical
| Op | Meaning |
|----|---------|
| `&&` | Logical AND |
| `\|\|` | Logical OR |
| `!` | Logical NOT |

### Bitwise
| Op | Meaning |
|----|---------|
| `&` | Bitwise AND |
| `\|` | Bitwise OR |
| `^` | Bitwise XOR |
| `~` | Bitwise NOT |
| `<<` `>>` | Shift left / right |

### Obsidian-Specific
| Op | Meaning |
|----|---------|
| `-:` | Spark feed — pipeline operator |
| `::` | Namespace / static access |
| `<>` | String concatenation |
| `${}` | String interpolation (inside strings) |
| `?.` | Optional chain |
| `!!` | Assert non-null / assert success |
| `..` | Exclusive range |
| `..=` | Inclusive range |
| `=>` | Match arm result |
| `->` | Return type annotation |
| `~T` | Ownership transfer hint |
| `&T` | Borrow hint |
| `*T` | Mutable borrow hint |

---

## 29. Compiler Modes & Flags

```
obsidian build              compile project
obsidian run                build and execute
obsidian test               run all @test functions
obsidian fmt                format all source files
obsidian check              type-check without building
obsidian doc                generate documentation
```

### Compiler Flags

| Flag | Effect |
|------|--------|
| `--strict` | Unhandled `!` calls, unused `var`, discarded errors = errors |
| `--lax` | All error warnings suppressed — for prototyping |
| `--release` | Full optimisation, `!!` panics stripped in production |
| `--debug` | Debug symbols, bounds checks, panic messages |
| `--target=wasm` | Compile to WebAssembly |
| `--target=native` | Native binary (default) |
| `--no-stdlib` | Bare-metal — no standard library linked |

---

## 30. Standard Library Surface

### `std::io`
```obsidian
print(msg: str)
println(msg: str)
read() -> str
read_line() -> str!
stderr.println(msg: str)
```

### `std::fs`
```obsidian
fs::read(path: str) -> str!
fs::write(path: str, data: str) -> void!
fs::exists(path: str) -> bool
fs::delete(path: str) -> void!
fs::mkdir(path: str) -> void!
fs::list(path: str) -> [str]!
fs::open(path: str) -> File!
```

### `std::math`
```obsidian
sqrt(x: f64) -> f64
abs(x: T) -> T
clamp(val: T, lo: T, hi: T) -> T
min(a: T, b: T) -> T
max(a: T, b: T) -> T
floor / ceil / round
sin / cos / tan
log / log2 / log10
pow(base: f64, exp: f64) -> f64
PI  E  TAU  INFINITY
```

### `std::str`
```obsidian
str::parse<T>(s: str) -> T!
str::format(template: str, args: ...) -> str
str::repeat(s: str, n: int) -> str
```

### `std::collections`
```obsidian
List<T>  Map<K,V>  Set<T>  Queue<T>  Stack<T>
Heap<T>  RingBuffer<T>
```

### `std::async`
```obsidian
spawn(f: async fn) -> Handle<T>
sleep(ms: int) -> void
timeout(ms: int, f: async fn) -> T!
Channel<T>::new(capacity: int) -> Channel<T>
```

### `std::http` (stdlib extension)
```obsidian
http::get(url: str) -> Response!
http::post(url: str, body: str) -> Response!
http::Server::new(port: int) -> Server
```

### `std::env`
```obsidian
env::get(key: str) -> str?
env::args() -> [str]
env::cwd() -> str
```

---

## 31. Complete Keyword Index

| Keyword | Section | Purpose |
|---------|---------|---------|
| `package` | §2 | Declare file's package |
| `import` | §2 | Import names from another package |
| `const` | §4 | Immutable binding |
| `var` | §4 | Mutable binding |
| `fn` | §5 | Declare a function |
| `return` | §5 | Explicit return from function |
| `async` | §18 | Mark function as asynchronous |
| `await` | §18 | Suspend until future resolves |
| `spawn` | §18 | Launch concurrent task |
| `scope` | §18 | Structured concurrency block |
| `class` | §6 | Declare a data structure |
| `impl` | §7 | Attach behavior to a class |
| `interface` | §8 | Declare a method contract |
| `enum` | §9 | Declare a sum type |
| `self` | §23 | Current instance in impl |
| `Self` | §7 | Current class type in impl |
| `pub` | §21 | Public visibility |
| `priv` | §21 | Private visibility |
| `mut` | §22 | Mutable self / mutable borrow |
| `if` | §11 | Conditional branch |
| `else` | §11 | Alternate branch / error handler |
| `for` | §11 | Iteration loop |
| `while` | §11 | Condition loop |
| `loop` | §11 | Infinite loop |
| `break` | §11 | Exit loop |
| `continue` | §11 | Skip to next iteration |
| `match` | §12 | Pattern matching |
| `fail` | §13 | Return a failure |
| `pass` | §13 | Propagate failure to caller |
| `guard` | §13 | All-or-nothing block |
| `try` | §13 | Catch runtime panics only |
| `catch` | §13 | Handle a panic |
| `defer` | §14 | Schedule cleanup at scope exit |
| `alloc` | §14 | Explicit heap allocation |
| `free` | §14 | Explicit heap deallocation |
| `true` | §3 | Boolean true |
| `false` | §3 | Boolean false |
| `none` | §25 | Absence of optional value |
| `some` | §25 | Present optional — used in match |
| `void` | §3 | No return value |
| `never` | §3 | Function never returns |
| `any` | §3 | Dynamic type escape hatch |
| `in` | §11 | Range/collection iteration |
| `or` | §13/§25 | Fallback value |
| `comptime` | §19 | Compile-time evaluation keyword |
| `@comptime` | §19 | Decorator: evaluate at compile time |
| `@inline` | §20 | Decorator: force inline |
| `@cold` | §20 | Decorator: cold path hint |
| `@test` | §20 | Decorator: test function |
| `@deprecated` | §20 | Decorator: usage warning |
| `@derive` | §20 | Decorator: auto-implement |
| `@packed` | §20 | Decorator: no field padding |
| `@align` | §20 | Decorator: force alignment |
| `@extern` | §20 | Decorator: link to C symbol |
| `@doc` | §20 | Decorator: documentation |
| `@allow` | §20 | Decorator: suppress warning |
| `step` | §11 | Range step in for loops |
