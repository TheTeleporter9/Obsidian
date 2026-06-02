
# DEVELOPMENT NOTE: 
============================================================================
### Initial logic for this system was drafted using AI pseudocode.
### You can find it in the Research_Psudeocode.md file!
### The entire codebase has since been manually rewritten, refactored, and 
### engineered from scratch. Future development is entirely human-written.
============================================================================

## What is Obsidian?
## Description
Obsidian is a high-level programming language designed to simplify low-level development by enforcing Object-Oriented Programming (OOP) design patterns. It operates as a source-to-source transpiler written in Rust that outputs standard C code. At the moment is the language still very rudementary and Simple!

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

Please note this behavior will change in future versions; this is an alpha build.

![Restult](images/example_output.png)