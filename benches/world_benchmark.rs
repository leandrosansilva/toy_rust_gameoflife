#[macro_use]
extern crate criterion;

use criterion::Criterion;

use gameoflife::rle;
use gameoflife::utils::*;
use gameoflife::world::*;

fn lwss_tagalong_bench(c: &mut Criterion) {
    let mut world = World::new();

    world.action(|world| {
        let life_content = r#"
#N LWSS tagalong
#O David Bell
#C A tagalong for two lightweight, middleweight, or heavyweight spaces
#C hips.
#C www.conwaylife.com/wiki/index.php?title=Lightweight_spaceship
x = 25, y = 19, rule = b3/s23
21bo3b$18b4o3b$13bo2bob2o5b$13bo11b$4o8bo3bob2o5b$o3bo5b2ob2obobob5o$o
9b2obobobo2b5o$bo2bo2b2o2bo3b3o2bob2ob$6bo2bob2o12b$6bo4b2o12b$6bo2bob
2o12b$bo2bo2b2o2bo3b3o2bob2ob$o9b2obobobo2b5o$o3bo5b2ob2obobob5o$4o8bo
3bob2o5b$13bo11b$13bo2bob2o5b$18b4o3b$21bo!
            "#;

        let mut placemaker = WorldLifePlaceMaker::new(Coord(0, 0), world);
        let _ = rle::parse(&life_content, &mut placemaker).unwrap();
    });

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

criterion_group!(benches, lwss_tagalong_bench);
criterion_main!(benches);
