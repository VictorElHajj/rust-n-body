use criterion::{black_box, criterion_group, criterion_main, Criterion};
use n_body::body::Body;
use n_body::cube::Cube;
use n_body::octree::OcTree;
use n_body::vector::Vector3;
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("quadtree insertion", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let mut qt = OcTree::new(Cube {
                pos: Vector3::new(-50.0, -50.0, -50.0),
                size: 100.0,
            });
            for i in 0..1000 {
                let b = Body {
                    id: i,
                    pos: Vector3::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0), 0.0),
                    vel: Vector3::zero(),
                    acc: Vector3::zero(),
                    mass: rng.gen_range(1.0..100.0),
                };
                qt.insert(black_box(b)).ok();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
