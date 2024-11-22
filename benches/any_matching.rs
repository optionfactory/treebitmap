use std::hint::black_box;
use std::net::Ipv6Addr;
use criterion::{criterion_group, criterion_main, Criterion};
use mytreebitmap::IpLookupTable;

fn criterion_benchmark(c: &mut Criterion) {
    let mut tbl = IpLookupTable::new();
    let needle = Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0x0, 0x0, 0x8a2e, 0x0370, 0x7334);
    tbl.insert(needle, 128, ());
    for i in 0..100 {
        let ip = Ipv6Addr::new(0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xffff, i);
        tbl.insert(ip, 128, ());
    }

    c.bench_function("any_matching", |b| b.iter(||
       assert!( black_box(tbl.any_matched_by(black_box(needle), 64)))
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);