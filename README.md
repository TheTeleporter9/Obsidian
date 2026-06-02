
# DEVELOPMENT NOTE: 
============================================================================
### Initial logic for this system was drafted using AI pseudocode.
### You can find it in the Research_Psudeocode.md file!
### The entire codebase has since been manually rewritten, refactored, and 
### engineered from scratch. Future development is entirely human-written.
============================================================================

## What is Obsidian?
Obsidian is a high-level programming language designed to simplify low-level development by enforcing Object-Oriented Programming (OOP) design patterns. It operates as a source-to-source transpiler written in Rust that outputs standard C code. At the moment is the language still very rudementary and Simple as it is a learning experiance!

## How to run:
First, download Rust.

Then download GCC (tested with version 13.3.0 on Ubuntu 24.04.1).

After that, create a file ending in `.obs` in the root directory and update the local path in the `main.rs` file.

Then run:
With cargo:
```bash
cargo run -- /abselute/path/to/file/
```
or with Binary:
```bash
obsidian /path/to/program.obs
```

## Language Syntax & Features
Core Syntax
is keywork driven

## Strings currently do not exists, it will cause an error!

```Variable Declaration: var <type> <name> = <value>```

### Types:
- #### bool -> either true or false (enforced)
- #### int -> any hole number
- #### float -> any fraction number

``` Example: var int y = 4 ```

#### Arithmetic Operations: +, -, *, /


#### To send output use: ``` print <expression> ```
### -> At the moment the print statment can't accept strings!

Example Code
Code snippet
```
# Hello World Program #
var bool x = true
var int y = 4
var int z = 10

# Arithmetic is permitted for numeric types #
var int result = z + y - 9

# Expressions can also be types wihtout needing a result var!#
z + y - 3 * 5 / z
# The first variable in the expression sequence will get the result assinged"

# The same as: #
z = z + y - 3 * 5 / z


# Prints the numeric result #
print result
```
Please note this behavior will change in future versions; this is an alpha build.

![Restult](images/example_output.png)
