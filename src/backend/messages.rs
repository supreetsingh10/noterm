use crate::backend::config::Config;
use serde::{Deserialize, Serialize};
use std::{env::var, io::Write};
use tui::style::Color;

use super::constants::{CONFIG_PATH, ENV_HOME_VAR};

pub type Commfun = fn(&Messages) -> Result<(), String>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Messages {
    pub messages: Vec<Message>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// Since it is vector, the last element of the vector will be the latest one.
pub struct Message {
    message_id: u32,
    message_text: String,
    color: Color,
}

impl Message {
    pub fn new(id: u32, m_text: String, colour: tui::style::Color) -> Self {
        Message {
            message_id: id,
            message_text: m_text,
            color: colour,
        }
    }

    pub fn get_message_id(&self) -> &u32 {
        &self.message_id
    }

    pub fn get_message_text(&self) -> &str {
        self.message_text.as_str()
    }

    pub fn get_note_color(&self) -> &tui::style::Color {
        &self.color
    }

    pub fn update_note_text(&mut self, text: String) {
        self.message_text = text;
    }

    pub fn update_note_color(&mut self, color: tui::style::Color) {
        self.color = color;
    }

    pub fn update_note_number(&mut self, num: usize) {
        self.message_id = num as u32;
    }
}

pub fn parse_config(config: Config) -> Result<Messages, String> {
    serde_json::from_str(config.get_config_text().as_str()).map_err(|e| e.to_string())
}

pub fn write_new_config(par_mess: &Messages) -> Result<(), String> {
    var(ENV_HOME_VAR)
        .map(|mut v| {
            v.push_str(CONFIG_PATH);
            v
        })
        .map_err(|var_e| var_e.to_string())
        .map(|path| {
            std::fs::File::create(path).map(|mut f| {
                let config_text = match serde_json::to_string(par_mess) {
                    Ok(ct) => ct,
                    Err(e) => {
                        println!("Gets wrapped in err");
                        e.to_string()
                    }
                };
                f.write_all(config_text.as_bytes())
            });
        })
        .map_err(|e| e.to_string())
}
