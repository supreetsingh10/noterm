use crate::backend::config::Config; 
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tui::{style::Color, text}; 

#[derive(Serialize, Deserialize, Debug, Clone,)]
pub struct Messages {
    pub messages: Vec<Message>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// Since it is vector, the last element of the vector will be the latest one. 
pub struct Message {
    message_id: u32,
    message_text: String,
    color: Color,
}


impl Message {
    pub fn get_message_id(&self) -> &u32 {
        &self.message_id
    }

    pub fn get_message_text(&self) -> &str {
        self.message_text.as_str()
    }

    pub fn get_note_color(&self) -> &tui::style::Color {
        &self.color 
    }
}

pub fn parse_config(config: Config) -> Result<Messages, String> {
    serde_json::from_str(config.get_config_text().as_str()).
        map_err(|e| e.to_string())
}


