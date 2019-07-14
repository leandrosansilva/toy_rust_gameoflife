use rayon::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_stays_dead() {
        assert_eq!(mutate(CellState::Dead, 0), CellState::Dead);
        assert_eq!(mutate(CellState::Dead, 1), CellState::Dead);
        assert_eq!(mutate(CellState::Dead, 2), CellState::Dead);
        assert_eq!(mutate(CellState::Dead, 4), CellState::Dead);
        assert_eq!(mutate(CellState::Dead, 5), CellState::Dead);
        assert_eq!(mutate(CellState::Dead, 6), CellState::Dead);
        assert_eq!(mutate(CellState::Dead, 7), CellState::Dead);
        assert_eq!(mutate(CellState::Dead, 8), CellState::Dead);
    }

    #[test]
    fn dead_cell_with_3_neighboord_lives() {
        assert_eq!(mutate(CellState::Dead, 3), CellState::Alive);
    }

    #[test]
    fn live_cell_dies() {
        assert_eq!(mutate(CellState::Alive, 0), CellState::Dead);
        assert_eq!(mutate(CellState::Alive, 1), CellState::Dead);
        assert_eq!(mutate(CellState::Alive, 4), CellState::Dead);
        assert_eq!(mutate(CellState::Alive, 5), CellState::Dead);
        assert_eq!(mutate(CellState::Alive, 6), CellState::Dead);
        assert_eq!(mutate(CellState::Alive, 7), CellState::Dead);
        assert_eq!(mutate(CellState::Alive, 8), CellState::Dead);
    }

    #[test]
    fn live_cell_lives() {
        assert_eq!(mutate(CellState::Alive, 2), CellState::Alive);
        assert_eq!(mutate(CellState::Alive, 3), CellState::Alive);
    }

    #[test]
    fn cell_neighbours() {
        let expected: Neighboors = smallvec![
            Coord(-1, -1),
            Coord(0, -1),
            Coord(1, -1),
            Coord(1, 0),
            Coord(1, 1),
            Coord(0, 1),
            Coord(-1, 1),
            Coord(-1, 0),
        ];

        assert_eq!(neighboors(Coord(0, 0)), expected);
    }

    #[test]
    fn in_empty_world_no_cell_has_live_neighboors() {
        let mut ic = InterestingCells::new();
        ic.finish();

        let n = ic.live_neighboors(Coord(0, 0));

        assert_eq!(ic.dead.len(), 0);
        assert_eq!(n.len(), 0);
    }

    #[test]
    fn no_cell_has_no_neighboors() {
        let mut ic = InterestingCells::new();
        ic.make_alive(Coord(0, 0));
        ic.finish();

        let n = ic.live_neighboors(Coord(0, 0));

        assert_eq!(ic.dead.len(), 8);
        assert_eq!(n.len(), 0);
    }

    #[test]
    fn cell_with_one_neighboor() {
        let mut ic = InterestingCells::new();

        ic.make_alive(Coord(0, 0)).make_alive(Coord(1, 1));

        ic.finish();

        let n = ic.live_neighboors(Coord(0, 0));

        let expected: Neighboors = smallvec![Coord(1, 1)];

        assert_eq!(ic.dead.len(), 12);
        assert_eq!(n, expected);
    }

    #[test]
    fn evolve_empty_world_into_empty_world() {
        let mut ic1 = InterestingCells::new();
        ic1.finish();

        let mut ic2 = InterestingCells::new();

        ic1.evolve_into(&mut ic2);

        assert_eq!(ic2.alive.len(), 0);
    }

    #[test]
    fn evolve_one_cell_world_into_empty_world() {
        let mut ic1 = InterestingCells::new();
        ic1.make_alive(Coord(0, 0));
        ic1.finish();

        let mut ic2 = InterestingCells::new();

        ic1.evolve_into(&mut ic2);

        assert_eq!(ic2.alive.len(), 0);
    }

    #[test]
    fn evolve_two_by_two_live_into_the_same_state() {
        let mut ic1 = InterestingCells::new();

        ic1.make_alive(Coord(0, 0));
        ic1.make_alive(Coord(1, 0));
        ic1.make_alive(Coord(0, 1));
        ic1.make_alive(Coord(1, 1));

        ic1.finish();

        let mut ic2 = InterestingCells::new();

        ic1.evolve_into(&mut ic2);

        let mut expected = vec![Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(1, 1)];

        expected.sort();

        assert_eq!(ic2.alive, expected);
    }

    #[test]
    fn evolve_to_one_cell_only() {
        let mut ic1 = InterestingCells::new();

        ic1.make_alive(Coord(1, 0));
        ic1.make_alive(Coord(0, 2));
        ic1.make_alive(Coord(2, 2));

        ic1.finish();

        let mut ic2 = InterestingCells::new();

        ic1.evolve_into(&mut ic2);

        let mut expected = vec![Coord(1, 1)];

        expected.sort();

        assert_eq!(ic2.alive, expected);
    }

    #[test]
    fn window_of_an_empty_world() {
        let mut world = World::new();
        world.finish();

        let mut cells = Coords::new();
        let window = Window::new(0, 0, 3, 4);

        world.live_cells(&window, &mut cells);

        assert_eq!(cells.len(), 0);
    }

    #[test]
    fn windows_of_one_cell_world() {
        let mut world = World::new();
        world.action(|world| {
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
        });

        let window = Window::new(2, 1, 3, 4);

        let mut cells = Coords::new();
        world.live_cells(&window, &mut cells);

        let expected = vec![
            Coord(2, 2),
            Coord(2, 3),
            Coord(3, 1),
            Coord(4, 2),
            Coord(4, 4),
        ];

        assert_eq!(cells, expected);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Coord(pub i64, pub i64);

pub type Coords = std::vec::Vec<Coord>;

type Neighboors = smallvec::SmallVec<[Coord; 8]>;

struct InterestingCells {
    alive: Coords,
    dead: Coords,
}

fn neighboors_to_dead_list(c: Coord, dead: &mut Coords) {
    neighboors(c).into_iter().for_each(|n| dead.push(n));
}

impl InterestingCells {
    fn live_neighboors(&self, c: Coord) -> Neighboors {
        // TODO: this filtering can be done in parallel
        neighboors(c)
            .into_iter()
            .filter(|c| self.alive.binary_search(c).is_ok())
            .collect()
    }

    fn new() -> Self {
        InterestingCells {
            alive: Coords::with_capacity(1000),
            dead: Coords::with_capacity(1000 * 8),
        }
    }

    fn build_dead_neighboors_from_alive(&mut self) {
        let alive = &self.alive;
        let mut dead = &mut self.dead;
        alive
            .iter()
            .for_each(|c| neighboors_to_dead_list(*c, &mut dead))
    }

    fn make_alive(&mut self, c: Coord) -> &mut InterestingCells {
        neighboors_to_dead_list(c, &mut self.dead);
        self.alive.push(c);
        self
    }

    fn finish(&mut self) {
        // TODO: refactor this!!!
        let dead = &mut self.dead;
        let alive = &mut self.alive;

        alive.par_sort_unstable();
        alive.dedup();

        dead.par_sort_unstable();
        dead.dedup();
        dead.retain(|c| alive.binary_search(c).is_err());
    }

    fn clear(&mut self) {
        self.alive.clear();
        self.dead.clear();
    }

    fn len(&self) -> usize {
        self.alive.len()
    }

    fn evolve_into(&self, e: &mut InterestingCells) {
        e.clear();

        let alive = &self.alive;
        let dead = &self.dead;

        e.alive.par_extend(alive.into_par_iter().filter(|c| {
            mutate(CellState::Alive, self.live_neighboors(**c).len()) == CellState::Alive
        }));
        e.alive.par_extend(dead.into_par_iter().filter(|c| {
            mutate(CellState::Dead, self.live_neighboors(**c).len()) == CellState::Alive
        }));

        e.build_dead_neighboors_from_alive();

        e.finish();
    }
}

pub struct Window {
    pub w: usize,
    pub h: usize,
    pub x: i64,
    pub y: i64,
}

impl Window {
    pub fn new(x: i64, y: i64, w: usize, h: usize) -> Self {
        Window { w, h, x, y }
    }
}

pub struct World {
    set1: InterestingCells,
    set2: InterestingCells,
    using_set1: bool,
    generation: usize,
}

pub trait CellStorage {
    fn add_cell(&mut self, cell: Coord);
}

impl CellStorage for Coords {
    fn add_cell(&mut self, cell: Coord) {
        self.push(cell);
    }
}

impl World {
    pub fn new() -> Self {
        World {
            set1: InterestingCells::new(),
            set2: InterestingCells::new(),
            using_set1: true,
            generation: 0,
        }
    }

    fn current_set(&self) -> &InterestingCells {
        if self.using_set1 {
            &self.set1
        } else {
            &self.set2
        }
    }

    fn working_sets(&mut self) -> (&mut InterestingCells, &mut InterestingCells) {
        if self.using_set1 {
            (&mut self.set1, &mut self.set2)
        } else {
            (&mut self.set2, &mut self.set1)
        }
    }

    pub fn make_alive(&mut self, c: Coord) {
        self.working_sets().0.make_alive(c);
    }

    fn finish(&mut self) {
        self.working_sets().0.finish();
    }

    pub fn action<F: Fn(&mut World)>(&mut self, f: F) {
        f(self);
        self.finish();
    }

    fn swap_sets(&mut self) {
        self.using_set1 = !self.using_set1;
    }

    pub fn evolve(&mut self) {
        let working_sets = self.working_sets();
        working_sets.0.evolve_into(working_sets.1);
        self.swap_sets();
        self.generation += 1;
    }

    pub fn population_size(&self) -> usize {
        self.current_set().len()
    }

    pub fn gen(&self) -> usize {
        self.generation
    }

    // FIXME: this method is very unoptimized
    pub fn live_cells(&self, window: &Window, cells: &mut CellStorage) {
        let alive = &self.current_set().alive;

        let find_index = |c: Coord| match alive.binary_search(&c) {
            Ok(index) => index,
            Err(index) => index,
        };

        let lower_index = find_index(Coord(window.x - 1, window.y));
        let upper_index = find_index(Coord(
            window.x + window.w as i64,
            window.y + window.h as i64 + 1,
        ));

        assert!(lower_index <= upper_index);

        let slice = &alive[lower_index..upper_index];

        slice
            .iter()
            .filter(|c| {
                (c.0 >= window.x)
                    && (c.0 < window.x + window.w as i64)
                    && (c.1 >= window.y)
                    && (c.1 < window.y + window.h as i64)
            })
            .for_each(|c| {
                cells.add_cell(*c);
            });
    }
}

#[derive(Debug, PartialEq)]
enum CellState {
    Dead,
    Alive,
}

fn mutate(state: CellState, neighboors: usize) -> CellState {
    match state {
        CellState::Dead => match neighboors {
            3 => CellState::Alive,
            _ => CellState::Dead,
        },
        CellState::Alive => match neighboors {
            2 | 3 => CellState::Alive,
            _ => CellState::Dead,
        },
    }
}

fn neighboors(c: Coord) -> Neighboors {
    smallvec![
        Coord(c.0 - 1, c.1 - 1),
        Coord(c.0, c.1 - 1),
        Coord(c.0 + 1, c.1 - 1),
        Coord(c.0 + 1, c.1),
        Coord(c.0 + 1, c.1 + 1),
        Coord(c.0, c.1 + 1),
        Coord(c.0 - 1, c.1 + 1),
        Coord(c.0 - 1, c.1)
    ]
}
