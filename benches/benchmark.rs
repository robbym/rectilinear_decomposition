use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use ::rand::prelude::*;
use rectilinear_decomposition::{generate_random_horizontally_convex_polygon, RectilinearPolygon};

fn criterion_benchmark(c: &mut Criterion) {
    let mut seed = [0u8; 32];
    // ::rand::thread_rng().fill(&mut seed);

    let vertices = generate_random_horizontally_convex_polygon(seed, 100000, 10);

    // print number of vertices
    println!("Number of vertices: {}", vertices.len());

    c.bench_function("RP", |b| {
        b.iter(|| {
            let rp = RectilinearPolygon::from_vertices(&vertices);
            black_box(rp);
        })
    });

    // c.bench_function("RPC", |b| b.iter(|| {
    //     let mut rp = RectilinearPolygon::from_vertices(&vertices);
    //     rp.find_vchords();
    //     black_box(rp);
    // }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
