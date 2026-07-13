# Quartz — Syntax Guide

A small language that compiles to C. A handful of simple rules make it instantly recognizable — no bloat, no 40 keywords to memorize.

---

## The 3 things that make Quartz *Quartz*

1. **`var` / `const`** for variables — nothing else.
2. **`[ ]` for calling things**, `( )` for what a function hands back.
3. **Whatever's inside a function's `( )` at the end is what gets returned** — no `return` keyword, ever.

That's the whole identity. Everything below is just applying those three rules.

---

## 1. Variables

```
var int x = 5
var bool alive = true
var y = 100          # type inferred

const PI = 3.14159
```

`var` = can change. `const` = can't. That's it.

---

## 2. Operations

```
var result = x + y
x = x - y + 10 * (5 / 3)   # assigns to x, the first variable in the line

x += 5
x -= 1
x *= 2
x /= 2
```

Comparisons and logic look like you'd expect:

```
if x > y && alive {
    print "yep"
}
```

---

## 3. Functions

A function is a name, a block of parameters in `[ ]`, a body, and a `( )` at the
end holding whatever variable(s) should come back out.

```
func add {
    [int a, int b]
    result = a + b
}(result)
```

Call it with `[ ]`, same bracket as the parameter list:

```
var sum = add[10, 20]
print sum
```

No `return`. Whatever name sits in the final `( )` is what you get.

### More than one value back
```
func splitTime {
    [int totalSeconds]
    minutes = totalSeconds / 60
    seconds = totalSeconds % 60
}(minutes, seconds)

var m, s = splitTime[125]
```

### One-liners
```
func square { [int x] }(x * x)

print square[5]   # 25
```

---

## 4. Comments

```
# a single line

#:
  a block, for longer notes
:#
```

---

## 5. Control Flow

```
if x > 10 {
    print "big"
} else {
    print "small"
}

while x > 0 {
    x -= 1
}

for i in 0..5 {
    print i
}
```

---

## 6. Arrays

```
var nums = [1, 2, 3, 4]

print nums[0]
nums[0] = 10
print nums.len()

for n in nums {
    print n
}
```

---

## 7. Strings

```
var name = "Alice"
print "Hello, " + name
print "Hello, $name"     # interpolation works too
```

Strings are arrays of `char` underneath, so they work like arrays too:

```
var c = name[0]          # 'A'
print name.len()

for c in name {
    print c
}
```

---

## 8. Structs

The one addition beyond the original three rules — and it's needed for
anything nontrivial, including a compiler: you need a way to group data
(a token, a line, an AST node).

```
struct Token {
    type: int
    text: string
}

var t = Token { type: 1, text: "var" }
print t.text
```

Structs are plain data. No methods bolted onto them — if you need
behavior, write a function that takes the struct as a parameter:

```
func describe {
    [Token t]
    msg = "token: " + t.text
}(msg)
```

---

## 9. Files (needed to read source code)

```
var source = readFile["input.qz"]

for c in source {
    print c
}
```

That's the only I/O primitive worth naming here — read a file into a
string, then use the string/array/char tools above on it.

---

## 10. A Full Example

```
func factorial {
    [int n]
    if n <= 1 {
        result = 1
    } else {
        result = n * factorial[n - 1]
    }
}(result)

var num = 5
print "factorial of " + num + " is " + factorial[num]
```

Compiles to roughly:

```c
int factorial(int n) {
    int result;
    if (n <= 1) {
        result = 1;
    } else {
        result = n * factorial(n - 1);
    }
    return result;
}
```

---

## 11. Separating Code (modules)

Quartz has no classes and no separate module keyword — **a file is a
module.** That's it. One less concept to learn, and it maps directly
onto how you'll actually lay out a compiler (`lexer.qz`, `parser.qz`,
`codegen.qz`, `main.qz`).

Pull another file in with `use`:

```
# main.qz
use "lexer.qz"
use "parser.qz"

var tokens = tokenize[source]
```

Everything that file defines (`func`, `struct`, `const`) becomes
available by name, flat — no `import lexer.tokenize` ceremony. If two
files define something with the same name, qualify it with the
filename (no `.qz`):

```
var t = lexer.Token { type: 1, text: "var" }
```

That's the whole system: no `pub`/`private` visibility rules, no
package hierarchy, no `mod.rs`. A struct or function either exists in a
file you `use`, or it doesn't. If you outgrow "everything in a used
file is visible," that's the moment to add real visibility — not before.

### How this looks for the compiler itself
```
# main.qz
use "lexer.qz"
use "parser.qz"
use "codegen.qz"

var source = readFile["input.qz"]
var tokens = tokenize[source]
var tree = parse[tokens]
var output = generate[tree]
print output
```

Four files, each with one job, no namespacing rules to fight with.

---

## 12. External C Libraries (Raylib, etc.)

Since Quartz compiles to C, hooking up a C library is just declaring
the functions exist, with no body — an `extern` block:

```
extern "raylib.h" {
    func InitWindow { [int width, int height, string title] }()
    func WindowShouldClose { [] }(bool)
    func CloseWindow { [] }()
}

InitWindow[800, 600, "My Game"]
while !WindowShouldClose[] {
    # game loop
}
CloseWindow[]
```

One rule bends here, on purpose: a normal `func`'s `( )` holds a
**variable name** to return, because there's a body to pull it from.
An `extern` function has no body, so nothing to pull from — its `( )`
holds the **type** instead. That's the only place in Quartz where `( )`
means something different, and it's forced by there being no code to
read a variable out of.

The compiler just emits `#include "raylib.h"` and trusts the
signatures you wrote. Nothing fancier than that.

---

## 13. The Standard Library

Not a separate concept — the std lib is just `.qz` files the compiler
ships with, used the same way as your own:

```
use "std/array.qz"
use "std/string.qz"
```

A small core (`print`, `len`, `readFile`, basic math) is baked directly
into the compiler because it needs to talk to the C runtime. Everything
else — sorting, map/filter/reduce helpers, string splitting — is
ordinary Quartz code living in `std/`, no different from code you'd
write yourself. Once the compiler is self-hosted, most of `std/` is
just... more Quartz, written the same way you'd write your own module.

---

## 14. Packages

A package is a folder of `.qz` files. That's the entire model —
no manifest file, no version resolver, no registry:

```
use "raylib_bindings/raylib.qz"
use "mymath/vectors.qz"
```

Quartz looks for `raylib_bindings/` either next to your project or in
a shared `packages/` folder. Sharing a package with someone else means
handing them a folder. This is deliberately the least amount of
package management that still works — a real registry/versioning
story is worth building only once you've actually felt the pain of not
having one, not before.

---

## 15. Memory — the default

Quartz's default is **scope-based, automatic, and boring on purpose**:
anything you allocate (a struct, an array) is freed the moment the
scope that created it ends — no GC pauses, no reference counting,
nothing to configure.

```
func buildToken {
    [string text]
    t = Token { type: 1, text: text }
}(t)

func tokenize {
    [string source]
    var tokens = []
    # ...
    tokens.push(buildToken["var"])   # ownership moves into tokens
}(tokens)                            # tokens survives — it's what's returned
```

The rule is simple: **if it's named in the trailing `( )`, it survives
past the scope; if it isn't, it's freed when the `}` closes.** No
tracking, no cycles to worry about, because nothing is shared —
a value has exactly one owner at a time, and it moves when you return
it or push it into something else. This is what makes it possible to
bootstrap the compiler without writing a garbage collector first: the
lexer builds tokens, hands them to the parser, the parser's scope ends
and only the AST it returned survives. Everything transient just
evaporates.

This costs you one thing: no sharing a struct between two long-lived
places at once. For a compiler, a game's entity list, most normal
programs — you don't need that. If you do, that's what section 16 is for.

### Lists and strings follow the same rule

A string is a char array; a list is just an array. Same ownership,
same rule — one owner, freed when that owner's scope ends unless it
was returned or pushed somewhere else.

Here's the important part: **Quartz does not statically check any of
this.** There's no analyzer walking your function looking for
use-after-move, no lifetimes, nothing that can refuse to compile your
code over how you used a variable. That's the actual borrow checker,
and it's deliberately not here. What Quartz does is much dumber and
much lighter: it tracks *one* thing — which scope is responsible for
eventually freeing a buffer — and frees it there. That's it.

```
var a = [1, 2, 3]
var b = a
print a[0]   # compiles fine, runs fine — nothing stops you
```

If you write something that double-frees or reads freed memory, you
find out by running it and it misbehaving — same as C, same as
`unique_ptr` in C++ before you've made a mistake with it. The compiler
isn't reasoning about your whole function to catch it ahead of time.
You're trusted. `.copy()` exists for when *you* want to guarantee two
independent buffers, not because the compiler is forcing you to.

This is the actual tradeoff, stated plainly: Rust catches these bugs
before the program runs, at the cost of the compiler arguing with you.
Quartz catches them the way C does — at runtime, if at all — at the
cost of it being possible to write a bug the compiler won't stop you
from writing. That's the deal being made here on purpose.

---

## 16. `unsafe` — for people who want real control

Everything above is the whole language for 95% of code. But since
Quartz compiles straight to C, nothing stops you from dropping to raw
memory when you actually need to share, or outlive a scope, or hand a
buffer to a C library:

```
unsafe {
    var buf = alloc[1024]
    buf[0] = 65
    free[buf]
}
```

`unsafe` isn't a performance switch — the rest of Quartz is already
just C underneath. It's a **visibility** switch: the ownership rule
from section 15 stops being enforced inside the block, so you're on
your own for `alloc`/`free`, same as writing C directly. Outside
`unsafe`, the compiler can promise "this doesn't leak, this doesn't
dangle." Inside it, that promise ends at the `{`, and picks back up at
the `}` — anything you built inside can still be handed back out
through a normal `( )` return, and from that point on it's owned
normally again.

The point of marking it explicitly, rather than just always allowing
raw pointers, is the same reason `extern` gets its own block: so
anyone reading the code can tell at a glance which parts of the
program the compiler is watching, and which parts are on the honor
system.

---

## Quick Reference

| Thing | Syntax |
|---|---|
| Variable | `var int x = 5` |
| Constant | `const PI = 3.14159` |
| Function | `func name { [params] }(what's returned)` |
| Calling | `name[args]` |
| Comment | `# ...` |
| Array | `[1, 2, 3]`, access with `arr[i]` |
| Struct | `struct Name { field: type }` |
| File read | `readFile["path"]` |
| Module | `use "file.qz"`, qualify with `file.thing` on collision |
| External C lib | `extern "lib.h" { func Name { [params] }(type) }` |
| Package | folder of `.qz` files, `use "folder/file.qz"` |
| Memory (default) | freed at end of scope unless returned/pushed |
| Manual control | `unsafe { alloc[n], free[ptr] }` |
| Explicit copy | `b = a.copy()` — plain `=` moves, not copies |

---

## On bootstrapping (writing Quartz's compiler in Quartz)

A self-hosting compiler needs to: read a file into a string, walk it
character by character, group characters into tokens, build a tree out
of tokens, then print out C. Everything above is exactly enough for
that and nothing more:

- **`readFile`** gets you the source
- **strings-as-char-arrays** let you lex it
- **`struct`** lets you represent a `Token` or an AST node
- **arrays** hold your list of tokens / list of AST nodes
- **`func`** with recursion (already shown with `factorial`) is enough
  for a recursive-descent parser
- **`print`** is enough to emit generated C, one line at a time

Nothing else here is required to bootstrap. If a future feature request
doesn't serve "can Quartz still read a file, build a tree, and print C
with this," it's probably scope creep — the whole point is staying small
enough that a first version of the compiler (written in C or Rust) can
be rewritten in Quartz itself without the language having ballooned out
from under it.

---

## Why keep it this small

The goal isn't to out-feature C or Rust — it's that the moment you see `func x { [...] }(...)`
or a call written as `thing[args]`, you know it's Quartz. A small, consistent identity
beats a big feature list. New syntax only gets added if it's worth losing that simplicity.
