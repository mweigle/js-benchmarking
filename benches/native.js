function objectPropertyAccess() {
    let obj = { a: { b: { c: 1 } } };

    for (let i = 0; i < 1000; i++) {
        let value = obj.a.b.c;
        obj = { a: { b: { c: value } } };
    }

    return obj;
}

function stringManipulation() {
    let str = '';
    for (let i = 0; i < 30; i++) {
        str += 'abcdefghijklmnopqrstuvwxyz';
        str = str.split('').reverse().join('');
    }
    return str;
}

function math() {
    let baseNumber = Math.pow(4, 7);
    let result = 0;
    for (let i = baseNumber; i >= 0; i--) {		
        result += Math.atan(i) * Math.tan(i);
    };
    return result;
}

const benchmark = (func, params, iterations) => {
    let times = [];
    for (let i = 0; i < iterations; i++) {
        const t0 = performance.now()    
        func(params)
        const t1 = performance.now()
        times.push(t1 - t0);
    }
    times.sort();
    let median = times[times.length / 2];

    return `${func.name}: ${median}ms`
}

console.log(benchmark(math, null, 200));
console.log(benchmark(stringManipulation, null, 400));
console.log(benchmark(objectPropertyAccess, null, 1300)); // iteration count same as in criterion