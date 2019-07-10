#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_life() {
        let content = "x = 0, y = 0\n";
        let parsed = parse(content).unwrap();
        assert!(parsed.x == 0);
        assert!(parsed.y == 0);
    }

    #[test]
    fn parse_glider() {
        let content = r#"
#C lala
#N Pattern Name
x = 3, y = 3
bo$2bo$3o!

"#;
        let parsed = parse(content).unwrap();
        assert!(parsed.x == 3);
        assert!(parsed.y == 3);

        let mut storage = FakeStorage{cells: vec![]};
        parsed.add_cells(&mut storage);

        use crate::world::Coord;

        assert_eq!(storage.cells, vec![
                   Coord(1, 0),
                   Coord(2, 1),
                   Coord(0, 2), Coord(1, 2), Coord(2, 2)
        ]);
    }

    #[test]
    fn parse_body() {
        LreFile::parse(Rule::Body, "bo$2bo$3o!").unwrap();
    }
}

pub trait LifePlaceMaker {
    fn make_cell_alive(&mut self, coord: Coord);
}

use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "lre.pest"]
struct LreFile;

struct LreLife {
    x: i32,
    y: i32,
}

impl LreLife {
    fn add_cells(&self, storage: &mut LifePlaceMaker) {

    }
}

use crate::world::Coord;

#[derive(Debug)]
struct FakeStorage {
    cells: Vec<Coord>,
}

impl LifePlaceMaker for FakeStorage {
    fn make_cell_alive(&mut self, coord: Coord) {

    }
}

fn parse(content: &str) -> Result<LreLife, pest::error::Error<Rule>> {
    use pest::iterators::Pair;

    fn get_x_y(pair: Pair<Rule>) -> (i32, i32) {
        let mut inner = pair.into_inner();

        let mut number_from_node = || {
            let node = inner.next();
            node.unwrap().as_str().parse::<i32>().unwrap()
        };

        let x = number_from_node();
        let y = number_from_node();

        (x, y)
    }

    let p = LreFile::parse(Rule::File, content)?.next().unwrap();

    let mut inner = p.into_inner();
    inner.next();
    let node = inner.next();

    let size = get_x_y(node.unwrap());

    Ok(LreLife {x: size.0, y: size.1})
}
