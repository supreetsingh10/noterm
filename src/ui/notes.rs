use tui::prelude::{Direction, Alignment};
use tui::style::{Color, Style};
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Rect};
use tui::widgets::{Block, Paragraph, Borders};
use tui::{Terminal, layout};
use std::io::{stdout, Stdout}; 
use crate::backend::messages::{Messages, Message}; 

// Create a basic Rect which has the specified layout and it will split it.
// the size of the rectangle has to depend on the messages.
fn create_rect_for_notes(msg: &Message) -> Rect {
    let r: Rect = Rect::new(msg.get_x(), msg.get_y(),40,16); 
    r
}


    //term.draw(|f| {
    //    // Make a unique layout for every note.
    //    let r = create_rect_for_notes(msg); 
    //    let chunks = Layout::default().direction(Direction::Horizontal)
    //        // the constraints have to set dynamically
    //        .constraints([
    //              Constraint::Length(25),
    //              Constraint::Length(25),
    //              Constraint::Length(25),
    //              Constraint::Length(25),

    //        ].as_ref()).split(r); 

    //    for c in chunks.into_iter() {
    //        let license = Paragraph::new() 
    //            .alignment(layout::Alignment::Center)
    //            .style(Style::default().bg(Color::Green).fg(Color::Red)); 

    //        f.render_widget(license, *c); 
    //    }

    //}).unwrap(); 


// how about we make rectangle for a single note, then add constraints to it, render it and keep
pub fn render_notes(msg: &Messages) {
    let mut term = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap(); 
    
    term.draw(|f| {

        for m in msg.messages.clone().into_iter() {
            let rect = create_rect_for_notes(&m);
            let chunks = Layout::default().
                direction(Direction::Vertical).
                constraints([
                            Constraint::Ratio(1, 4),
                            Constraint::Ratio(2, 4),
                            Constraint::Ratio(1, 4),
                ].
                as_ref()).
                split(rect);

            for (index, r) in chunks.into_iter().enumerate() {
                match index {
                    0 => {
                        let title = Paragraph::new(m.get_message_id().to_string()).
                            style(Style::default().bg(*m.get_note_color())).
                            alignment(Alignment::Center);

                        f.render_widget(title, *r); 
                    }, 
                    1 => {
                        let mess = Paragraph::new(m.get_message_text().clone()).
                            style(Style::default().bg(*m.get_note_color())).
                            alignment(Alignment::Left);

                        f.render_widget(mess, *r); 
                    } 2 => {
                        let mess = Paragraph::new(m.get_message_text().clone()).
                            style(Style::default().bg(*m.get_note_color())).
                            alignment(Alignment::Left);

                        f.render_widget(mess, *r); 
                    }
                    _ => {
                        println!("Missed it"); 
                    }
                }
            }

        }
    }).unwrap();
}

