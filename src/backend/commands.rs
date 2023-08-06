use crate::{backend::constants::SubCommandNames, ui::notes}; 
use clap::ArgMatches;

use super::messages::Messages;

pub fn parsed_commands(clargs: &ArgMatches<'_>) -> Result<Vec<SubCommandNames>, String> {
    let mut v_parsed: Vec<SubCommandNames> = vec![]; 
    if clargs.is_present("show") && !v_parsed.contains(&SubCommandNames::SHOW) {
        v_parsed.push(SubCommandNames::SHOW);
    } else if clargs.is_present("pin") && !v_parsed.contains(&SubCommandNames::PIN) {
        v_parsed.push(SubCommandNames::PIN);
    } else if clargs.is_present("update") && !v_parsed.contains(&SubCommandNames::UPDATE) {
        v_parsed.push(SubCommandNames::UPDATE);
    } else {
        return Err(String::from("No command given")); 
    }

    Ok(v_parsed)
}

pub fn exec_cmds(cmd: Vec<SubCommandNames>, par_mess: Messages, clargs: &ArgMatches<'_>) {
}

fn show(par_mess: &Messages) -> Result<(), String> {
    notes::render(&par_mess)
}
