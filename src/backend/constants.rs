pub const CONFIG_PATH: &str = "/.config/sticky_notes/config.json"; 
pub const ENV_HOME_VAR: &str = "HOME"; 

pub const NOTE_WIDTH: u16 = 40;

#[derive(PartialEq, Debug)]
pub enum SubCommandNames {
    SHOW,
    PIN,
    UPDATE,
}



