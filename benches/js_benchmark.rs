use boa_engine::{Context, Source};
use criterion::{criterion_group, criterion_main, Criterion};

fn js_benchmark(c: &mut Criterion) {
    let mut context = Context::default();
    context
        .eval(Source::from_bytes(
            r#"
    function math() {
        let baseNumber = Math.pow(4, 7);
        let result = 0;
        for (let i = baseNumber; i >= 0; i--) {		
            result += Math.atan(i) * Math.tan(i);
        };
        return result;
    }

    function stringManipulation() {
        let str = '';
        for (let i = 0; i < 30; i++) {
            str += 'abcdefghijklmnopqrstuvwxyz';
            str = str.split('').reverse().join('');
        }
        return str;
    }

    function objectPropertyAccess() {
        let obj = { a: { b: { c: 1 } } };

        for (let i = 0; i < 1000; i++) {
            let value = obj.a.b.c;
            obj = { a: { b: { c: value } } };
        }
    
        return obj;
    }

    function randomNumbers() {
        let list = [];
        iterations = 2000;
        for (let i = 0; i < iterations; i++) {
            let randomFloat = Math.random();
            list.push(randomFloat);
        }
        return list;
    }
    "#,
        ))
        .unwrap();
    c.bench_function("math", |b| {
        b.iter(|| context.eval(Source::from_bytes("math()")))
    });
    c.bench_function("string manipulation", |b| {
        b.iter(|| context.eval(Source::from_bytes("stringManipulation()")))
    });
    c.bench_function("object property access", |b| {
        b.iter(|| context.eval(Source::from_bytes("objectPropertyAccess()")))
    });
    c.bench_function("random numbers", |b| {
        b.iter(|| context.eval(Source::from_bytes("randomNumbers()")))
    });
}

criterion_group!(benches, js_benchmark);
criterion_main!(benches);
