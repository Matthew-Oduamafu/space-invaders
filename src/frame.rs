use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    // let mut cols = vec![" "; NUM_COLS];
    // cols[0] = "X";
    // cols[NUM_COLS - 1] = "X";
    // let mut rows = vec![cols; NUM_ROWS];
    // rows[0] = vec!["X"; NUM_COLS];
    // rows[NUM_ROWS - 1] = vec!["X"; NUM_COLS];
    // rows

    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}