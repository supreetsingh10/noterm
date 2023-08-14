use std::str::FromStr;

use crate::{backend::constants::SubCommandNames, ui::notes};
use clap::ArgMatches;

use super::messages::{write_new_config, Message, Messages};

// Check if can just do all of this with messages itself.
// make a specific struct of a something that will hold all the values I will need.
pub fn parse_args_exec_command(
    clargs: &ArgMatches<'_>,
    par_mess: &mut Messages,
) -> Result<(), String> {
    match clargs.subcommand() {
        ("show", Some(_)) => show_note(par_mess).map_err(|e| {
            return e.to_string();
        }),
        ("pin", Some(sub_m)) => pin_note(par_mess, sub_m).map_err(|e| {
            return e.to_string();
        }),
        ("update", Some(sub_m)) => update_note(&par_mess, sub_m).map_err(|e| {
            return e.to_string();
        }),
        _ => Err(String::from("Invalud command executed")),
    }
}

// how to execute all the commands Should we make a functional pointer?

fn show_note(par_mess: &Messages) -> Result<(), String> {
    notes::render(&par_mess)
}

// ovewrite the config in the specific functions.
fn pin_note(par_mess: &mut Messages, sub_m: &ArgMatches<'_>) -> Result<(), String> {
    let v = match sub_m.value_of("message") {
        Some(v) => {
            if v.len() == 0 as usize {
                return Err(String::from("Message has no text"));
            }
            v
        }
        None => "Hello world",
    };

    let c = match sub_m.value_of("color") {
        Some(c) => match tui::style::Color::from_str(c) {
            Ok(col) => col,
            Err(_) => {
                return Err(String::from("Color unavailable"));
            }
        },
        None => tui::style::Color::Green,
    };

    let p_mess = Message::new(par_mess.messages.len() as u32, String::from(v), c);
    par_mess.messages.push(p_mess);

    show_note(&par_mess).map_err(|e| {
        return e.to_string();
    });

    write_new_config(&par_mess);

    Ok(())
}

fn update_note(par_mess: &Messages, sub_m: &ArgMatches<'_>) -> Result<(), String> {
    notes::render(&par_mess)
}
