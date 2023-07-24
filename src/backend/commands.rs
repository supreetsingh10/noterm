use crate::backend::constants::SubCommandNames; 
use clap::ArgMatches; 

pub fn parsed_commands(clargs: &ArgMatches<'_>) -> Result<Vec<SubCommandNames>, String> {
    let mut v_parsed: Vec<SubCommandNames> = vec![]; 
    if clargs.is_present("show") && !v_parsed.contains(&SubCommandNames::SHOW) {
        v_parsed.push(SubCommandNames::SHOW);
    } else if clargs.is_present("pin") && !v_parsed.contains(&SubCommandNames::PIN) {
        v_parsed.push(SubCommandNames::PIN);
    } else if clargs.is_present("update") && !v_parsed.contains(&SubCommandNames::UPDATE) {
        v_parsed.push(SubCommandNames::UPDATE);
    } else {
        println!("NO COMMAND GIVEN"); 
        return Err(String::from("No command given")); 
    }

    Ok(v_parsed)
}


pub fn exec_cmds(cmd: Vec<SubCommandNames>, clargs: &ArgMatches<'_>) {
    for c in cmd.into_iter() {
        match c {
            SubCommandNames::SHOW => {
                println!("{}", clargs.value_of("message").unwrap());
            }, 
            SubCommandNames::PIN => {
                let submatch = clargs.subcommand_matches("pin").unwrap();
                println!("{:?}", submatch.value_of("message").unwrap()); 
            }, 
            SubCommandNames::UPDATE => {
                println!("{}", clargs.value_of("update").unwrap());
            }
        }
    }
}
