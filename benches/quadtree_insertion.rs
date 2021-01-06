use criterion::{black_box, criterion_group, criterion_main, Criterion};
use n_body::body::Body;
use n_body::quadtree::QuadTree;
use n_body::rectangle::Rectangle;
use n_body::vector::Vector2;
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("quadtree insertion", |b| {
        let mut qt = QuadTree::new(Rectangle {
            pos: Vector2::new(-50.0, -50.0),
            size: 100.0,
        });
        let mut rng = rand::thread_rng();
        b.iter(|| {
            for _ in 0..1000 {
                let b = Body {
                    pos: Vector2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0)),
                    vel: Vector2::zero(),
                    mass: rng.gen_range(1.0..100.0),
                };
                qt.insert(black_box(b)).ok();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
