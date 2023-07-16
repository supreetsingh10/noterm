use clap::{Arg, App};
use libra::backend::commands; 
use libra::backend::config::init_config;
use libra::backend::messages;
use libra::ui::notes; 

fn main() -> Result<(), String> {
    // this will have argument parsing.
    // backend will have command selection

    let matches = App::new("Sticky term").
        about("Make sticky notes for your terminal").
        arg(Arg::with_name("pin").
            short("p").
            long("pin").
            takes_value(true)).
        arg(Arg::with_name("show").
            short("s").
            long("show").
            takes_value(false)).
        get_matches();

    let lazy_config = match init_config() {
        Ok(c) => c,
        Err(e) => {
            return Err(e);
        }
    }; 

    let par_cmds = match commands::parsed_commands(&matches) {
        Ok(com) => com,
        Err(e) => {
            return Err(e);
        }
    }; 


    let parsed_messages = match messages::parse_config(lazy_config) {
        Ok(m) => m ,
        Err(e) => {
            return Err(e);
        },
    };

    notes::render_notes(&parsed_messages); 

    Ok(())
}

