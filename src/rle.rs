#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_life() {
        let mut storage = FakeStorage { cells: vec![] };
        let content = "x = 0, y = 0\n";
        let parsed = parse(content, &mut storage).unwrap();
        assert_eq!(parsed.x, 0);
        assert_eq!(parsed.y, 0);
        assert_eq!(storage.cells, vec![]);
    }

    #[test]
    fn parse_glider() {
        let content = r#"
#C lala
#N Pattern Name
x = 3, y = 3
bo$2bo$3o!

"#;
        let mut storage = FakeStorage { cells: vec![] };
        let parsed = parse(content, &mut storage).unwrap();
        assert_eq!(parsed.x, 3);
        assert_eq!(parsed.y, 3);

        use crate::world::Coord;

        assert_eq!(
            storage.cells,
            vec![
                Coord(1, 0),
                Coord(2, 1),
                Coord(0, 2),
                Coord(1, 2),
                Coord(2, 2)
            ]
        );
    }

    #[test]
    fn parse_gosper_glider() {
        let content = r#"
        #N Gosper glider gun
#C This was the first gun discovered.
#C As its name suggests, it was discovered by Bill Gosper.
x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
obo$10bo5bo7bo$11bo3bo$12b2o!
            "#;

        let mut storage = FakeStorage { cells: vec![] };
        let parsed = parse(content, &mut storage).unwrap();
        assert_eq!(parsed.x, 36);
        assert_eq!(parsed.y, 9);

        assert!(storage.cells.len() > 0);

    }

    #[test]
    fn parse_body() {
        LreFile::parse(Rule::Body, "bo$2bo$3o\n3o!").unwrap();
    }
}

pub trait LifePlaceMaker {
    fn make_cell_alive(&mut self, coord: Coord);
}

use crate::world::Coord;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "rle.pest"]
struct LreFile;

pub struct LreLife {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct FakeStorage {
    cells: Vec<Coord>,
}

impl LifePlaceMaker for FakeStorage {
    fn make_cell_alive(&mut self, coord: Coord) {
        self.cells.push(coord);
    }
}

fn get_x_y(pair: pest::iterators::Pair<Rule>) -> (i32, i32) {
    let mut inner = pair.into_inner();

    let mut number_from_node = || {
        let node = inner.next();
        node.unwrap().as_str().parse::<i32>().unwrap()
    };

    let x = number_from_node();
    let y = number_from_node();

    (x, y)
}

fn get_body_contents(node: pest::iterators::Pair<Rule>, storage: &mut LifePlaceMaker) {
    let patterns = node.into_inner().next().unwrap().into_inner();

    let mut line = 0i64;
    let mut column = 0i64;

    // TODO: refactor this loop and all those nested blocks to their own functions!
    for pattern in patterns {
        for pattern_type in pattern.into_inner() {
            match pattern_type.as_rule() {
                Rule::EndOfLinePattern => {
                    line += 1;
                    column = 0;
                }

                Rule::DeadOrAlive => {
                    let mut run_count = 1i64;
                    let mut should_add = false;

                    for component in pattern_type.into_inner() {
                        match component.as_rule() {
                            Rule::RunCount => {
                                run_count = component.as_str().parse::<i64>().unwrap();
                            }
                            Rule::Tag => {
                                if component.into_inner().next().unwrap().as_rule()
                                    == Rule::AliveTag
                                {
                                    should_add = true;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }

                    if should_add {
                        for c in 0..run_count {
                            let column = column + c;
                            storage.make_cell_alive(Coord(column, line));
                        }
                    }

                    column += run_count;
                }

                _ => unreachable!(),
            }
        }
    }
}

pub fn parse(
    content: &str,
    storage: &mut LifePlaceMaker,
) -> Result<LreLife, pest::error::Error<Rule>> {
    use pest::iterators::Pair;

    let p = LreFile::parse(Rule::File, content)?.next().unwrap();

    // TODO: define this in terms of for/match instead of manually unwrapping
    let mut inner = p.into_inner();
    inner.next();
    let node = inner.next();
    let size = get_x_y(node.unwrap());

    if let Some(node) = inner.next() {
        get_body_contents(node, storage);
    }

    Ok(LreLife {
        x: size.0,
        y: size.1,
    })
}
