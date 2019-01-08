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
        let ic = InterestingCells{
            alive: vec![],
            dead: vec![]
        };

        let n = ic.live_neighboors(Coord(0, 0));

        assert_eq!(n.len(), 0);
    }

    #[test]
    fn no_cell_has_no_neighboors() {
        let ic = InterestingCells{
            alive: vec![Coord(0,0)],
            dead: vec![]
        };

        let n = ic.live_neighboors(Coord(0, 0));

        assert_eq!(n.len(), 0);
    }

    #[test]
    fn cell_with_one_neighboor() {
        let ic = InterestingCells{
            alive: vec![Coord(0,0), Coord(1,1)],
            dead: vec![]
        };

        let n = ic.live_neighboors(Coord(0, 0));

        let expected: Neighboors = smallvec![Coord(1,1)];

        assert_eq!(n, expected);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord(i64, i64);

type Neighboors = smallvec::SmallVec<[Coord; 8]>;

struct InterestingCells {
    alive: std::vec::Vec<Coord>,
    dead: std::vec::Vec<Coord>
}

impl InterestingCells {
    fn live_neighboors(&self, c: Coord) -> Neighboors {
        // TODO: this filtering can be done in parallel
        let is_alive = |c:&Coord| {
            match self.alive.binary_search(c) {
                Ok(_) => true,
                Err(_) => false
            }
        };

        neighboors(c).into_iter().filter(|c| is_alive(c)).collect()
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
