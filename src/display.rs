extern crate crossterm;

use super::world;
use crate::common;

pub trait WorldDisplay {
    fn display(&mut self, cells: &[world::Coord], window: &mut world::Window, world: &world::World);
}

pub struct TerminalDisplay<'stdout> {
    terminal: crossterm::terminal::Terminal<'stdout>,
    input: crossterm::input::TerminalInput<'stdout>,
    cursor: crossterm::cursor::TerminalCursor<'stdout>,
}

impl<'stdout> TerminalDisplay<'stdout> {
    pub fn new() -> Self {
        TerminalDisplay {
            terminal: crossterm::terminal::terminal(),
            input: crossterm::input::input(),
            cursor: crossterm::cursor::cursor(),
        }
    }

    fn clear(&self) {
        let _ = self.terminal.clear(crossterm::terminal::ClearType::All);
    }

    pub fn best_window(&self, x: common::Int, y: common::Int) -> world::Window {
        let (w, h) = self.terminal.terminal_size();
        world::Window::new(x, y, w as usize, h as usize)
    }

    pub fn update_window(&mut self, window: &mut world::Window) {
        if let Ok(c) = self.input.read_char() {
            match c {
                'w' => window.y -= 3,
                's' => window.y += 3,
                'a' => window.x -= 3,
                'd' => window.x += 3,
                'W' => window.y -= 30,
                'S' => window.y += 30,
                'A' => window.x -= 30,
                'D' => window.x += 30,
                'R' => {
                    window.x = 0;
                    window.y = 0;
                }
                _ => (),
            }
        }
    }
}

impl<'stdout> WorldDisplay for TerminalDisplay<'stdout> {
    fn display(
        &mut self,
        cells: &[world::Coord],
        window: &mut world::Window,
        world: &world::World,
    ) {
        self.clear();

        let x = window.x;
        let y = window.y;

        {
            let cursor = &self.cursor;

            cells.iter().for_each(|c| {
                let actual_x = c.0 - x;
                let actual_y = c.1 - y;
                let _ = cursor.goto(actual_x as u16, actual_y as u16);
                print!("@");
            });
        }

        let _ = self.cursor.goto(0, 0);
        println!(
            "x: {}, y: {}, population: {}, gen: {}",
            window.x,
            window.y,
            world.population_size(),
            world.gen()
        );
    }
}
