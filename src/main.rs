fn main() {
    use gameoflife::world::*;
    use gameoflife::display::*;

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

    let mut display = TerminalDisplay::new();

    let mut window = display.best_window();
    let mut cells: std::vec::Vec<Coord> = std::vec::Vec::new();

    loop {
        cells.clear();
        world.live_cells(&window, &mut cells);
        display.display(&cells, &mut window);
        world.evolve();
        display.update_window(&mut window);
    }
}
