//! Benchmarks for mobile wallet

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use silver_mobile::*;

fn bench_wallet_creation(c: &mut Criterion) {
    c.bench_function("wallet_creation", |b| {
        b.iter(|| {
            MobileWallet::new(black_box("ValidPass123"))
        });
    });
}

fn bench_transaction_creation(c: &mut Criterion) {
    c.bench_function("transaction_creation", |b| {
        let mut wallet = MobileWallet::new("ValidPass123").unwrap();
        wallet.set_balance(10000);
        
        b.iter(|| {
            wallet.create_transaction(
                black_box("silver_recipient"),
                black_box(1000),
                black_box(100),
            )
        });
    });
}

criterion_group!(benches, bench_wallet_creation, bench_transaction_creation);
criterion_main!(benches);
