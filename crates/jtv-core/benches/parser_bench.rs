// Benchmark for JtV parser
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jtv_core::parse_program;

fn parse_simple_addition(c: &mut Criterion) {
    c.bench_function("parse simple addition", |b| {
        b.iter(|| {
            parse_program(black_box("x = 5 + 3")).unwrap();
        });
    });
}

fn parse_function(c: &mut Criterion) {
    let code = r#"
        fn add(a: Int, b: Int): Int {
            return a + b
        }
    "#;

    c.bench_function("parse function", |b| {
        b.iter(|| {
            parse_program(black_box(code)).unwrap();
        });
    });
}

fn parse_complex_program(c: &mut Criterion) {
    let code = r#"
        module Math {
            @pure fn add(a: Int, b: Int): Int {
                return a + b
            }

            fn factorial(n: Int): Int {
                result = 1
                for i in 2..n+1 {
                    result = result + i
                }
                return result
            }
        }

        import Math

        x = Math.add(5, 3)
        y = Math.factorial(10)
    "#;

    c.bench_function("parse complex program", |b| {
        b.iter(|| {
            parse_program(black_box(code)).unwrap();
        });
    });
}

criterion_group!(
    benches,
    parse_simple_addition,
    parse_function,
    parse_complex_program
);
criterion_main!(benches);
