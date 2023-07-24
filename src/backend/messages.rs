use crate::backend::config::Config; 
use serde::{Deserialize, Serialize};
use tui::{style::Color, text}; 
use crate::backend::constants::{SMALL_WIDTH, MID_WIDTH, WIDE_WIDTH};

use super::constants::{MID_HEIGHT, SMALL_HEIGHT, TALL_HEIGHT}; 

#[derive(Serialize, Deserialize, Debug, Clone,)]
pub struct Messages {
    pub messages: Vec<Message>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Message {
    message_id: u32,
    message_text: String,
    color: Color,
    x: u16,
    y: u16,
}

const TEXT_AREA_WIDTH_LOW: u16 = 50; 
const TEXT_AREA_WIDTH_HIGH: u16 = 75; 

impl Message {
    pub fn required_spaces(&self) -> u16 {
        let c: u16 = 3; 
        c
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

     pub fn get_x(&self) -> u16 {
         self.x
     }

     pub fn get_y(&self) -> u16 {
         self.y
     }

     pub fn get_width(&self) -> u16 {
        let text_length = self.get_message_text().len(); 
        return if text_length <= TEXT_AREA_WIDTH_LOW.into() {
            SMALL_WIDTH
        } else if text_length > TEXT_AREA_WIDTH_LOW.into() && text_length <= TEXT_AREA_WIDTH_HIGH.into() {
            MID_WIDTH
        } else {
            WIDE_WIDTH
        };
     }

     // Write an algorithm to make it more efficient. 
     pub fn get_height(&self) -> u16 {
         let w = self.get_width(); 
         return if w == SMALL_WIDTH {
             SMALL_HEIGHT
         } else if w == MID_WIDTH {
             MID_HEIGHT
         } else {
             TALL_HEIGHT
         }
     }

}

pub fn parse_config(config: Config) -> Result<Messages, String> {
    serde_json::from_str(config.get_config_text().as_str()).
        map_err(|e| e.to_string())
}


