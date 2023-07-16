use tui::style::{Color, Style};
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Rect};
use tui::widgets::{Block, Paragraph, Borders};
use tui::{Terminal, layout};
use std::io::{stdout, Stdout}; 
use crate::backend::messages::{Messages, Message}; 



pub fn generate_note_template() -> Result<Terminal<CrosstermBackend<Stdout>>, String>  {
    let mut term = match Terminal::new(CrosstermBackend::new(std::io::stdout())) {
        Ok(t) => t, 
        Err(e) => return Err(e.to_string()),
    };

    term.draw(|f| {
        let size = f.size(); 
        let chunks = Layout::default()
            .constraints([
                         Constraint::Length(3),
                         Constraint::Length(3),
                         Constraint::Min(3),
            ].as_ref()
            ).split(size);

    }).unwrap(); 

    Ok(term)
}

fn create_rect_for_notes(msg: Message) -> Rect {
    let rect: Rect = Rect::new(msg.get_x(), msg.get_y(), msg.get_width(), msg.get_height()); 
    rect
}

pub fn render_notes(msg: &Messages) {
    let mut term = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap(); 

    for m in msg.messages.clone() {
        term.draw(|f| {
                let license = Paragraph::new(m.get_message_text())
                    .alignment(layout::Alignment::Center)
                    .style(Style::default().bg(Color::Green).fg(Color::Red)); 

            f.render_widget(license, create_rect_for_notes(m.clone()));
        }).unwrap(); 
    }
}

