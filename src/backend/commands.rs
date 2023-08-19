use super::messages::{write_new_config, Message, Messages};
use crate::ui::notes;
use clap::ArgMatches;
use std::str::FromStr;
use tui::style::Color;

pub fn parse_args_exec_command(
    clargs: &ArgMatches<'_>,
    par_mess: &mut Messages,
) -> Result<(), String> {
    match clargs.subcommand() {
        ("show", Some(_)) => show_note(par_mess).map_err(|e| e.to_string()),
        ("pin", Some(sub_m)) => pin_note(par_mess, sub_m).map_err(|e| e.to_string()),
        ("update", Some(sub_m)) => update_note(par_mess, sub_m).map_err(|e| e.to_string()),
        ("delete", Some(sub_m)) => delete_note(par_mess, sub_m).map_err(|e| e.to_string()),
        _ => Err(String::from("Invalud command executed")),
    }
}

pub trait ParseNoteArgs {
    fn get_note_args_num(&self) -> Result<usize, String>;
    fn get_note_args_message(&self) -> Result<String, String>;
    fn get_note_args_color(&self) -> Result<Color, Color>;
}

impl ParseNoteArgs for ArgMatches<'_> {
    fn get_note_args_num(&self) -> Result<usize, String> {
        let note_num = match self.value_of("note") {
            Some(note) => match note.parse::<usize>() {
                Ok(n) => n,
                Err(e) => {
                    return Err(e.to_string());
                }
            },
            None => {
                return Err(String::from("No value of note given"));
            }
        };

        Ok(note_num)
    }

    fn get_note_args_message(&self) -> Result<String, String> {
        let mess = match self.value_of("message") {
            Some(mess) => {
                if mess.len() == 0 as usize {
                    return Err(String::from("Message has no text body"));
                }
                mess
            }
            None => {
                return Err(String::from("No message value was given"));
            }
        };

        Ok(String::from(mess))
    }

    fn get_note_args_color(&self) -> Result<Color, Color> {
        self.value_of("color")
            .map(|color| {
                tui::style::Color::from_str(color)
                    .map(|color| color)
                    .map_err(|_| tui::style::Color::Green)
            })
            .unwrap()

    }
}

fn show_note(par_mess: &Messages) -> Result<(), String> {
    notes::render(&par_mess)
}

fn pin_note(par_mess: &mut Messages, sub_m: &ArgMatches<'_>) -> Result<(), String> {
    let v = match sub_m.get_note_args_message() {
        Ok(mess_value) => mess_value,
        Err(e) => {
            return Err(e);
        }
    };

    let c = sub_m.get_note_args_color().unwrap();

    let p_mess = Message::new(par_mess.messages.len() as u32, String::from(v), c);
    par_mess.messages.push(p_mess);

    show_note(&par_mess)
        .map_err(|e| {
            return e.to_string();
        })
        .ok();

    write_new_config(&par_mess)
}

fn update_note(par_mess: &mut Messages, sub_m: &ArgMatches<'_>) -> Result<(), String> {
    let num = match sub_m.get_note_args_num() {
        Ok(num) => num,
        Err(e) => {
            return Err(e);
        }
    };

    let note: &mut Message = match par_mess.messages.get_mut(num as usize) {
        Some(note) => note,
        None => {
            return Err(String::from(format!(
                "Invalid number of note give there are only {} notes",
                par_mess.messages.len()
            )));
        }
    };

    match sub_m.get_note_args_message() {
        Ok(mess) => {
            note.update_note_text(mess);
        }
        Err(_) => {}
    };

    sub_m.value_of("color").and_then(|c| {
        tui::style::Color::from_str(c)
            .ok()
            .map(|col| note.update_note_color(col))
    });

    write_new_config(&par_mess)
}

fn delete_note(par_mess: &mut Messages, sub_m: &ArgMatches<'_>) -> Result<(), String> {
    sub_m
        .get_note_args_num()
        .map(|num| {
            (num >= 0 as usize && num < par_mess.messages.len())
                .then(|| {
                    par_mess.messages.remove(num);
                    // Starts the index from 0 irrespective where we start.
                    for (index, mess) in par_mess.messages[num..].iter_mut().enumerate() {
                        mess.update_note_number(num + index);
                    }

                })
                .unwrap()
        })
        .map_err(|e| e.to_string());

    write_new_config(&par_mess)
}
