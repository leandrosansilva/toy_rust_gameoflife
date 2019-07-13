extern crate crossterm;

use super::world;

pub trait WorldDisplay {
    fn display(&mut self, cells: &[world::Coord], window: &mut world::Window);
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
        self.terminal.clear(crossterm::terminal::ClearType::All);
    }

    pub fn best_window(&self) -> world::Window {
        let (w, h) = self.terminal.terminal_size();
        world::Window::new(0, 0, w as usize, h as usize)
    }

    pub fn update_window(&mut self, window: &mut world::Window) {
        if let Ok(c) = self.input.read_char() {
            match c {
                'w' => window.y -= 3,
                's' => window.y += 3,
                'a' => window.x -= 3,
                'd' => window.x += 3,
                'r' => {
                    window.x = 0;
                    window.y = 0;
                }
                _ => (),
            }
        }
    }
}

impl<'stdout> WorldDisplay for TerminalDisplay<'stdout> {
    fn display(&mut self, cells: &[world::Coord], window: &mut world::Window) {
        self.clear();

        let x = window.x;
        let y = window.y;

        {
            let cursor = &self.cursor;

            cells.iter().for_each(|c| {
                let actual_x = c.0 - x;
                let actual_y = c.1 - y;
                cursor.goto(actual_x as u16, actual_y as u16);
                print!("@");
            });
        }

        self.cursor.goto(0, 0);
        println!(
            "x: {}, y: {}, w: {}, h: {}",
            window.x, window.y, window.w, window.h
        );
    }
}
