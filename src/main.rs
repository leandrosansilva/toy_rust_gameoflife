use gameoflife::rle;
use gameoflife::world::{World, Coord, Coords};
use gameoflife::utils::{WorldLifePlaceMaker};

fn main() {
    use gameoflife::display::*;
    use std::env;
    use std::fs;

    let mut world = World::new();

    world.action(|world| {
        let args: Vec<String> = env::args().collect();
        let filename = &args[1];
        let mut placemaker = WorldLifePlaceMaker::new(Coord(0, 0), world);
        let life_content = fs::read_to_string(filename).unwrap();
        let _ = rle::parse(&life_content, &mut placemaker).unwrap();
    });

    let mut display = TerminalDisplay::new();
    let mut window = display.best_window(0, 0);
    let mut cells = Coords::new();

    for _ in 1..=10 {
        //loop {
        println!(
            "Generation {}, population: {}",
            world.gen(),
            world.population_size()
        );
        cells.clear();
        world.live_cells(&window, &mut cells);
        //display.display(&cells, &mut window, &world);
        world.evolve();
        //display.update_window(&mut window);
    }
}
