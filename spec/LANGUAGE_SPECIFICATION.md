# Julia the Viper Language Specification

**Version:** 1.0.0
**Date:** 2026-03-14

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Lexical Structure](#2-lexical-structure)
3. [Syntax](#3-syntax)
4. [Type System](#4-type-system)
5. [Semantics](#5-semantics)
6. [Purity and Effects](#6-purity-and-effects)
7. [Memory Model](#7-memory-model)
8. [Concurrency Model](#8-concurrency-model)
9. [Module System](#9-module-system)
10. [Error Handling](#10-error-handling)
11. [Standard Library](#11-standard-library)
12. [Examples](#12-examples)

---

## 1. Introduction

Julia the Viper (JtV) is a statically typed, interpreted language with a Rust-based interpreter and a pest-based parser. It follows a Harvard architecture, separating code and data to enhance security and predictability.

### Key Features
- **Harvard Architecture**: Separation of code (Control Language) and data (Data Language).
- **Seven Number Systems**: Support for `Int`, `Float`, `Rational`, `Complex`, `Hex`, `Binary`, and `Symbolic`.
- **Purity System**: Annotations for `@total`, `@pure`, and `@impure` functions.
- **Reversible Computing**: `reverse` blocks for invertible operations.
- **Type Safety**: Strong static typing with decidable type checking.

---

## 2. Lexical Structure

### Character Set
JtV uses Unicode UTF-8 encoding. Source files are interpreted as UTF-8.

### Comments
- Single-line comments: `// comment`
- Multi-line comments: `/* comment */`

### Identifiers
- Must start with a letter or underscore: `[a-zA-Z_][a-zA-Z0-9_]*`
- Case-sensitive

### Keywords
```
fn, let, if, else, while, for, in, return, print, reverse, @total, @pure, @impure
```

### Literals
- **Integers**: `42`, `-7`
- **Floats**: `3.14`, `-2.718`
- **Rationals**: `3/4`, `22/7`
- **Complex**: `3+4i`, `2.5-1.2i`
- **Hexadecimal**: `0xFF`, `0x1A`
- **Binary**: `0b1010`, `0b11`
- **Symbolic**: `x`, `π`, `e`
- **Booleans**: `true`, `false`
- **Strings**: `"hello"`, `"world"`
- **Unit**: `()`

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Assignment: `=`, `+=`, `-=`

---

## 3. Syntax

### Program Structure
A JtV program consists of a sequence of declarations and statements:

```ebnf
Program ::= (Declaration | Statement)*
```

### Declarations

#### Function Declaration
```ebnf
FunctionDecl ::= "fn" Identifier "(" (Parameter ("," Parameter)*)? ")" (":" Type)? ("@" Purity)? "{" Statement* "}"
Parameter ::= Identifier ":" Type
Purity ::= "total" | "pure" | "impure"
```

Example:
```jtv
fn add(x: Int, y: Int): Int @pure {
    return x + y
}
```

#### Variable Declaration
```ebnf
VariableDecl ::= "let" Identifier (":" Type)? "=" Expression
```

Example:
```jtv
let x: Int = 42
```

### Statements

#### Expression Statement
```ebnf
ExpressionStmt ::= Expression
```

#### Assignment Statement
```ebnf
AssignmentStmt ::= Identifier "=" Expression
```

#### If Statement
```ebnf
IfStmt ::= "if" Expression "{" Statement* "}" ("else" "{" Statement* "}")?
```

#### While Statement
```ebnf
WhileStmt ::= "while" Expression "{" Statement* "}"
```

#### For Statement
```ebnf
ForStmt ::= "for" Identifier "in" Expression ".." Expression "{" Statement* "}"
```

#### Return Statement
```ebnf
ReturnStmt ::= "return" Expression
```

#### Print Statement
```ebnf
PrintStmt ::= "print(" Expression ")"
```

#### Reverse Block
```ebnf
ReverseStmt ::= "reverse" "{" Statement* "}"
```

### Expressions

#### Literal Expression
```ebnf
LiteralExpr ::= IntegerLiteral | FloatLiteral | RationalLiteral | ComplexLiteral | HexLiteral | BinaryLiteral | SymbolicLiteral | BooleanLiteral | StringLiteral | UnitLiteral
```

#### Variable Expression
```ebnf
VariableExpr ::= Identifier
```

#### Binary Expression
```ebnf
BinaryExpr ::= Expression ("+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | ">" | "<=" | ">=") Expression
```

#### Unary Expression
```ebnf
UnaryExpr ::= ("-" | "!") Expression
```

#### Function Call Expression
```ebnf
FunctionCallExpr ::= Identifier "(" (Expression ("," Expression)*)? ")"
```

#### List Expression
```ebnf
ListExpr ::= "[" (Expression ("," Expression)*)? "]"
```

#### Tuple Expression
```ebnf
TupleExpr ::= "(" (Expression ("," Expression)*)? ")"
```

---

## 4. Type System

### Type Language

```ebnf
Type ::= "Int" | "Float" | "Rational" | "Complex" | "Hex" | "Binary" | "Symbolic" | "Bool" | "String" | "Unit" | "List<" Type ">" | "(" Type ("," Type)* ")" | "(" Type ("," Type)* ")" "->" Type | "Any"
```

### Type Hierarchy

- **Numeric Types**: `Int`, `Float`, `Rational`, `Complex`, `Hex`, `Binary`, `Symbolic`
- **Compound Types**: `List<T>`, `(T1, T2, ..., Tn)`
- **Function Types**: `(T1, T2, ..., Tn) -> Tr`
- **Top Type**: `Any`

### Subtyping

- `Int` ≤ `Float` ≤ `Complex`
- `Hex` ≤ `Int`
- `Binary` ≤ `Int`
- `List<T>` ≤ `List<T'>` if `T` ≤ `T'`
- `(T1, T2, ..., Tn) -> Tr` ≤ `(T1', T2', ..., Tn') -> Tr'` if `T1'` ≤ `T1`, `T2'` ≤ `T2`, ..., `Tn'` ≤ `Tn`, and `Tr` ≤ `Tr'`

### Type Inference

JtV uses a bidirectional type checking algorithm:
- **Data Expressions**: Inferred using `infer_data_expr()`
- **Control Statements**: Checked using `check_control_stmt()`

---

## 5. Semantics

### Data Language Semantics

The Data Language is total and addition-only:
- **Termination**: All Data expressions terminate.
- **Operations**: Only addition and unary negation are allowed.

### Control Language Semantics

The Control Language is Turing-complete:
- **Termination**: Undecidable in general.
- **Operations**: Includes loops, conditionals, and I/O.

### Evaluation Rules

#### Data Expressions

```
E-Int: n ⟶ n
E-Var: x ⟶ σ(x)
E-Add: e1 + e2 ⟶ e1' + e2 (if e1 ⟶ e1')
E-Add: n1 + e2 ⟶ n1 + e2' (if e2 ⟶ e2')
E-Add: n1 + n2 ⟶ n1 + n2
E-Neg: -e ⟶ -e' (if e ⟶ e')
E-Neg: -n ⟶ -n
```

#### Control Statements

```
E-IfTrue: if true { s1 } else { s2 } ⟶ s1
E-IfFalse: if false { s1 } else { s2 } ⟶ s2
E-While: while e { s } ⟶ if e { s; while e { s } } else { skip }
E-For: for i in start..end { s } ⟶ s[i/start]; for i in start+1..end { s }
E-Return: return e ⟶ e
E-Print: print(e) ⟶ print(e') (if e ⟶ e')
```

---

## 6. Purity and Effects

### Purity Levels

- **@total**: Terminates for all valid inputs, no side effects.
- **@pure**: No side effects, may not terminate.
- **@impure**: May perform any effect, including I/O and non-termination.

### Effect System

- **IO**: Input/output operations.
- **State**: Mutable state modifications.
- **Diverge**: May not terminate.

### Effect Typing Rules

```
E-Pure: Γ ⊢ e : τ ! ∅
E-IO: Γ ⊢ print(e) : Γ ! IO
E-Loop: Γ ⊢ while e { s } : Γ ! ε ∪ Diverge
```

---

## 7. Memory Model

### Value-Based Memory Management
- **Cloned Values**: Values are cloned on assignment and when passing arguments to functions.
- **No Shared State**: Harvard architecture enforces separation between code and data.
- **Stack-Allocated Numerics**: Integers and floats are stored directly on the Rust stack.
- **Heap-Allocated Compounds**: Strings, lists, and maps are heap-allocated.
- **Environment Ownership**: Each scope owns its local variables.
- **No Garbage Collector**: Memory is managed through Rust's ownership system.

---

## 8. Concurrency Model

### Sequential Execution
- **No Threads or Async**: JtV does not currently support concurrency.
- **Reverse Blocks**: Execute statements in reverse order, but do not introduce parallelism.

---

## 9. Module System

### File-Based Modules
- Each source file acts as an implicit module.
- No explicit exports; all top-level definitions are visible to importers.
- File inclusion via a basic include mechanism.

---

## 10. Error Handling

### Compile-Time Errors
- **Type Errors**: Mismatched types, undefined variables.
- **Syntax Errors**: Invalid syntax, missing tokens.
- **Purity Violations**: Calling impure functions from pure contexts.

### Runtime Errors
- **Division by Zero**: Attempting to divide by zero.
- **Out of Bounds**: Accessing list indices out of bounds.
- **File Not Found**: Attempting to read a non-existent file.

---

## 11. Standard Library

### Built-in Functions
- **Numeric Operations**: `add`, `sub`, `mul`, `div`, `mod`
- **Comparison Operations**: `eq`, `neq`, `lt`, `gt`, `leq`, `geq`
- **Logical Operations**: `and`, `or`, `not`
- **List Operations**: `length`, `append`, `prepend`, `reverse`
- **I/O Operations**: `print`, `read`, `write`

---

## 12. Examples

### Example 1: Simple Function

```jtv
fn add(x: Int, y: Int): Int @pure {
    return x + y
}

let result = add(3, 4)
print(result)  // Output: 7
```

### Example 2: Reverse Block

```jtv
fn reversible_swap(x: Int, y: Int): (Int, Int) @reversible {
    x += y
    y -= x
    x += y
    return (x, y)
}

let (a, b) = reversible_swap(3, 4)
print(a)  // Output: 4
print(b)  // Output: 3
```

### Example 3: Purity Annotations

```jtv
fn factorial(n: Int): Int @total {
    if n == 0 {
        return 1
    }
    return n * factorial(n - 1)
}

let fact5 = factorial(5)
print(fact5)  // Output: 120
```

---

## References

1. [System Specifications](system-specs.md)
2. [Type System](type-system.md)
3. [Computational Theory](COMPUTATIONAL_THEORY.md)
4. [Type Theory](TYPE_THEORY.md)
