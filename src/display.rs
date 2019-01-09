extern crate crossterm;

use super::world;

pub trait WorldDisplay {
    fn display(&mut self, cells: &std::vec::Vec<world::Coord>, window: &mut world::Window);
}

pub struct TerminalDisplay {
    canvas: crossterm::Crossterm
}

impl TerminalDisplay {
    pub fn new() -> TerminalDisplay {
        TerminalDisplay{canvas: crossterm::Crossterm::new()}
    }

    pub fn best_window() -> world::Window {
        let terminal = crossterm::terminal::terminal();
        let (w, h) = terminal.terminal_size();
        world::Window::new(0, 0, w as usize, h as usize)
    }

    pub fn update_window(&mut self, window: &mut world::Window) {
        let mut input = crossterm::input::input();

        if let Ok(c) = input.read_char() {
            match c {
                'w' => window.y = window.y - 3,
                's' => window.y = window.y + 3,
                'a' => window.x = window.x - 3,
                'd' => window.x = window.x + 3,
                'r' => {window.x = 0; window.y = 0;},
                _ => ()
            }
        }
    }
}

impl WorldDisplay for TerminalDisplay {
    fn display(&mut self, cells: &std::vec::Vec<world::Coord>, window: &mut world::Window) {
        let mut cursor = crossterm::cursor::cursor();
        let mut terminal = crossterm::terminal::terminal();

        terminal.clear(crossterm::terminal::ClearType::All);

        cursor.goto(0, 0);
        println!("x: {}, y: {}, w: {}, h: {}", window.x, window.y, window.w, window.h);

        let x = window.x;
        let y = window.y;

        cells.into_iter().for_each(|c| {
            let actual_x = c.0 - x;
            let actual_y = c.1 - y;
            cursor.goto(actual_x as u16, actual_y as u16);
            print!("@");
        });
    }
}
