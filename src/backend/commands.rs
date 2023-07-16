use crate::backend::constants::CommandNames; 
use clap::ArgMatches; 

pub fn parsed_commands(clargs: &ArgMatches<'_>) -> Result<Vec<CommandNames>, String> {
    let mut v_parsed: Vec<CommandNames> = vec![]; 
    if clargs.is_present("show") && !v_parsed.contains(&CommandNames::SHOW) {
        v_parsed.push(CommandNames::SHOW);
    } else if clargs.is_present("pin") && !v_parsed.contains(&CommandNames::PIN) {
        v_parsed.push(CommandNames::PIN);
    } else {
        println!("NO COMMAND GIVEN"); 
        return Err(String::from("No command given")); 
    }

    Ok(v_parsed)
}
