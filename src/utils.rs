use crate::rle;
use crate::world::{Coord, World};

pub struct WorldLifePlaceMaker<'a> {
    position: Coord,
    world: &'a mut World,
}

impl<'a> WorldLifePlaceMaker<'a> {
    pub fn new(position: Coord, world: &'a mut World) -> WorldLifePlaceMaker<'a> {
        WorldLifePlaceMaker { position, world }
    }
}

impl<'a> rle::LifePlaceMaker for WorldLifePlaceMaker<'a> {
    fn make_cell_alive(&mut self, coord: Coord) {
        self.world
            .make_alive(Coord(self.position.0 + coord.0, self.position.1 + coord.1));
    }
}
