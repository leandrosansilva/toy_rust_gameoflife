#[macro_use]
extern crate criterion;

use criterion::Criterion;

use gameoflife::world::*;

fn world_bench(c: &mut Criterion) {
    let mut world = World::new();

    for n in 0..100 {
        world.make_alive(Coord(n + 100, n));
        world.make_alive(Coord(n + 1 + 100, n));
        world.make_alive(Coord(n + 1 + 100, n + 12));
        world.make_alive(Coord(n + 1 + 30, n + 40));
    }

    world.make_alive(Coord(2, 0));
    world.make_alive(Coord(1, 1));
    world.make_alive(Coord(3, 1));
    world.make_alive(Coord(0, 2));
    world.make_alive(Coord(2, 2));
    world.make_alive(Coord(2, 3));
    world.make_alive(Coord(4, 2));
    world.make_alive(Coord(5, 1));
    world.make_alive(Coord(4, 4));
    world.make_alive(Coord(2, 5));
    world.make_alive(Coord(42, 3));
    world.make_alive(Coord(3, 42));

    world.finish();

    c.bench_function("do 1000 iterations", |b| {
        b.iter(|| {
            let mut world = World::new();

            for n in 0..1000 {
                world.evolve();
            }

            ()
        })
    });
}

criterion_group!(benches, world_bench);
criterion_main!(benches);
