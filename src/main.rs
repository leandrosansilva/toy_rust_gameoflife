use gameoflife::rle;
use gameoflife::world::*;

struct WorldLifePlaceMaker<'a> {
    position: Coord,
    world: &'a mut World,
}

impl<'a> WorldLifePlaceMaker<'a> {
    fn new(position: Coord, world: &'a mut World) -> WorldLifePlaceMaker<'a> {
        WorldLifePlaceMaker { position, world }
    }
}

impl<'a> rle::LifePlaceMaker for WorldLifePlaceMaker<'a> {
    fn make_cell_alive(&mut self, coord: Coord) {
        self.world
            .make_alive(Coord(self.position.0 + coord.0, self.position.1 + coord.1));
    }
}

fn main() {
    use gameoflife::display::*;
    use std::env;
    use std::fs;

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut world = World::new();

    world.action(|world| {
        let mut placemaker = WorldLifePlaceMaker::new(Coord(0, 0), world);
        let life_content = fs::read_to_string(filename).unwrap();
        let _ = rle::parse(&life_content, &mut placemaker).unwrap();
    });

    let mut display = TerminalDisplay::new();
    let mut window = display.best_window(0, 0);
    let mut cells = Coords::new();

    //for i in 1..200 {
    loop {
        cells.clear();
        world.live_cells(&window, &mut cells);
        display.display(&cells, &mut window);
        world.evolve();
        display.update_window(&mut window);
        //println!("Generation {}", i);
    }
}
