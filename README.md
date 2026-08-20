# What is Obsidian
Obsidian (soon to be **Flint**) is a simple scripting language that transpiles it's backend into C. It is written in Rust.
This project is primaraly a learning experience and is still in its early stages. But many core features alreddy exist.

---

# Installation

## Requirements
* **Rust** (latest stable version)
* **Cargo**
* **GCC** (Preferably C99 or higier)
* **Git** (Optional, for cloning repo)


# Build the compiler
```bash
cargo build --release
```

Then find it in the release folder of the project.
and run the executiable!

At the release is a premade executiable, so just run that
to get the compiler going


Using the compiled executable:

### Linux

```bash
./target/release/obsidian /absolute/path/to/program.obs
```

### Windows

```powershell
target\release\obsidian.exe C:\Users\User\Example\Path\to\program.obs
```


---

# Current language features

## Variable Declaration

```obsidian
var <type> <name> = <expression>
```

Example:

```obsidian
var int number = 42
var float pi = 3.14159
var bool running = true
var string message = "Hello!"
```

## Supported types
* **int** - Whole numbers
* **float** - Decimal numbers
* **bool** - `true` or `false`
* **string** - Text enclosed in quotation marks

## Arithmetic

Supported operator:

```text
+ - * / 
```


Example:
```obsidian
var int result = 5 + 10 * 2
```

Arithmetic only works for numbers i.e int and float!

## Expression

Expression can be written without explicitly assigning the result.

```obsidian
x + 5
```

Currently the **first** variable that comes in the expression will receive the result/

The expression above is the same as below:

```obsidian
x = x + 5
```

> Note Make shore that the right variable comes first in the expression!

## Assignment

Variables can be reassigned.

```obsidian
x = 25

message = "Goodbye"

running = false
```


## Comparisons

Supported comparison operators:

```text
==
!=
<
<=
>
>=
```

Example:

```obsidian
var bool bigger = x > y
```


## Logical Operators

```text
and
or
!
```

Example:

```obsidian
var bool valid = x > 10 and !(y == 5)
```

Logical expressions are fully type checked.


## Strings

String literals are supported.

```obsidian
var string name = "Flint"

print name
```

Current limitations:

* No concatenation (`+`) yet
* No interpolation (`"Hello $name"`) yet
* No indexing
* No iteration

These features are coming soon

## Printing

Print any expression using:

```obsidian
print <expression>
```

Supported output:

* int
* float
* bool (`true` / `false`)
* string

Examples:

```obsidian
print 42

print 3.14

print true

print "Hello!"
```


## Type Checking

The compiler performs semantic analysis before generating C.

It currently checks:

* Variable declarations
* Assignments
* Arithmetic expressions
* Logical expressions
* Comparison expressions
* Unary operators
* Undefined variables
* Type mismatches

Compilation stops if a type error is detected.


# Example Program

```obsidian
# Hello World #

var int x = 10
var int y = 20

var float scale = 3.6

var string greeting = "Hello"

var bool valid = x < y and scale != 10 and greeting == "Hello"

print "Before"
print x

# Expression statements automatically assign back
# to the first variable (temporary behaviour)

x + 5 * 2

print "After"
print x

print greeting

print valid
```


![Result](images/example_output.png)
