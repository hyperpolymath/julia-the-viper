# Julia the Viper - Quick Start Guide

## What is JtV?

Julia the Viper is a programming language that makes code injection **grammatically impossible** by separating:

- **Control Language**: Loops, conditionals, I/O (Turing-complete)
- **Data Language**: Pure expressions, addition-only (Total/provably halting)

This architectural separation provides formal security guarantees that runtime checks cannot match.

## Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Hyperpolymath/julia-the-viper
cd julia-the-viper

# Build using Just (requires just command runner)
just build

# Or build directly with Cargo
cd packages/jtv-lang
cargo build --release

# The CLI binary will be at: target/release/jtv
# Or run via cargo: cargo run --release --bin jtv
```

### Using Nix (Reproducible Build)

```bash
# Enter development shell
nix develop

# Or run directly
nix run github:Hyperpolymath/julia-the-viper
```

### Requirements

- **Rust 1.70+** - for building from source
- **Just** (optional) - task runner for build commands
- **Nix** (optional) - for reproducible builds

## Your First JtV Program

Create `hello.jtv`:

```jtv
// Addition is the fundamental operation
x = 5
y = 3
result = x + y

print(result)  // Output: 8
```

Run it:

```bash
jtv run hello.jtv
```

## Core Concepts

### 1. Data Language (Total)

Data expressions **cannot** contain:
- Loops
- Conditionals
- Function calls to impure functions
- Assignments

This grammatically prevents code injection:

```jtv
// ✅ VALID: Pure addition
user_input = 5
calculation = user_input + 10

// ❌ INVALID: Would be parse error
// calculation = user_input + eval("malicious code")
// Parser rejects this before execution!
```

### 2. Control Language (Turing-complete)

Control statements **can** do anything:

```jtv
// Loops are fine in Control context
sum = 0
for i in 1..11 {
    sum = sum + i  // Data expression: pure addition
}
print(sum)  // 55
```

### 3. Pure Functions

Mark functions as `@pure` to call them from Data context:

```jtv
@pure fn double(x: Int): Int {
    return x + x  // Only addition allowed
}

// Can use in Data expressions
result = double(5) + double(3)  // 16
```

### 4. Number Systems

JtV supports 7 number types:

```jtv
int_val = 42
float_val = 3.14
rational_val = 1/3         // Exact fractions
complex_val = 3+4i         // Complex numbers
hex_val = 0xFF             // Hexadecimal
binary_val = 0b1010        // Binary
symbolic_val = x           // Symbolic math (v2)
```

## Common Patterns

### Multiplication via Addition

```jtv
fn multiply(a: Int, b: Int): Int {
    result = 0
    for i in 0..b {
        result = result + a
    }
    return result
}

product = multiply(7, 8)  // 56
```

### Factorial

```jtv
fn factorial(n: Int): Int {
    result = 1
    for i in 2..n+1 {
        result = multiply(result, i)
    }
    return result
}
```

### Maximum of Two Numbers

```jtv
@pure fn max(a: Int, b: Int): Int {
    if a > b {
        return a
    } else {
        return b
    }
}
```

## Smart Contract Example

```jtv
module TokenTransfer {
    fn transfer(from: Int, to: Int, amount: Int) {
        from_balance = 1000  // Fetch from storage
        to_balance = 500

        // Grammatically impossible to bypass this check
        if from_balance >= amount {
            from_balance = from_balance - amount
            to_balance = to_balance + amount

            // Store updated balances
            // No reentrancy possible - operations are atomic
        }
    }
}
```

**Why this is secure:**
1. Balance check cannot be injected/bypassed (grammar enforces it)
2. Arithmetic is provably correct (addition-only, no overflow tricks)
3. No reentrancy (all operations are atomic in Data context)

## Type System

```jtv
// Explicit types
fn add(a: Int, b: Int): Int {
    return a + b
}

// Type inference
fn double(x) {
    return x + x  // Type inferred from usage
}

// Complex types
fn process_list(items: List<Int>): Int {
    sum = 0
    for item in items {
        sum = sum + item
    }
    return sum
}
```

## Modules

```jtv
// math.jtv
module Math {
    @pure fn abs(x: Int): Int {
        if x < 0 {
            return -x
        } else {
            return x
        }
    }
}

// main.jtv
import Math

result = Math.abs(-42)  // 42
```

## WASM Integration

### JavaScript

```javascript
import { JtvWasm } from '@jtv/wasm';

const jtv = new JtvWasm();
jtv.run(`
    x = 5
    y = 3
    result = x + y
`);

console.log(jtv.get_variable('result'));  // "8"
```

### Rust

```rust
use jtv_lang::{parse_program, Interpreter};

let code = "x = 5 + 3";
let program = parse_program(code).unwrap();

let mut interpreter = Interpreter::new();
interpreter.run(&program).unwrap();

let x = interpreter.get_variable("x").unwrap();
println!("{}", x);  // 8
```

## V2: Reversible Computing

```jtv
// Quantum computing simulation
x = 5

reverse {
    x += 10  // Forward: x = 15
    x += 5   // Forward: x = 20

    // Reverse execution automatically inverts:
    // x -= 5  (x = 15)
    // x -= 10 (x = 5)
}

// Enables Bennett's trick, Grover's algorithm, etc.
```

## Next Steps

- **Basic Examples**: Start with `examples/basic/` for fundamentals
- **Advanced Examples**: See `examples/advanced/` for fibonacci, matrices, sorting
- **Smart Contracts**: Check `examples/contracts/` for ERC-20, NFT, DAO examples
- **Integration Examples**: See `examples/integrations/` for Python/JS interop
- **Wiki**: Browse `wiki/` for in-depth documentation
- **Standard Library**: Explore `packages/jtv-lang/stdlib/` for built-in functions

## Getting Help

- **GitHub Issues**: https://github.com/Hyperpolymath/julia-the-viper/issues
- **Documentation**: See `docs/` directory and `wiki/`
- **CLAUDE.md**: Project overview for AI assistants and new contributors

## Philosophy

> "Code injection isn't prevented by careful programming - it's prevented by making it grammatically impossible to express."

JtV proves that security and expressiveness are not mutually exclusive. By structurally separating decidable from undecidable computation, we achieve:

1. **Formal guarantees**: Provable termination, no injection
2. **Performance**: Aggressive optimization of pure Data expressions
3. **Simplicity**: Addition-only is universal with Control loops
4. **Reversibility**: Quantum computing simulation (v2)

Welcome to the future of secure computing.
