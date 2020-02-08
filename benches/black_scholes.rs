use black_box as b;
use black_scholes::{
    black_scholes_call_div, black_scholes_put_div, euro_vanilla_call, euro_vanilla_put,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct NoDivInput {
    S: f64,
    K: f64,
    T: f64,
    r: f64,
    sigma: f64,
}

struct DivInput {
    S: f64,
    K: f64,
    T: f64,
    r: f64,
    q: f64,
    sigma: f64,
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // TODO: random inputs
    let (S, K, T, r, q, sigma) = (b(50.), b(100.), b(1.), b(0.05), b(0.06), b(0.25));

    c.bench_function("call no-div", |b| {
        b.iter(|| euro_vanilla_call(S, K, T, r, sigma))
    });
    c.bench_function("put no-div", |b| {
        b.iter(|| euro_vanilla_put(S, K, T, r, sigma))
    });
    c.bench_function("call div", |b| {
        b.iter(|| black_scholes_call_div(S, K, T, r, q, sigma))
    });
    c.bench_function("put div", |b| {
        b.iter(|| black_scholes_put_div(S, K, T, r, q, sigma))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
