extern crate crossterm;

use super::world;

pub trait WorldDisplay {
    fn display(&mut self, cells: &std::vec::Vec<world::Coord>, window: &mut world::Window);
}

pub struct TerminalDisplay<'a> {
    canvas: crossterm::Crossterm,
    terminal: crossterm::terminal::Terminal<'a>,
    input: crossterm::input::TerminalInput<'a>,
    cursor: crossterm::cursor::TerminalCursor<'a>
}

impl<'a> TerminalDisplay<'a> {
    pub fn new() -> TerminalDisplay<'a> {
        let mut terminal = crossterm::terminal::terminal();

        terminal.clear(crossterm::terminal::ClearType::All);

        TerminalDisplay{
            canvas: crossterm::Crossterm::new(),
            terminal: terminal,
            input: crossterm::input::input(),
            cursor: crossterm::cursor::cursor()
        }
    }

    pub fn best_window(&self) -> world::Window {
        let (w, h) = self.terminal.terminal_size();
        world::Window::new(0, 0, w as usize, h as usize)
    }

    pub fn update_window(&mut self, window: &mut world::Window) {
        if let Ok(c) = self.input.read_char() {
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

impl<'a> WorldDisplay for TerminalDisplay<'a> {
    fn display(&mut self, cells: &std::vec::Vec<world::Coord>, window: &mut world::Window) {
        self.terminal.clear(crossterm::terminal::ClearType::All);

        let x = window.x;
        let y = window.y;

        {
            let mut cursor = &self.cursor;

            cells.into_iter().for_each(|c| {
                let actual_x = c.0 - x;
                let actual_y = c.1 - y;
                cursor.goto(actual_x as u16, actual_y as u16);
                print!("@");
            });
        }

        self.cursor.goto(0, 0);
        println!("x: {}, y: {}, w: {}, h: {}", window.x, window.y, window.w, window.h);
    }
}
