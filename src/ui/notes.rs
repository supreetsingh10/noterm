use std::io::{stdout, Stdout};

use tui::prelude::{Direction, Alignment};
use tui::style::{Color, Style};
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Rect};
use tui::widgets::{Paragraph, Wrap};
use tui::{Terminal, layout};
use tui::prelude::Text; 
use crate::backend::messages::{Messages, Message}; 
use crate::backend::constants::WIDTH; 


// so we need a function that will handle the coordinate part of the function
// Paragraph will be inside a block. The dimensions of this block is fixed because of rectangle.

// the recent note is rendered first
// the width of the vector is fixed. 

// next line will be because of /n, or if it exceeds the width 
type coords = (u16, u16); 
fn count_note_lines(msg_text: &str) -> u16 {
    let mut height: u16 = 0; 
    let mut sen_len: u32 = 0;  
    for c in msg_text.chars().into_iter() {
        if c == '\n' || sen_len >= WIDTH.into() {
            sen_len = 0; 
            height += 1; 
        } else {
            sen_len += 1;
        }
    }

    height
}


// [(0, 3), (2, 0)]
pub fn render(msg: &Messages) {
    let mut term = Terminal::new(CrosstermBackend::new(stdout())).unwrap(); 
    let coord_vec = fix_coordinates_of_notes(&msg, &term).unwrap(); 
    // use these coordinates to render later.
    //
    term.draw(|f| {
        let rect = f.size(); 
    }); 

}

// Needs to be tested.
fn fix_coordinates_of_notes(msg: &Messages, term: &Terminal<CrosstermBackend<Stdout>>) -> Result<Vec<coords>, String> {
    let term_rect = match term.size() {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    }; 

    let mut coords_vec: Vec<coords> = Vec::new(); 
    let max_row_notes = term_rect.width / WIDTH; 
    let mut row_of_note: u16; 
    for (index, m) in msg.messages.clone().into_iter().enumerate() {
        let mut coord: coords = (0,0); 
        let height;
        row_of_note = (index as u16) / max_row_notes; 
        if row_of_note > 0 {
            height = (count_note_lines(&m.get_message_text())) + coords_vec[index - max_row_notes as usize].1; 
        } else {
            height = count_note_lines(&m.get_message_text());
        }

        let width = WIDTH * index as u16;
        coord.0 = width; 
        coord.1 = height;
        coords_vec.push(coord); 
    }

    Ok(coords_vec)
}
