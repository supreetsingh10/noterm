use clap::{Arg, App, SubCommand, ArgMatches};
use libra::backend::commands; 
use libra::backend::config::init_config;
use libra::backend::messages;
use libra::ui::notes; 

fn get_matches() -> ArgMatches<'static> {
     App::new("Sticky term").
        about("Make sticky notes for your terminal").
        subcommand(SubCommand::with_name("pin").
                   about("Pin the message").
                   arg(Arg::with_name("message").
                       short("m").
                       long("msg").
                       takes_value(true)).
                   arg(Arg::with_name("color").
                       short("col").
                       long("color"))).
        subcommand(SubCommand::with_name("show").
                   about("Show the messages").
                   arg(Arg::with_name("all").
                       short("a").
                       long("all").
                       takes_value(false)).
                   arg(Arg::with_name("note").
                       short("n").
                       long("note").
                       takes_value(true))).
        subcommand(SubCommand::with_name("update").
                   about("Update the given note").
                   arg(Arg::with_name("note").
                       short("n").
                       long("note").
                       takes_value(true))).get_matches() 

}

fn main() -> Result<(), String> {
    let matches = get_matches(); 

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

    commands::exec_cmds(par_cmds, parsed_messages, &matches); 

    Ok(())
}

