use std::io::{stdout, Stdout};
use tui::prelude::{Direction, Alignment};
use tui::style::{Color, Style};
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Rect};
use tui::widgets::{Paragraph, Wrap};
use tui::{Terminal, layout};
use crate::backend::messages::{Messages, Message}; 
use crate::backend::constants::NOTE_WIDTH; 


// so we need a function that will handle the coordinate part of the function
// Paragraph will be inside a block. The dimensions of this block is fixed because of rectangle.

// the recent note is rendered first
// the width of the vector is fixed. 

// next line will be because of /n, or if it exceeds the width 
const INIT_X:u16 = 0; 
const INIT_Y:u16 = 0; 
//              x,y,width,height
type Coords = (u16, u16, u16, u16); 
fn count_note_lines(msg_text: &str) -> u16 {
    let mut height: u16 = 0; 
    let mut sen_len: u32 = 0;  
    for c in msg_text.chars().into_iter() {
        if c == '\n' || sen_len >= NOTE_WIDTH.into() {
            sen_len = 0; 
            height += 1; 
        } else {
            sen_len += 1;
        }
    }

    height
}

fn rects_for_notes(coord: Coords) -> Rect {
    Rect::new(coord.0, coord.1, coord.2, coord.3)
}

pub fn render(msg: &Messages) -> Result<(), String> {
    let mut term = match Terminal::new(CrosstermBackend::new(stdout())) {
        Ok(t) => t,
        Err(s) => return Err(s.to_string()),
    };

    let coord_vec = match fix_coordinates_of_notes(&msg, &mut term) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    term.draw(|f| {
        for (m_it, c_it) in msg.messages.clone().iter().zip(coord_vec.iter()) {
            let rect = rects_for_notes(c_it.to_owned()); 
            let chunks = Layout::default().
                direction(Direction::Vertical).
                constraints([
                            Constraint::Ratio(1,10),
                            Constraint::Ratio(5, 2),
                ].as_ref()).
                split(rect);

            for (index, r) in chunks.into_iter().enumerate() {
                match index {
                    0 => {
                        let title = Paragraph::new(m_it.get_message_id().to_string()).
                            style(Style::default().bg(*m_it.get_note_color())).
                            alignment(Alignment::Center);

                        f.render_widget(title,*r); 
                    },
                    1 => {
                        let body = Paragraph::new(m_it.get_message_text().to_string()).
                            style(Style::default().bg(*m_it.get_note_color())).
                            alignment(Alignment::Left).
                            wrap(Wrap {trim: true});

                        f.render_widget(body,*r); 
                    },
                    _ => {
                        println!("Missed it"); 
                    }
                }
            }
        }
    }).
    map(|_| {
        ()
    }).
    map_err(|e| {
        e.to_string()
    })

}

/// Needs to be tested.
// Make sure that it does not go out of the window
// Check if we can get more space in the terminal if try to render something longer than 60 pixels.
fn fix_coordinates_of_notes(msg: &Messages, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<Vec<Coords>, String> {
    let term_rect = match term.size() {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    }; 

    let mut coords_vec: Vec<Coords> = Vec::new(); 
    let max_notes_in_row = term_rect.width / NOTE_WIDTH; 
    let mut row_of_note: u16; 
    let mut row_change: bool = false; 
    let mut last_row: u16 = 0; 
    for (index, m) in msg.messages.clone().into_iter().enumerate() {
        let mut coord: Coords = (0,0,0,0); 
        let height;
        row_of_note = (index as u16) / max_notes_in_row; 

        if row_of_note == last_row {
            row_change = false; 
        } else {
            last_row = row_of_note;
            row_change = true; 
        }

        // height will be a variable for every note
        height = count_note_lines(&m.get_message_text());

        let width = NOTE_WIDTH; 
        coord.2 = width; 
        coord.3 = height + 10;


        if index == 0 {
            coord.0 = INIT_X;
            coord.1 = INIT_Y;
        } else {
            if row_change {
                // x,y,width,height
                // set the new x and y of the note as the last x of the note above and y to be the
                // y of the note above + height of the last note. 
                coord.0 = coords_vec[index - max_notes_in_row as usize].0; 
                coord.1 = coords_vec[index - max_notes_in_row as usize].1 + coords_vec[index - max_notes_in_row as usize].3; 
            } else {
                // x will be the x of last note + it's width.
                // y will be the y of the last note. 
                coord.0 = coords_vec[index - 1].0 + coords_vec[index - 1].2; 
                coord.1 = coords_vec[index - 1].1; 
            }
        }
        coords_vec.push(coord); 
    }

    Ok(coords_vec)
}
