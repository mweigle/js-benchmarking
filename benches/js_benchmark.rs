use boa_engine::{Context, Source};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn run_js(code: &str) {
    let mut context = Context::default();

    context.eval(Source::from_bytes(code)).unwrap();
    context.eval(Source::from_bytes("main()")).unwrap();
}

fn js_benchmark(c: &mut Criterion) {
    c.bench_function("math", |b| {
        b.iter(|| {
            run_js(black_box(
                r#"
        function main() {
            let baseNumber = Math.pow(4, 7);
            let result = 0;
            for (let i = baseNumber; i >= 0; i--) {		
                result += Math.atan(i) * Math.tan(i);
            };
            return result;
        }
        "#,
            ))
        })
    });
    c.bench_function("string manipulation", |b| {
        b.iter(|| {
            run_js(black_box(
                r#"
        function main() {
            let str = '';
            for (let i = 0; i < 30; i++) {
                str += 'abcdefghijklmnopqrstuvwxyz';
                str = str.split('').reverse().join('');
            }
            return str;
        }
        "#,
            ))
        })
    });
    c.bench_function("object property access", |b| {
        b.iter(|| {
            run_js(black_box(
                r#"
        function main() {
            let obj = { a: { b: { c: 1 } } };

            for (let i = 0; i < 1000; i++) {
                let value = obj.a.b.c;
                obj = { a: { b: { c: value } } };
            }
        
            return obj;
        }
        "#,
            ))
        })
    });
}

criterion_group!(benches, js_benchmark);
criterion_main!(benches);
