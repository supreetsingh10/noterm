pub const CONFIG_PATH: &str = "/.config/sticky_notes/config.json"; 
pub const ENV_HOME_VAR: &str = "HOME"; 

pub const SMALL_WIDTH: u16 = 30;
pub const MID_WIDTH: u16 = 35;
pub const WIDE_WIDTH: u16 = 40;

pub const SMALL_HEIGHT: u16 = 10;
pub const MID_HEIGHT: u16 = 15;
pub const TALL_HEIGHT: u16 = 20;

#[derive(PartialEq, Debug)]
pub enum CommandNames {
    SHOW,
    PIN,
}



