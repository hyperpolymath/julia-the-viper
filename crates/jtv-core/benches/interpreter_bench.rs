// Benchmark for JtV interpreter
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jtv_lang::{parse_program, Interpreter};

fn fibonacci_iterative(c: &mut Criterion) {
    let code = r#"
        fn fibonacci(n: Int): Int {
            if n <= 1 {
                return n
            }

            prev = 0
            curr = 1

            for i in 2..n+1 {
                next = prev + curr
                prev = curr
                curr = next
            }

            return curr
        }

        result = fibonacci(20)
    "#;

    let program = parse_program(code).unwrap();

    c.bench_function("fibonacci(20)", |b| {
        b.iter(|| {
            let mut interpreter = Interpreter::new();
            interpreter.run(black_box(&program)).unwrap();
        });
    });
}

fn factorial(c: &mut Criterion) {
    let code = r#"
        fn multiply(a: Int, b: Int): Int {
            result = 0
            for i in 0..b {
                result = result + a
            }
            return result
        }

        fn factorial(n: Int): Int {
            result = 1
            for i in 2..n+1 {
                result = multiply(result, i)
            }
            return result
        }

        result = factorial(10)
    "#;

    let program = parse_program(code).unwrap();

    c.bench_function("factorial(10)", |b| {
        b.iter(|| {
            let mut interpreter = Interpreter::new();
            interpreter.run(black_box(&program)).unwrap();
        });
    });
}

fn sum_range(c: &mut Criterion) {
    let code = r#"
        sum = 0
        for i in 1..1001 {
            sum = sum + i
        }
    "#;

    let program = parse_program(code).unwrap();

    c.bench_function("sum(1..1000)", |b| {
        b.iter(|| {
            let mut interpreter = Interpreter::new();
            interpreter.run(black_box(&program)).unwrap();
        });
    });
}

fn nested_loops(c: &mut Criterion) {
    let code = r#"
        total = 0
        for i in 1..11 {
            for j in 1..11 {
                total = total + 1
            }
        }
    "#;

    let program = parse_program(code).unwrap();

    c.bench_function("nested loops 10x10", |b| {
        b.iter(|| {
            let mut interpreter = Interpreter::new();
            interpreter.run(black_box(&program)).unwrap();
        });
    });
}

fn pure_function_calls(c: &mut Criterion) {
    let code = r#"
        @pure fn add(a: Int, b: Int): Int {
            return a + b
        }

        total = 0
        for i in 1..101 {
            total = add(total, i)
        }
    "#;

    let program = parse_program(code).unwrap();

    c.bench_function("100 pure function calls", |b| {
        b.iter(|| {
            let mut interpreter = Interpreter::new();
            interpreter.run(black_box(&program)).unwrap();
        });
    });
}

criterion_group!(
    benches,
    fibonacci_iterative,
    factorial,
    sum_range,
    nested_loops,
    pure_function_calls
);
criterion_main!(benches);
