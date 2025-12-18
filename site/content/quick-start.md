---
title: Quick Start Guide
date: 2025-01-01
tags: [tutorial, getting-started]
draft: false
template: default
---

# Quick Start Guide

## Installation

Clone and build from source:

```
git clone https://github.com/Hyperpolymath/julia-the-viper
cd julia-the-viper
just build
```

Or use Nix:

```
nix develop
nix run github:Hyperpolymath/julia-the-viper
```

## Your First Program

Create `hello.jtv`:

```
x = 5
y = 3
result = x + y
print(result)
```

Run it:

```
jtv run hello.jtv
```

## Core Concepts

### Data Language (Total)

Data expressions cannot contain loops, conditionals, or impure function calls:

```
// Valid: Pure addition
user_input = 5
calculation = user_input + 10
```

### Control Language (Turing-complete)

Control statements can do anything:

```
sum = 0
for i in 1..11 {
    sum = sum + i
}
print(sum)  // 55
```

### Pure Functions

Mark functions as `@pure` to call them from Data context:

```
@pure fn double(x: Int): Int {
    return x + x
}

result = double(5) + double(3)  // 16
```
