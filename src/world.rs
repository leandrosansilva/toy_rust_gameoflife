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
        ic.make_alive(Coord(0,0));
        ic.finish();

        let n = ic.live_neighboors(Coord(0, 0));

        assert_eq!(ic.dead.len(), 8);
        assert_eq!(n.len(), 0);
    }

    #[test]
    fn cell_with_one_neighboor() {
        let mut ic = InterestingCells::new();

        ic.make_alive(Coord(0,0))
          .make_alive(Coord(1,1));

        ic.finish();

        let n = ic.live_neighboors(Coord(0, 0));

        let expected: Neighboors = smallvec![Coord(1,1)];

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

        let mut expected = vec![
            Coord(0, 0),
            Coord(1, 0),
            Coord(0, 1),
            Coord(1, 1)
        ];

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

        let mut expected = vec![
            Coord(1, 1)
        ];

        expected.sort();

        assert_eq!(ic2.alive, expected);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct Coord(i64, i64);

type Neighboors = smallvec::SmallVec<[Coord; 8]>;

struct InterestingCells {
    alive: std::vec::Vec<Coord>,
    dead: std::vec::Vec<Coord>
}

impl InterestingCells {
    fn live_neighboors(&self, c: Coord) -> Neighboors {
        // TODO: this filtering can be done in parallel
        neighboors(c).into_iter().filter(|c| !self.alive.binary_search(c).is_err()).collect()
    }

    fn new() -> InterestingCells {
        InterestingCells{
            alive: vec![],
            dead: vec![]
        }
    }

    fn make_alive(&mut self, c: Coord) -> &mut InterestingCells {
        neighboors(c).into_iter().for_each(|n| self.dead.push(n));
        self.alive.push(c);
        self
    }

    fn finish(&mut self) {
        // TODO: refactor this!!!
        let dead = &mut self.dead;
        let alive = &mut self.alive;

        alive.sort();
        alive.dedup();

        dead.sort();
        dead.dedup();

        dead.retain(|c| alive.binary_search(c).is_err());
    }

    fn clear(&mut self) {
        self.alive.clear();
        self.dead.clear();
    }

    fn evolve_into(&self, e: &mut InterestingCells) {
        e.clear();

        let alive = &self.alive;
        let dead = &self.dead;

        alive.into_iter().for_each(|c| {
            let count = self.live_neighboors(*c).len();

            if mutate(CellState::Alive, count as u8) == CellState::Alive {
               e.make_alive(*c);
            }
        });

        dead.into_iter().for_each(|c| {
            let count = self.live_neighboors(*c).len();

            if mutate(CellState::Dead, count as u8) == CellState::Alive {
               e.make_alive(*c);
            }
        });

        e.finish();
    }
}

#[derive(Debug, PartialEq)]
enum CellState {
    Dead,
    Alive
}

fn mutate(state: CellState, neighboors: u8) -> CellState {
    match state {
        CellState::Dead => match neighboors {
            3 => CellState::Alive,
            _ => CellState::Dead
        },
        CellState::Alive => match neighboors {
            2|3 => CellState::Alive,
            _ => CellState::Dead
        }
    }
}

fn neighboors(c: Coord) -> Neighboors {
    smallvec![
        Coord(c.0 - 1, c.1 - 1),
        Coord(c.0    , c.1 - 1),
        Coord(c.0 + 1, c.1 - 1),
        Coord(c.0 + 1, c.1),
        Coord(c.0 + 1, c.1 + 1),
        Coord(c.0    , c.1 + 1),
        Coord(c.0 - 1, c.1 + 1),
        Coord(c.0 - 1, c.1)
    ]
} 
