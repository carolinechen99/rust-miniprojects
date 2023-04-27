# Calculator

## Overview

A calculator that performs basic arithmetic operations. Users enter a command from the command line, and the program returns the result of the operation. The calculator supports four operations: addition, subtraction, multiplication, and division.

## Usage

```bash
cargo run -- --x <x> --y <y> <SUBCOMMAND>
```

x and y are the two numbers that the user wants to perform the operation on. SUBCOMMAND is the operation that the user wants to perform on the two numbers, which can be one of the following: `add`, `sub`, `mul`, `div`, where `add` is addition, `sub` is subtraction, `mul` is multiplication, and `div` is division.

Example:

```bash
cargo run -- --x 5 --y 3 add
```

will return 8.
