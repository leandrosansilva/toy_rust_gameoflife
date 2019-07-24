use crate::common;
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
        let expected = vec![
            Coord(-1, -1),
            Coord(0, -1),
            Coord(1, -1),
            Coord(1, 0),
            Coord(1, 1),
            Coord(0, 1),
            Coord(-1, 1),
            Coord(-1, 0),
        ];

        assert_eq!(neighboors(Coord(0, 0)).iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn in_empty_world_no_cell_has_live_neighboors() {
        let mut ic = InterestingCells::new();
        ic.finish();

        let n = ic.live_neighboors(Coord(0, 0));

        assert_eq!(ic.dead.len(), 0);
        assert_eq!(n.count(), 0);
    }

    #[test]
    fn no_cell_has_no_neighboors() {
        let mut ic = InterestingCells::new();
        ic.make_alive(Coord(0, 0));
        ic.finish();

        let n = ic.live_neighboors(Coord(0, 0));

        assert_eq!(ic.dead.len(), 8);
        assert_eq!(n.count(), 0);
    }

    #[test]
    fn cell_with_one_neighboor() {
        let mut ic = InterestingCells::new();

        ic.make_alive(Coord(0, 0)).make_alive(Coord(1, 1));

        ic.finish();

        let n = ic.live_neighboors(Coord(0, 0)).collect::<Vec<_>>();

        let expected = vec![Coord(1, 1)];

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
pub struct Coord(pub common::Int, pub common::Int);

pub type Coords = std::vec::Vec<Coord>;

struct Neighbboors {
    c: Coord,
}

impl Neighbboors {
    fn iter(&self) -> NeighboorIter {
        NeighboorIter::empty(self.c)
    }
}

#[derive(Debug, Clone)]
struct NeighboorIter {
    c: Coord,
    i: usize,
}

impl NeighboorIter {
    fn empty(c: Coord) -> Self {
        Self { c, i: 0 }
    }
}

impl std::iter::Iterator for NeighboorIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 8 {
            return None;
        }

        let n: [(i8, i8); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ];

        let p = n[self.i];

        self.i += 1;

        Some(Coord(
            self.c.0 + p.0 as common::Int,
            self.c.1 + p.1 as common::Int,
        ))
    }
}
struct InterestingCells {
    alive: Coords,
    dead: Coords,
}

impl InterestingCells {
    fn live_neighboors<'a>(&'a self, c: Coord) -> impl Iterator<Item = Coord> + 'a {
        neighboors(c)
            .iter()
            .filter(move |c| self.alive.binary_search(c).is_ok())
    }

    fn new() -> Self {
        InterestingCells {
            alive: Coords::with_capacity(1000),
            dead: Coords::with_capacity(1000 * 8),
        }
    }

    fn make_alive(&mut self, c: Coord) -> &mut InterestingCells {
        self.alive.push(c);
        self
    }

    fn finish(&mut self) {
        self.make_dead_from_alive();
        let dead = &mut self.dead;
        let alive = &mut self.alive;

        alive.par_sort_unstable();

        dead.par_sort_unstable();
        dead.dedup();
        dead.retain(|c| alive.binary_search(c).is_err());
    }

    fn len(&self) -> usize {
        self.alive.len()
    }

    fn make_dead_from_alive(&mut self) {
        let alive = &self.alive;
        let dead = &mut self.dead;

        dead.extend(alive.iter().flat_map(|c| {
            neighboors(*c)
                .iter()
                .filter(|c| alive.binary_search(c).is_err())
        }));

        dead.par_sort_unstable();
        dead.dedup();
    }

    fn evolve_into(&self, e: &mut InterestingCells) {
        e.alive.clear();
        e.dead.clear();

        let alive = &self.alive;
        let dead = &self.dead;

        e.alive.par_extend(alive.into_par_iter().filter(|c| {
            mutate(CellState::Alive, self.live_neighboors(**c).count()) == CellState::Alive
        }));

        e.alive.par_extend(dead.into_par_iter().filter(|c| {
            mutate(CellState::Dead, self.live_neighboors(**c).count()) == CellState::Alive
        }));

        e.alive.par_sort_unstable();

        e.make_dead_from_alive();
    }
}

pub struct Window {
    pub w: usize,
    pub h: usize,
    pub x: common::Int,
    pub y: common::Int,
}

impl Window {
    pub fn new(x: common::Int, y: common::Int, w: usize, h: usize) -> Self {
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
            window.x + window.w as common::Int,
            window.y + window.h as common::Int + 1,
        ));

        assert!(lower_index <= upper_index);

        let slice = &alive[lower_index..upper_index];

        slice
            .iter()
            .filter(|c| {
                (c.0 >= window.x)
                    && (c.0 < window.x + window.w as common::Int)
                    && (c.1 >= window.y)
                    && (c.1 < window.y + window.h as common::Int)
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

fn neighboors(c: Coord) -> Neighbboors {
    Neighbboors { c }
}
