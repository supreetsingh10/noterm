use tui::prelude::{Direction, Alignment};
use tui::style::{Color, Style};
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Rect};
use tui::widgets::{Block, Paragraph, Borders, Wrap};
use tui::{Terminal, layout};
use std::io::{stdout, Stdout}; 
use crate::backend::messages::{Messages, Message}; 

fn create_rect_for_notes(msg: &Message) -> Rect {
    let r: Rect = Rect::new(msg.get_x(), msg.get_y(), msg.get_width(), msg.get_height()); 
    r
}

pub fn render_notes(msg: &Messages) {
    let mut term = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap(); 
    
    term.draw(|f| {

        for m in msg.messages.clone().into_iter() {
            let rect = create_rect_for_notes(&m);
            let chunks = Layout::default().
                direction(Direction::Vertical).
                constraints([
                            Constraint::Ratio(1, 10),
                            Constraint::Ratio(5, 2),
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
                            alignment(Alignment::Left).
                            wrap(Wrap { trim: true }); 

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

