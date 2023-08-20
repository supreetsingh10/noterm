use clap::{App, Arg, ArgMatches, SubCommand};
use libra::backend::commands;
use libra::backend::config::init_config;
use libra::backend::messages;

fn get_matches() -> ArgMatches<'static> {
    App::new("Sticky term")
        .about("Make sticky notes for your terminal")
        .subcommand(
            SubCommand::with_name("pin")
                .about("Pin the message")
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("msg")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("color")
                        .short("col")
                        .takes_value(true)
                        .long("color"),
                ),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show the messages")
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("note")
                        .short("n")
                        .long("note")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update the given note")
                .arg(
                    Arg::with_name("note")
                        .short("n")
                        .long("note")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("color")
                        .short("c")
                        .long("color")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete the note")
                .arg(
                    Arg::with_name("note")
                        .short("n")
                        .long("note")
                        .takes_value(true),
                ),
        )
        .get_matches()
}

fn main() -> Result<(), String> {
    let matches = get_matches();

    let lazy_config = match init_config() {
        Ok(config) => config,
        Err(e) => {
            return Err(e);
        }
    };

    let mut parsed_messages = messages::parse_config(lazy_config);
    if let Err(e) = commands::parse_args_exec_command(&matches, &mut parsed_messages) {
        println!("Execution of the command failed {}", e); 
        return Err(e); 
    };

    Ok(())
}
