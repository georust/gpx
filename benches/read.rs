#![feature(test)]

extern crate test;

const NITER: usize = 100;

#[bench]
fn bench_read(bencher: &mut test::Bencher) {
    let gpx_bytes = include_bytes!("../tests/fixtures/wikipedia_example.gpx");

    bencher.iter(|| {
        for _ in 0..NITER {
            test::black_box(gpx::read(&gpx_bytes[..]).unwrap());
        }
    });
}
