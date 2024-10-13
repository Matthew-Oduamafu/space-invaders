use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, cols) in current_frame.iter().enumerate() {
        for (y, cell) in cols.iter().enumerate() {
            if force || *cell != last_frame[x][y] {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *cell);
            }
        }
    }
    stdout.flush().unwrap();
}